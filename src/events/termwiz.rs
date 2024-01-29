use termwiz::input::KeyCode as TermwizKeyCode;

use super::KeyPressedEvent;

impl From<TermwizKeyCode> for KeyPressedEvent {
    fn from(key_code: TermwizKeyCode) -> Self {
        use TermwizKeyCode::*;
        match key_code {
            Char(_) => todo!(),
            Hyper => todo!(),
            Super => todo!(),
            Meta => todo!(),
            Cancel => todo!(),
            Backspace => todo!(),
            Tab => todo!(),
            Clear => todo!(),
            Enter => todo!(),
            Shift => todo!(),
            Escape => todo!(),
            LeftShift => todo!(),
            RightShift => todo!(),
            Control => todo!(),
            LeftControl => todo!(),
            RightControl => todo!(),
            Alt => todo!(),
            LeftAlt => todo!(),
            RightAlt => todo!(),
            Menu => todo!(),
            LeftMenu => todo!(),
            RightMenu => todo!(),
            Pause => todo!(),
            CapsLock => todo!(),
            PageUp => todo!(),
            PageDown => todo!(),
            End => todo!(),
            Home => todo!(),
            LeftArrow => todo!(),
            RightArrow => todo!(),
            UpArrow => todo!(),
            DownArrow => todo!(),
            Select => todo!(),
            Print => todo!(),
            Execute => todo!(),
            PrintScreen => todo!(),
            Insert => todo!(),
            Delete => todo!(),
            Help => todo!(),
            LeftWindows => todo!(),
            RightWindows => todo!(),
            Applications => todo!(),
            Sleep => todo!(),
            Numpad0 => todo!(),
            Numpad1 => todo!(),
            Numpad2 => todo!(),
            Numpad3 => todo!(),
            Numpad4 => todo!(),
            Numpad5 => todo!(),
            Numpad6 => todo!(),
            Numpad7 => todo!(),
            Numpad8 => todo!(),
            Numpad9 => todo!(),
            Multiply => todo!(),
            Add => todo!(),
            Separator => todo!(),
            Subtract => todo!(),
            Decimal => todo!(),
            Divide => todo!(),
            Function(_) => todo!(),
            NumLock => todo!(),
            ScrollLock => todo!(),
            Copy => todo!(),
            Cut => todo!(),
            Paste => todo!(),
            BrowserBack => todo!(),
            BrowserForward => todo!(),
            BrowserRefresh => todo!(),
            BrowserStop => todo!(),
            BrowserSearch => todo!(),
            BrowserFavorites => todo!(),
            BrowserHome => todo!(),
            VolumeMute => todo!(),
            VolumeDown => todo!(),
            VolumeUp => todo!(),
            MediaNextTrack => todo!(),
            MediaPrevTrack => todo!(),
            MediaStop => todo!(),
            MediaPlayPause => todo!(),
            ApplicationLeftArrow => todo!(),
            ApplicationRightArrow => todo!(),
            ApplicationUpArrow => todo!(),
            ApplicationDownArrow => todo!(),
            KeyPadHome => todo!(),
            KeyPadEnd => todo!(),
            KeyPadPageUp => todo!(),
            KeyPadPageDown => todo!(),
            KeyPadBegin => todo!(),
            _ => todo!(),
        }
    }
}