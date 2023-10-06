use std::sync::Mutex;

pub struct OverlayTextureInfo {
    pub width: u32,
    pub height: u32,
    pub bytes: Vec<u8>,
}

pub struct OverlayState {
    pub texture_info: Option<OverlayTextureInfo>,
    pub is_active: bool,
}

unsafe impl Send for OverlayState {}

lazy_static::lazy_static! {
    pub static ref OVERLAY_STATE: Mutex<OverlayState> = Mutex::new(
        OverlayState {
            texture_info: None,
            is_active: false,
        },
    );
}

pub fn set_overlay_texture_info(overlay_texture_info: OverlayTextureInfo) {
    let mut overlay_state_guard = OVERLAY_STATE.lock().unwrap();

    let overlay_state = &mut *overlay_state_guard;

    overlay_state.texture_info = Some(overlay_texture_info);
}

pub fn set_is_overlay_active(is_overlay_active: bool) {
    let mut overlay_state_guard = OVERLAY_STATE.lock().unwrap();

    let overlay_state = &mut *overlay_state_guard;

    overlay_state.is_active = is_overlay_active;
}

