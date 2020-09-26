use winapi::shared::minwindef::{HIWORD, LOWORD, LPARAM, UINT, WPARAM};
use winapi::um::winuser::{
    MK_CONTROL, MK_LBUTTON, MK_MBUTTON, MK_RBUTTON, MK_SHIFT, MK_XBUTTON1, MK_XBUTTON2, WM_CLOSE,
    WM_COMMAND, WM_CREATE, WM_DESTROY, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MBUTTONDOWN, WM_MBUTTONUP,
    WM_RBUTTONDOWN, WM_RBUTTONUP,
};

#[derive(Debug)]
pub struct MouseData {
    wparam: WPARAM,
    lparam: LPARAM,
}

#[derive(Debug)]
pub struct CommandData {
    wparam: WPARAM,
    lparam: LPARAM,
}

#[derive(Debug)]
pub enum Message {
    Create,
    Destroy,
    Close,
    LeftMouseButtonDown(MouseData),
    RightMouseButtonDown(MouseData),
    MiddleMouseButtonDown(MouseData),
    LeftMouseButtonUp(MouseData),
    RightMouseButtonUp(MouseData),
    MiddleMouseButtonUp(MouseData),
    Command(CommandData),
    Other {
        msg: UINT,
        wparam: WPARAM,
        lparam: LPARAM,
    },
}

// https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-lbuttondown
impl MouseData {
    /// The x-coordinate of the cursor. The coordinate is relative to the upper-left corner of the client area.
    pub fn x(&self) -> i32 {
        (self.lparam & 0xffff) as i32
    }

    /// The y-coordinate of the cursor. The coordinate is relative to the upper-left corner of the client area.
    pub fn y(&self) -> i32 {
        ((self.lparam >> 16) & 0xffff) as i32
    }

    /// Whether the CTRL key is down.
    pub fn control(&self) -> bool {
        (self.wparam & MK_CONTROL) != 0
    }

    /// Whether the left mouse button is down.
    pub fn lmb(&self) -> bool {
        (self.wparam & MK_LBUTTON) != 0
    }

    /// Whether the middle mouse button is down.
    pub fn mmb(&self) -> bool {
        (self.wparam & MK_MBUTTON) != 0
    }

    /// Whether the right mouse button is down.
    pub fn rmb(&self) -> bool {
        (self.wparam & MK_RBUTTON) != 0
    }

    /// Whether the SHIFT key is down.
    pub fn shift(&self) -> bool {
        (self.wparam & MK_SHIFT) != 0
    }

    /// Whether the first X button is down.
    pub fn xbutton1(&self) -> bool {
        (self.wparam & MK_XBUTTON1) != 0
    }

    /// Whether the second X button is down.
    pub fn xbutton2(&self) -> bool {
        (self.wparam & MK_XBUTTON2) != 0
    }
}

// https://docs.microsoft.com/en-us/windows/win32/menurc/wm-command
impl CommandData {
    /// The selected menu identifier if the message source is a menu.
    pub fn menu_id(&self) -> Option<u16> {
        if self.lparam == 0 && HIWORD(self.wparam as u32) == 0 {
            Some(LOWORD(self.wparam as u32))
        } else {
            None
        }
    }

    /// The selected accelerator identifier if the message source is an accelerator.
    pub fn accelerator_id(&self) -> Option<u16> {
        if self.lparam == 0 && HIWORD(self.wparam as u32) == 1 {
            Some(LOWORD(self.wparam as u32))
        } else {
            None
        }
    }
}

impl Message {
    pub(crate) fn from_raw(msg: UINT, wparam: WPARAM, lparam: LPARAM) -> Self {
        match msg {
            WM_CREATE => Message::Create,
            WM_DESTROY => Message::Destroy,
            WM_CLOSE => Message::Close,
            WM_LBUTTONDOWN => Message::LeftMouseButtonDown(MouseData { wparam, lparam }),
            WM_RBUTTONDOWN => Message::RightMouseButtonDown(MouseData { wparam, lparam }),
            WM_MBUTTONDOWN => Message::MiddleMouseButtonDown(MouseData { wparam, lparam }),
            WM_LBUTTONUP => Message::LeftMouseButtonUp(MouseData { wparam, lparam }),
            WM_RBUTTONUP => Message::RightMouseButtonUp(MouseData { wparam, lparam }),
            WM_MBUTTONUP => Message::MiddleMouseButtonUp(MouseData { wparam, lparam }),
            WM_COMMAND => Message::Command(CommandData { wparam, lparam }),
            _ => Message::Other {
                msg,
                wparam,
                lparam,
            },
        }
    }
}
