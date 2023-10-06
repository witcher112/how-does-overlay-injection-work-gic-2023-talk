use crate::*;

pub fn init() {
    init_debug_console_client();

    unsafe { init_d3d11_hook(); }
}
