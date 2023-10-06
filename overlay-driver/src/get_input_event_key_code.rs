use winapi::um::winuser::*;

pub fn get_input_event_key_code(msg_w_param: usize) -> String {
    return match msg_w_param as i32 {
        VK_BACK => "Backspace".into(),
        VK_TAB => "Tab".into(),
        VK_RETURN => "Enter".into(),
        VK_SHIFT => "Shift".into(),
        VK_CONTROL => "Ctrl".into(),
        VK_MENU => "Alt".into(),
        VK_PAUSE => "Pause".into(),
        VK_CAPITAL => "CapsLock".into(),
        VK_ESCAPE => "Escape".into(),
        VK_SPACE => " ".into(),
        33 => "PageUp".into(),
        34 => "PageDown".into(),
        VK_END => "End".into(),
        VK_HOME => "Home".into(),
        VK_LEFT => "Left".into(),
        VK_UP => "Up".into(),
        VK_RIGHT => "Right".into(),
        VK_DOWN => "Down".into(),
        VK_INSERT => "Insert".into(),
        VK_DELETE => "Delete".into(),
        48 => "0".into(),
        49 => "1".into(),
        50 => "2".into(),
        51 => "3".into(),
        52 => "4".into(),
        53 => "5".into(),
        54 => "6".into(),
        55 => "7".into(),
        56 => "8".into(),
        57 => "9".into(),
        65 => "A".into(),
        66 => "B".into(),
        67 => "C".into(),
        68 => "D".into(),
        69 => "E".into(),
        70 => "F".into(),
        71 => "G".into(),
        72 => "H".into(),
        73 => "I".into(),
        74 => "J".into(),
        75 => "K".into(),
        76 => "L".into(),
        77 => "M".into(),
        78 => "N".into(),
        79 => "O".into(),
        80 => "P".into(),
        81 => "Q".into(),
        82 => "R".into(),
        83 => "S".into(),
        84 => "T".into(),
        85 => "U".into(),
        86 => "V".into(),
        87 => "W".into(),
        88 => "X".into(),
        89 => "Y".into(),
        90 => "Z".into(),
        VK_LWIN => "Meta".into(),
        VK_RWIN => "Meta".into(),
        VK_APPS => "ContextMenu".into(),
        VK_NUMPAD0 => "0".into(),
        VK_NUMPAD1 => "1".into(),
        VK_NUMPAD2 => "2".into(),
        VK_NUMPAD3 => "3".into(),
        VK_NUMPAD4 => "4".into(),
        VK_NUMPAD5 => "5".into(),
        VK_NUMPAD6 => "6".into(),
        VK_NUMPAD7 => "7".into(),
        VK_NUMPAD8 => "8".into(),
        VK_NUMPAD9 => "9".into(),
        VK_MULTIPLY => "*".into(),
        VK_ADD => "+".into(),
        VK_SUBTRACT => "-".into(),
        VK_DECIMAL => ".".into(),
        VK_DIVIDE => "/".into(),
        VK_F1 => "F1".into(),
        VK_F2 => "F2".into(),
        VK_F3 => "F3".into(),
        VK_F4 => "F4".into(),
        VK_F5 => "F5".into(),
        VK_F6 => "F6".into(),
        VK_F7 => "F7".into(),
        VK_F8 => "F8".into(),
        VK_F9 => "F9".into(),
        VK_F10 => "F10".into(),
        VK_F11 => "F11".into(),
        VK_F12 => "F12".into(),
        VK_F13 => "F13".into(),
        VK_F14 => "F14".into(),
        VK_F15 => "F15".into(),
        VK_F16 => "F16".into(),
        VK_F17 => "F17".into(),
        VK_F18 => "F18".into(),
        VK_F19 => "F19".into(),
        VK_F20 => "F20".into(),
        VK_F21 => "F21".into(),
        VK_F22 => "F22".into(),
        VK_F23 => "F23".into(),
        VK_F24 => "F24".into(),
        VK_NUMLOCK => "NumLock".into(),
        VK_SCROLL => "ScrollLock".into(),
        182 => "My Computer".into(),
        183 => "My Calculator".into(),
        VK_OEM_1 => ";".into(),
        VK_OEM_PLUS => "=".into(),
        VK_OEM_COMMA => ",".into(),
        VK_OEM_MINUS => "-".into(),
        VK_OEM_PERIOD => ".".into(),
        VK_OEM_2 => "/".into(),
        VK_OEM_3 => "`".into(),
        VK_OEM_4 => "[".into(),
        VK_OEM_5 => "\\".into(),
        VK_OEM_6 => "]".into(),
        VK_OEM_7 => "'".into(),
        250 => "Play".into(),
        _ => (msg_w_param as u8 as char).to_string(),
    };
}
