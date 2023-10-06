const path = require("path");
const net = require("net");
const stream = require("stream");
const { execSync, spawn, spawnSync } = require("child_process");
const electron = require("electron");
const protobufjs = require("protobufjs");

class EncodedMessageDataStream extends stream.Transform {

  constructor() {
    super();
    this._messageDataLength = 0;
    this._messageDataLengthBytesCount = 0;
    this._isReadingMessageDataLength = true;
    this._buffer = Buffer.alloc(0);
  }

  _transform(chunk, _encoding, callback) {

    this._buffer = Buffer.concat([this._buffer, chunk]);

    while (this._buffer.length > 0) {

      if (this._isReadingMessageDataLength) {

        const bufferFirstByte = this._buffer[0];

        this._messageDataLength += (bufferFirstByte & 0x7f) << (7 * this._messageDataLengthBytesCount);

        if ((bufferFirstByte & (1 << 7)) !== 0) {

          this._messageDataLengthBytesCount += 1;
          this._isReadingMessageDataLength = true;

        } else {

          this._messageDataLengthBytesCount = 0;
          this._isReadingMessageDataLength = false;
        }

        this._buffer = this._buffer.slice(1);

      } else {

        if (this._messageDataLength <= this._buffer.length) {

          this.push(this._buffer.slice(0, this._messageDataLength));
          this._buffer = this._buffer.slice(this._messageDataLength);
          this._messageDataLength = 0;
          this._isReadingMessageDataLength = true;

        } else {

          break;
        }
      }
    }

    callback();
  }
}

(async () => {

  execSync(
    `cargo build --release`,
    {
      cwd: path.resolve(__dirname, `..`, `debug-console`),
      stdio: `ignore`,
    },
  );

  execSync(
    `cargo build --release`,
    {
      cwd: path.resolve(__dirname, `..`, `overlay-driver-injector`),
      stdio: `ignore`,
    },
  );

  execSync(
    `cargo build --release`,
    {
      cwd: path.resolve(__dirname, `..`, `overlay-driver`),
      stdio: `ignore`,
    },
  );

  await electron.app.whenReady();

  const gameProcessHandle = spawn(
    path.resolve(`D:/Steam/steamapps/common/Divinity Original Sin Enhanced Edition/Shipping/EoCApp.exe`),
    {
      cwd: path.resolve(`D:/Steam/steamapps/common/Divinity Original Sin Enhanced Edition/Shipping`),
    },
  );

  gameProcessHandle.on(`exit`, () => process.exit(0));

  const debugConsoleProcessHandle = spawn(
    path.resolve(__dirname, `..`, `debug-console`, `target`, `release`, `debug-console.exe`),
    {
      stdio: `pipe`,
    },
  );

  debugConsoleProcessHandle.stdout.pipe(process.stdout);

  const overlayDriverDllFilePath = path.resolve(
    __dirname,
    `..`,
    `overlay-driver`,
    `target`,
    `release`,
    `overlay_driver.dll`,
  );

  spawnSync(
    path.resolve(__dirname, `..`, `overlay-driver-injector`, `target`, `release`, `overlay-driver-injector.exe`),
    [
      overlayDriverDllFilePath,
      gameProcessHandle.pid.toString(),
    ],
  );

  const overlayWindowHandle = new electron.BrowserWindow({
    webPreferences: {
      offscreen: true,
    },
    transparent: true,
    show: false,
  });

  const protoSchema = await new Promise((resolve, reject) => {

    protobufjs.load(
      path.resolve(__dirname, `..`, `overlay-driver`, `src`, `proto.proto`),
      (error, result) => {

        if (error !== null) {

          rejectPromise(error);
          return;
        }

        resolve(result);
      },
    );
  });

  const clientMessagePayloadType = protoSchema.lookupType(`ClientMessagePayload`);
  const serverMessagePayloadType = protoSchema.lookupType(`ServerMessagePayload`);

  const serverHandle = new net.Server();

  const clientHandle = await new Promise((resolve) => {

    serverHandle.on(`connection`, (clientHandle) => {
      resolve(clientHandle);
    });

    serverHandle.listen(64128, `127.0.0.1`);
  });

  clientHandle.on(`error`, () => { });

  const clientEncodedMessageDataStream = new EncodedMessageDataStream();

  clientHandle.on(`data`, (data) => {

    clientEncodedMessageDataStream.write(data);
  });

  clientEncodedMessageDataStream.on(`data`, (data) => {

    const serverMessagePayload = serverMessagePayloadType.decode(data);

    const onSizeChangedMessagePayload = serverMessagePayload.onSizeChangedMessagePayload;

    if (onSizeChangedMessagePayload !== undefined) {

      console.log(onSizeChangedMessagePayload);

      overlayWindowHandle.setSize(
        onSizeChangedMessagePayload.width,
        onSizeChangedMessagePayload.height,
      );
    }
  });

  overlayWindowHandle.webContents.on(
    `paint`,
    (_event, _dirty, image) => {

      const size = image.getSize();

      const clientMessagePayloadProtoAsBytes = clientMessagePayloadType.encodeDelimited(
        clientMessagePayloadType.create({
          setTextureInfoMessagePayload: {
            width: size.width,
            height: size.height,
            bytes: image.getBitmap(),
          },
        }),
      ).finish();

      clientHandle.write(clientMessagePayloadProtoAsBytes);
    },
  );

  overlayWindowHandle.loadURL(path.resolve(__dirname, `index.html`));

})();