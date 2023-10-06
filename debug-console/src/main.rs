use std::io::prelude::BufRead;

const DEBUG_CONSOLE_SERVER_PORT: u32 = 64129;

fn handle_debug_console_client(debug_console_client: std::net::TcpStream) {
    println!("# New client: {:?}", debug_console_client);

    let mut debug_console_client_messages_stream_reader_handle =
        std::io::BufReader::new(debug_console_client);

    loop {
        let mut debug_console_client_message = String::new();

        match debug_console_client_messages_stream_reader_handle
            .read_line(&mut debug_console_client_message)
        {
            Ok(_) => print!("{}", debug_console_client_message),
            Err(e) => {
                eprintln!("{}", e);
                break;
            }
        }
    }

    println!("# Client terminated");
}

fn main() {
    let debug_console_server_handle =
        std::net::TcpListener::bind(format!("127.0.0.1:{}", DEBUG_CONSOLE_SERVER_PORT)).unwrap();

    println!("###########################");
    println!("# Debug Console #");
    println!("###########################");
    println!();
    println!("# Server started on port: {}", DEBUG_CONSOLE_SERVER_PORT);

    for debug_console_client_handle in debug_console_server_handle.incoming() {
        match debug_console_client_handle {
            Ok(stream) => handle_debug_console_client(stream),
            Err(e) => eprintln!("{}", e),
        }
    }
}
