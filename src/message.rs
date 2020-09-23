use winapi::shared::minwindef::{LPARAM, UINT, WPARAM};
use winapi::um::winuser::{WM_CLOSE, WM_DESTROY};

#[derive(Debug)]
pub enum Message {
    Destroy,
    Close,
    Other {
        msg: UINT,
        wparam: WPARAM,
        lparam: LPARAM,
    },
}

impl Message {
    pub(crate) fn from_raw(msg: UINT, wparam: WPARAM, lparam: LPARAM) -> Self {
        match msg {
            WM_DESTROY => Message::Destroy,
            WM_CLOSE => Message::Close,
            _ => Message::Other {
                msg,
                wparam,
                lparam,
            },
        }
    }
}
