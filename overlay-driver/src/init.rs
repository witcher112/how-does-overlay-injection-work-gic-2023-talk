use crate::*;

pub fn init() {
    init_debug_console_client();


    init_client();

    unsafe {
        init_d3d11_hook();
    }

    loop {
        let client_pending_message_payload_proto_option =
            read_client_pending_message_payload_proto_option();

        if let Some(client_pending_message_payload_proto) =
            client_pending_message_payload_proto_option
        {
            if client_pending_message_payload_proto.has_set_texture_info_message_payload() {
                let set_texture_info_message_payload_proto =
                    client_pending_message_payload_proto.get_set_texture_info_message_payload();

                let overlay_texture_info = OverlayTextureInfo {
                    width: set_texture_info_message_payload_proto.get_width(),
                    height: set_texture_info_message_payload_proto.get_height(),
                    bytes: set_texture_info_message_payload_proto.get_bytes().to_vec(),
                };

                set_overlay_texture_info(overlay_texture_info);
            } else if client_pending_message_payload_proto.has_set_is_overlay_active_message_payload() {
                let set_is_overlay_active_message_payload_proto =
                    client_pending_message_payload_proto.get_set_is_overlay_active_message_payload();
                
                set_is_overlay_active(set_is_overlay_active_message_payload_proto.is_active);
            }
        }
    }
}
