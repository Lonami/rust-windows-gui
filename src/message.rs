use winapi::shared::minwindef::{LPARAM, UINT, WPARAM};
use winapi::um::winuser::{
    MK_CONTROL, MK_LBUTTON, MK_MBUTTON, MK_RBUTTON, MK_SHIFT, MK_XBUTTON1, MK_XBUTTON2, WM_CLOSE,
    WM_DESTROY, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MBUTTONDOWN, WM_MBUTTONUP, WM_RBUTTONDOWN,
    WM_RBUTTONUP,
};

#[derive(Debug)]
pub struct MouseData {
    wparam: WPARAM,
    lparam: LPARAM,
}

#[derive(Debug)]
pub enum Message {
    Destroy,
    Close,
    LeftMouseButtonDown(MouseData),
    RightMouseButtonDown(MouseData),
    MiddleMouseButtonDown(MouseData),
    LeftMouseButtonUp(MouseData),
    RightMouseButtonUp(MouseData),
    MiddleMouseButtonUp(MouseData),
    Other {
        msg: UINT,
        wparam: WPARAM,
        lparam: LPARAM,
    },
}

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

impl Message {
    pub(crate) fn from_raw(msg: UINT, wparam: WPARAM, lparam: LPARAM) -> Self {
        match msg {
            WM_DESTROY => Message::Destroy,
            WM_CLOSE => Message::Close,
            WM_LBUTTONDOWN => Message::LeftMouseButtonDown(MouseData { wparam, lparam }),
            WM_RBUTTONDOWN => Message::RightMouseButtonDown(MouseData { wparam, lparam }),
            WM_MBUTTONDOWN => Message::MiddleMouseButtonDown(MouseData { wparam, lparam }),
            WM_LBUTTONUP => Message::LeftMouseButtonUp(MouseData { wparam, lparam }),
            WM_RBUTTONUP => Message::RightMouseButtonUp(MouseData { wparam, lparam }),
            WM_MBUTTONUP => Message::MiddleMouseButtonUp(MouseData { wparam, lparam }),
            _ => Message::Other {
                msg,
                wparam,
                lparam,
            },
        }
    }
}
