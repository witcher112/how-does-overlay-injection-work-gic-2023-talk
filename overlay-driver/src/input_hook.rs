use std::mem::zeroed;
use std::ptr::null_mut;
use std::{ffi::CString, sync::Mutex};
use widestring::WideCString;
use winapi::shared::minwindef::HIWORD;
use winapi::shared::minwindef::LOWORD;
use winapi::shared::minwindef::LPARAM;
use winapi::shared::minwindef::LRESULT;
use winapi::shared::minwindef::WPARAM;
use winapi::shared::windef::RECT;
use winapi::um::winuser::{GetCursorInfo, GetWindowThreadProcessId, SetWindowsHookExW, WH_GETMESSAGE};
use winapi::um::winuser::TranslateMessage;
use winapi::um::winuser::CURSORINFO;
use winapi::um::winuser::GET_KEYSTATE_WPARAM;
use winapi::um::winuser::GET_WHEEL_DELTA_WPARAM;
use winapi::um::winuser::KF_EXTENDED;
use winapi::um::winuser::MK_CONTROL;
use winapi::um::winuser::MK_LBUTTON;
use winapi::um::winuser::MK_MBUTTON;
use winapi::um::winuser::MK_RBUTTON;
use winapi::um::winuser::MK_SHIFT;
use winapi::um::winuser::MSG;
use winapi::um::winuser::PM_REMOVE;
use winapi::um::winuser::VK_ADD;
use winapi::um::winuser::VK_CAPITAL;
use winapi::um::winuser::VK_CLEAR;
use winapi::um::winuser::VK_CONTROL;
use winapi::um::winuser::VK_DECIMAL;
use winapi::um::winuser::VK_DELETE;
use winapi::um::winuser::VK_DIVIDE;
use winapi::um::winuser::VK_DOWN;
use winapi::um::winuser::VK_END;
use winapi::um::winuser::VK_HOME;
use winapi::um::winuser::VK_INSERT;
use winapi::um::winuser::VK_LCONTROL;
use winapi::um::winuser::VK_LEFT;
use winapi::um::winuser::VK_LMENU;
use winapi::um::winuser::VK_LSHIFT;
use winapi::um::winuser::VK_LWIN;
use winapi::um::winuser::VK_MENU;
use winapi::um::winuser::VK_MULTIPLY;
use winapi::um::winuser::VK_NEXT;
use winapi::um::winuser::VK_NUMLOCK;
use winapi::um::winuser::VK_NUMPAD0;
use winapi::um::winuser::VK_NUMPAD1;
use winapi::um::winuser::VK_NUMPAD2;
use winapi::um::winuser::VK_NUMPAD3;
use winapi::um::winuser::VK_NUMPAD4;
use winapi::um::winuser::VK_NUMPAD5;
use winapi::um::winuser::VK_NUMPAD6;
use winapi::um::winuser::VK_NUMPAD7;
use winapi::um::winuser::VK_NUMPAD8;
use winapi::um::winuser::VK_NUMPAD9;
use winapi::um::winuser::VK_PRIOR;
use winapi::um::winuser::VK_RCONTROL;
use winapi::um::winuser::VK_RETURN;
use winapi::um::winuser::VK_RIGHT;
use winapi::um::winuser::VK_RMENU;
use winapi::um::winuser::VK_RSHIFT;
use winapi::um::winuser::VK_RWIN;
use winapi::um::winuser::VK_SHIFT;
use winapi::um::winuser::VK_SUBTRACT;
use winapi::um::winuser::VK_UP;
use winapi::um::winuser::WM_CHAR;
use winapi::um::winuser::WM_KEYDOWN;
use winapi::um::winuser::WM_KEYFIRST;
use winapi::um::winuser::WM_KEYLAST;
use winapi::um::winuser::WM_KEYUP;
use winapi::um::winuser::WM_LBUTTONDBLCLK;
use winapi::um::winuser::WM_LBUTTONDOWN;
use winapi::um::winuser::WM_LBUTTONUP;
use winapi::um::winuser::WM_MBUTTONDBLCLK;
use winapi::um::winuser::WM_MBUTTONDOWN;
use winapi::um::winuser::WM_MBUTTONUP;
use winapi::um::winuser::WM_MOUSEFIRST;
use winapi::um::winuser::WM_MOUSELAST;
use winapi::um::winuser::WM_MOUSEMOVE;
use winapi::um::winuser::WM_MOUSEWHEEL;
use winapi::um::winuser::WM_NULL;
use winapi::um::winuser::WM_RBUTTONDBLCLK;
use winapi::um::winuser::WM_RBUTTONDOWN;
use winapi::um::winuser::WM_RBUTTONUP;
use winapi::um::winuser::WM_SYSKEYDOWN;
use winapi::um::winuser::WM_SYSKEYUP;
use winapi::um::winuser::WM_XBUTTONDBLCLK;
use winapi::um::winuser::WM_XBUTTONDOWN;
use winapi::um::winuser::WM_XBUTTONUP;
use winapi::um::winuser::{CallNextHookEx, SetCursor};
use winapi::um::winuser::{GetAsyncKeyState, LoadCursorA, IDC_ARROW};
use winapi::um::winuser::{GetClientRect, ShowCursor};
use winapi::{
    ctypes::c_int,
    shared::{minwindef::BOOL, windef::HWND},
    um::libloaderapi::{FreeLibrary, GetProcAddress, LoadLibraryW},
};

use crate::*;

pub type ClipCursor = extern "system" fn(*const RECT) -> BOOL;
pub type SetCursorPos = unsafe fn(c_int, c_int) -> BOOL;

pub struct InputHookState {
    pub hwnd: HWND,
    pub clip_cursor_hook: detour::RawDetour,
    pub set_cursor_pos_hook: detour::RawDetour,
}

unsafe impl Send for InputHookState {}

lazy_static::lazy_static! {
    pub static ref INPUT_HOOK_STATE: Mutex<Option<InputHookState>> = Mutex::new(None);
}

pub unsafe fn init_input_hook(hwnd: HWND) {
    let user_32_library_name = WideCString::from_str("User32.DLL").unwrap();
    let user_32_library_handle = LoadLibraryW(user_32_library_name.as_ptr());
    if user_32_library_handle.is_null() {
        panic!();
    }
    let _free_user_32_library_guard = scopeguard::guard((), |_| {
        FreeLibrary(user_32_library_handle);
    });

    let clip_cursor_name = CString::new("ClipCursor").unwrap();
    let clip_cursor: ClipCursor = std::mem::transmute(GetProcAddress(
        user_32_library_handle,
        clip_cursor_name.as_ptr(),
    ));

    let set_cursor_pos_name = CString::new("SetCursorPos").unwrap();
    let set_cursor_pos: SetCursorPos = std::mem::transmute(GetProcAddress(
        user_32_library_handle,
        set_cursor_pos_name.as_ptr(),
    ));

    let clip_cursor_hook = detour::RawDetour::new(
        clip_cursor as *const (),
        clip_cursor_hook_detour as *const (),
    )
    .unwrap();

    let set_cursor_pos_hook = detour::RawDetour::new(
        set_cursor_pos as *const (),
        set_cursor_pos_hook_detour as *const (),
    )
    .unwrap();

    let mut input_hook_state_guard = INPUT_HOOK_STATE.lock().unwrap();

    *input_hook_state_guard = Some(InputHookState {
        hwnd,
        clip_cursor_hook,
        set_cursor_pos_hook,
    });

    let input_hook_state = &mut *input_hook_state_guard;

    let input_hook_state = input_hook_state.as_mut().unwrap();

    input_hook_state.clip_cursor_hook.enable().unwrap();
    input_hook_state.set_cursor_pos_hook.enable().unwrap();

    let thread_id = GetWindowThreadProcessId(hwnd, null_mut());

    SetWindowsHookExW(
        WH_GETMESSAGE,
        Some(get_message_hook_proc),
        null_mut(),
        thread_id,
    );
}

pub unsafe extern "system" fn clip_cursor_hook_detour(rect: *const RECT) -> BOOL {
    let input_hook_state_guard = INPUT_HOOK_STATE.lock().unwrap();

    let input_hook_state = &*input_hook_state_guard;

    let input_hook_state = input_hook_state.as_ref().unwrap();

    let clip_cursor_hook_trampoline: ClipCursor =
        std::mem::transmute(input_hook_state.clip_cursor_hook.trampoline());

    return clip_cursor_hook_trampoline(rect);
}

pub unsafe extern "system" fn set_cursor_pos_hook_detour(x: c_int, y: c_int) -> BOOL {
    let input_hook_state_guard = INPUT_HOOK_STATE.lock().unwrap();

    let input_hook_state = &*input_hook_state_guard;

    let input_hook_state = input_hook_state.as_ref().unwrap();

    let set_cursor_pos_hook_trampoline: SetCursorPos =
        std::mem::transmute(input_hook_state.set_cursor_pos_hook.trampoline());

    return set_cursor_pos_hook_trampoline(x, y);
}

pub unsafe extern "system" fn get_message_hook_proc(
    code: i32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    if code >= 0 {
        let msg_ptr = l_param as *mut MSG;

        let msg = msg_ptr.as_mut().unwrap();

        let input_hook_state_guard = INPUT_HOOK_STATE.lock().unwrap();

        let input_hook_state = &*input_hook_state_guard;

        let input_hook_state = input_hook_state.as_ref().unwrap();

        {
            let clip_cursor_hook_trampoline: ClipCursor =
                std::mem::transmute(input_hook_state.clip_cursor_hook.trampoline());

            clip_cursor_hook_trampoline(null_mut());

            SetCursor(LoadCursorA(null_mut(), IDC_ARROW as *const _));

            let mut cursor_info: CURSORINFO = zeroed();

            GetCursorInfo(&mut cursor_info);

            if cursor_info.flags == 0 {
                ShowCursor(1);
            }
        }

        if w_param == PM_REMOVE as WPARAM {
            if msg.message >= WM_KEYFIRST && msg.message <= WM_KEYLAST {
                let mut input_event_electron_info_proto_option = None;

                if msg.message == WM_KEYDOWN || msg.message == WM_SYSKEYDOWN {
                    let mut input_event_electron_info_proto =
                        proto::OnInputEventReceivedMessagePayload::new();

                    input_event_electron_info_proto.set_field_type("keyDown".into());

                    input_event_electron_info_proto
                        .set_key_code(get_input_event_key_code(msg.wParam));

                    input_event_electron_info_proto_option = Some(input_event_electron_info_proto);
                } else if msg.message == WM_KEYUP || msg.message == WM_SYSKEYUP {
                    let mut input_event_electron_info_proto =
                        proto::OnInputEventReceivedMessagePayload::new();

                    input_event_electron_info_proto.set_field_type("keyUp".into());

                    input_event_electron_info_proto
                        .set_key_code(get_input_event_key_code(msg.wParam));

                    input_event_electron_info_proto_option = Some(input_event_electron_info_proto);
                } else if msg.message == WM_CHAR {
                    let mut input_event_electron_info_proto =
                        proto::OnInputEventReceivedMessagePayload::new();

                    input_event_electron_info_proto.set_field_type("char".into());

                    input_event_electron_info_proto.set_key_code(
                        widestring::U16String::from_vec(vec![msg.wParam as u16]).to_string_lossy(),
                    );

                    input_event_electron_info_proto_option = Some(input_event_electron_info_proto);
                }

                if let Some(input_event_electron_info_proto) =
                    input_event_electron_info_proto_option.as_mut()
                {
                    if GetAsyncKeyState(VK_SHIFT) as i64 & 0x8000 != 0 {
                        input_event_electron_info_proto
                            .modifiers
                            .push("shift".into());
                    }

                    if GetAsyncKeyState(VK_CONTROL) as i64 & 0x8000 != 0 {
                        input_event_electron_info_proto
                            .modifiers
                            .push("control".into());
                    }

                    if GetAsyncKeyState(VK_MENU) as i64 & 0x8000 != 0 {
                        input_event_electron_info_proto.modifiers.push("alt".into());
                    }

                    if GetAsyncKeyState(VK_LWIN) as i64 & 0x8000 != 0
                        || GetAsyncKeyState(VK_RWIN) as i64 & 0x8000 != 0
                    {
                        input_event_electron_info_proto
                            .modifiers
                            .push("meta".into());
                    }

                    if GetAsyncKeyState(VK_NUMLOCK) as i64 & 0x0001 != 0 {
                        input_event_electron_info_proto
                            .modifiers
                            .push("numLock".into());
                    }

                    if GetAsyncKeyState(VK_CAPITAL) as i64 & 0x0001 != 0 {
                        input_event_electron_info_proto
                            .modifiers
                            .push("capsLock".into());
                    }

                    match msg.wParam as i32 {
                        VK_RETURN => {
                            if (msg.lParam >> 16) & KF_EXTENDED as isize != 0 {
                                input_event_electron_info_proto
                                    .modifiers
                                    .push("isKeypad".into());
                            }
                        }
                        VK_INSERT | VK_DELETE | VK_HOME | VK_END | VK_PRIOR | VK_NEXT | VK_UP
                        | VK_DOWN | VK_LEFT | VK_RIGHT => {
                            if (msg.lParam >> 16) & KF_EXTENDED as isize == 0 {
                                input_event_electron_info_proto
                                    .modifiers
                                    .push("isKeypad".into());
                            }
                        }
                        VK_NUMLOCK | VK_NUMPAD0 | VK_NUMPAD1 | VK_NUMPAD2 | VK_NUMPAD3
                        | VK_NUMPAD4 | VK_NUMPAD5 | VK_NUMPAD6 | VK_NUMPAD7 | VK_NUMPAD8
                        | VK_NUMPAD9 | VK_DIVIDE | VK_MULTIPLY | VK_SUBTRACT | VK_ADD
                        | VK_DECIMAL | VK_CLEAR => {
                            input_event_electron_info_proto
                                .modifiers
                                .push("isKeypad".into());
                        }
                        VK_SHIFT => {
                            if GetAsyncKeyState(VK_LSHIFT) as i64 & 0x8000 != 0 {
                                input_event_electron_info_proto
                                    .modifiers
                                    .push("left".into());
                            }
                            if GetAsyncKeyState(VK_RSHIFT) as i64 & 0x8000 != 0 {
                                input_event_electron_info_proto
                                    .modifiers
                                    .push("right".into());
                            }
                        }
                        VK_CONTROL => {
                            if GetAsyncKeyState(VK_LCONTROL) as i64 & 0x8000 != 0 {
                                input_event_electron_info_proto
                                    .modifiers
                                    .push("left".into());
                            }
                            if GetAsyncKeyState(VK_RCONTROL) as i64 & 0x8000 != 0 {
                                input_event_electron_info_proto
                                    .modifiers
                                    .push("right".into());
                            }
                        }
                        VK_MENU => {
                            if GetAsyncKeyState(VK_LMENU) as i64 & 0x8000 != 0 {
                                input_event_electron_info_proto
                                    .modifiers
                                    .push("left".into());
                            }
                            if GetAsyncKeyState(VK_RMENU) as i64 & 0x8000 != 0 {
                                input_event_electron_info_proto
                                    .modifiers
                                    .push("right".into());
                            }
                        }
                        VK_LWIN => {
                            input_event_electron_info_proto
                                .modifiers
                                .push("left".into());
                        }
                        VK_RWIN => {
                            input_event_electron_info_proto
                                .modifiers
                                .push("right".into());
                        }
                        _ => {}
                    }

                    let mut server_message_payload_proto = proto::ServerMessagePayload::new();

                    server_message_payload_proto.set_on_input_event_received_message_payload(
                        input_event_electron_info_proto.clone(),
                    );

                    send_server_message(server_message_payload_proto);
                }

                {
                    TranslateMessage(msg);
                    msg.message = WM_NULL;
                    return 0;
                }
            } else if msg.message >= WM_MOUSEFIRST && msg.message <= WM_MOUSELAST {
                let mut client_rect: RECT = zeroed();

                if GetClientRect(input_hook_state.hwnd, &mut client_rect) == 1 {
                    let client_width = client_rect.right - client_rect.left;
                    let client_height = client_rect.bottom - client_rect.top;

                    let x = LOWORD(msg.lParam as u32);
                    let y = HIWORD(msg.lParam as u32);

                    if x as i32 <= client_width && y as i32 <= client_height {
                        let mut input_event_electron_info_proto =
                            proto::OnInputEventReceivedMessagePayload::new();

                        if msg.message == WM_LBUTTONDOWN
                            || msg.message == WM_RBUTTONDOWN
                            || msg.message == WM_MBUTTONDOWN
                            || msg.message == WM_XBUTTONDOWN
                            || msg.message == WM_LBUTTONDBLCLK
                            || msg.message == WM_RBUTTONDBLCLK
                            || msg.message == WM_MBUTTONDBLCLK
                            || msg.message == WM_XBUTTONDBLCLK
                        {
                            input_event_electron_info_proto.set_field_type("mouseDown".into());

                            if msg.message == WM_LBUTTONDBLCLK
                                || msg.message == WM_RBUTTONDBLCLK
                                || msg.message == WM_MBUTTONDBLCLK
                                || msg.message == WM_XBUTTONDBLCLK
                            {
                                input_event_electron_info_proto.set_click_count(2);
                            } else {
                                input_event_electron_info_proto.set_click_count(1);
                            }

                            input_event_electron_info_proto
                                .set_key_code(get_input_event_key_code(msg.wParam));
                        } else if msg.message == WM_LBUTTONUP
                            || msg.message == WM_RBUTTONUP
                            || msg.message == WM_MBUTTONUP
                            || msg.message == WM_XBUTTONUP
                        {
                            input_event_electron_info_proto.set_field_type("mouseUp".into());

                            input_event_electron_info_proto.set_click_count(1);
                        } else if msg.message == WM_MOUSEMOVE {
                            input_event_electron_info_proto.set_field_type("mouseMove".into());
                        } else if msg.message == WM_MOUSEWHEEL {
                            input_event_electron_info_proto.set_field_type("mouseWheel".into());

                            input_event_electron_info_proto
                                .set_delta_y(GET_WHEEL_DELTA_WPARAM(msg.wParam).into());

                            input_event_electron_info_proto.set_can_scroll(true);
                        }

                        input_event_electron_info_proto.set_x(x.into());
                        input_event_electron_info_proto.set_y(y.into());

                        if msg.message == WM_LBUTTONDOWN
                            || msg.message == WM_LBUTTONUP
                            || msg.message == WM_LBUTTONDBLCLK
                        {
                            input_event_electron_info_proto.set_button("left".into());
                        } else if msg.message == WM_RBUTTONDOWN
                            || msg.message == WM_RBUTTONUP
                            || msg.message == WM_RBUTTONDBLCLK
                        {
                            input_event_electron_info_proto.set_button("right".into());
                        } else if msg.message == WM_MBUTTONDOWN
                            || msg.message == WM_MBUTTONUP
                            || msg.message == WM_MBUTTONDBLCLK
                        {
                            input_event_electron_info_proto.set_button("middle".into());
                        }

                        let vk_state = GET_KEYSTATE_WPARAM(msg.wParam);

                        if vk_state & MK_CONTROL as u16 != 0 {
                            input_event_electron_info_proto
                                .modifiers
                                .push("control".into());

                            if GetAsyncKeyState(VK_LCONTROL) as i64 & 0x8000 != 0 {
                                input_event_electron_info_proto
                                    .modifiers
                                    .push("left".into());
                            }

                            if GetAsyncKeyState(VK_RCONTROL) as i64 & 0x8000 != 0 {
                                input_event_electron_info_proto
                                    .modifiers
                                    .push("right".into());
                            }
                        }

                        if vk_state & MK_SHIFT as u16 != 0 {
                            input_event_electron_info_proto
                                .modifiers
                                .push("shift".into());

                            if GetAsyncKeyState(VK_LSHIFT) as i64 & 0x8000 != 0 {
                                input_event_electron_info_proto
                                    .modifiers
                                    .push("left".into());
                            }

                            if GetAsyncKeyState(VK_RSHIFT) as i64 & 0x8000 != 0 {
                                input_event_electron_info_proto
                                    .modifiers
                                    .push("right".into());
                            }
                        }

                        if vk_state & VK_MENU as u16 != 0 {
                            input_event_electron_info_proto.modifiers.push("alt".into());

                            if GetAsyncKeyState(VK_LMENU) as i64 & 0x8000 != 0 {
                                input_event_electron_info_proto
                                    .modifiers
                                    .push("left".into());
                            }

                            if GetAsyncKeyState(VK_RMENU) as i64 & 0x8000 != 0 {
                                input_event_electron_info_proto
                                    .modifiers
                                    .push("right".into());
                            }
                        }

                        if vk_state & MK_LBUTTON as u16 != 0 {
                            input_event_electron_info_proto
                                .modifiers
                                .push("leftButtonDown".into());
                        }

                        if vk_state & MK_RBUTTON as u16 != 0 {
                            input_event_electron_info_proto
                                .modifiers
                                .push("rightButtonDown".into());
                        }

                        if vk_state & MK_MBUTTON as u16 != 0 {
                            input_event_electron_info_proto
                                .modifiers
                                .push("middleButtonDown".into());
                        }

                        if GetAsyncKeyState(VK_LWIN) as i64 & 0x8000 != 0
                            || GetAsyncKeyState(VK_RWIN) as i64 & 0x8000 != 0
                        {
                            input_event_electron_info_proto
                                .modifiers
                                .push("meta".into());
                        }

                        if GetAsyncKeyState(VK_NUMLOCK) as i64 & 0x0001 != 0 {
                            input_event_electron_info_proto
                                .modifiers
                                .push("numLock".into());
                        }

                        if GetAsyncKeyState(VK_CAPITAL) as i64 & 0x0001 != 0 {
                            input_event_electron_info_proto
                                .modifiers
                                .push("capsLock".into());
                        }

                        let mut server_message_payload_proto = proto::ServerMessagePayload::new();

                        server_message_payload_proto.set_on_input_event_received_message_payload(
                            input_event_electron_info_proto.clone(),
                        );

                        send_server_message(server_message_payload_proto);

                        {
                            TranslateMessage(msg);
                            msg.message = WM_NULL;
                            return 0;
                        }
                    }
                }
            }
        }
    }

    return CallNextHookEx(null_mut(), code, w_param, l_param);
}
