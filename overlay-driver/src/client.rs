use protobuf::Message;
use std::{collections::VecDeque, sync::Mutex};

use crate::*;

pub struct ClientState {
    pub socket_handle: std::net::TcpStream,
    pub pending_messages_payload_proto: VecDeque<proto::ClientMessagePayload>,
}

lazy_static::lazy_static! {
  pub static ref CLIENT_STATE_OPTION_MUTEX: Mutex<Option<ClientState>> =
      Mutex::new(None);
}

pub fn init_client() {
    let mut client_state_option_guard = CLIENT_STATE_OPTION_MUTEX.lock().unwrap();

    let client_state_option = &mut *client_state_option_guard;

    if client_state_option.is_some() {
        panic!();
    }

    let server_port = 64128;

    log::info!("Connecting to Server on port {}...", server_port,);

    let client_socket = loop {
        match std::net::TcpStream::connect(format!("127.0.0.1:{}", server_port)) {
            Ok(s) => {
                break s;
            }
            _ => {
                log::info!(
                    "Failed to connect to Server on port {}. Retrying in 100ms...",
                    server_port,
                );
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    };

    log::info!("Connected to Server");

    *client_state_option = Some(ClientState {
        socket_handle: client_socket,
        pending_messages_payload_proto: VecDeque::new(),
    });

    std::thread::spawn(update_client_thread);
}

pub fn read_client_pending_message_payload_proto_option() -> Option<proto::ClientMessagePayload> {
    let mut client_state_option_guard = CLIENT_STATE_OPTION_MUTEX.lock().unwrap();

    let client_state_option = &mut *client_state_option_guard;

    if client_state_option.is_none() {
        panic!();
    }

    let client_state = client_state_option.as_mut().unwrap();

    return client_state.pending_messages_payload_proto.pop_front();
}

pub fn send_server_message(server_message_payload_proto: proto::ServerMessagePayload) {
    let mut client_state_option_guard = CLIENT_STATE_OPTION_MUTEX.lock().unwrap();

    let client_state_option = &mut *client_state_option_guard;

    if client_state_option.is_none() {
        panic!();
    }

    let client_state = client_state_option.as_mut().unwrap();

    return server_message_payload_proto
        .write_length_delimited_to_writer(&mut client_state.socket_handle)
        .unwrap();
}

pub fn update_client_thread() {
    let client_socket_handle;

    {
        let mut client_state_option_guard = CLIENT_STATE_OPTION_MUTEX.lock().unwrap();

        let client_state_option = &mut *client_state_option_guard;

        if client_state_option.is_none() {
            panic!();
        }

        let client_state = client_state_option.as_mut().unwrap();

        client_socket_handle = client_state.socket_handle.try_clone().unwrap();
    }

    let mut client_socket_buf_reader_handle = std::io::BufReader::new(client_socket_handle);

    let mut client_coded_input_stream =
        protobuf::CodedInputStream::from_buffered_reader(&mut client_socket_buf_reader_handle);

    loop {
        let client_pending_message_payload_proto_option = client_coded_input_stream
            .read_message::<proto::ClientMessagePayload>()
            .ok();

        if let Some(client_pending_message_payload_proto) =
            client_pending_message_payload_proto_option
        {
            let mut client_state_option_guard = CLIENT_STATE_OPTION_MUTEX.lock().unwrap();

            let client_state_option = &mut *client_state_option_guard;

            if client_state_option.is_none() {
                panic!();
            }

            let client_state = client_state_option.as_mut().unwrap();

            client_state
                .pending_messages_payload_proto
                .push_back(client_pending_message_payload_proto);
        }
    }
}
