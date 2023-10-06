const DEBUG_CONSOLE_SERVER_PORT: u32 = 64129;

pub fn init_debug_console_client() {
    let debug_console_client_socket =
        std::net::TcpStream::connect(format!("127.0.0.1:{}", DEBUG_CONSOLE_SERVER_PORT));

    if let Ok(debug_console_client_socket) = debug_console_client_socket {
        simplelog::WriteLogger::init(
            simplelog::LevelFilter::Debug,
            simplelog::Config::default(),
            debug_console_client_socket,
        )
        .unwrap();

        log::info!("Debug console client started");
    }

    std::panic::set_hook(Box::new(|panic_info| {
        log::error!("{}", panic_info);
    }));
}
