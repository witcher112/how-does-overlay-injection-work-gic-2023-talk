const path = require("path");
const { execSync, spawn, spawnSync } = require("child_process");

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
