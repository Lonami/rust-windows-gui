use crate::{messagebox, window};
use std::ptr::NonNull;
use winapi::shared::minwindef::{HIWORD, LOWORD, LPARAM, UINT, WPARAM};
use winapi::shared::windef::{HDC, HWND};
use winapi::um::wingdi::{
    GetBValue, GetGValue, GetRValue, SetBkMode, SetTextColor, CLR_INVALID, OPAQUE, RGB, TRANSPARENT,
};
use winapi::um::winuser::{
    LBN_SELCHANGE, MK_CONTROL, MK_LBUTTON, MK_MBUTTON, MK_RBUTTON, MK_SHIFT, MK_XBUTTON1,
    MK_XBUTTON2, SIZE_MAXHIDE, SIZE_MAXIMIZED, SIZE_MAXSHOW, SIZE_MINIMIZED, SIZE_RESTORED,
    WM_CLOSE, WM_COMMAND, WM_CREATE, WM_CTLCOLORDLG, WM_CTLCOLORSTATIC, WM_DESTROY, WM_INITDIALOG,
    WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MBUTTONDOWN, WM_MBUTTONUP, WM_PAINT, WM_RBUTTONDOWN,
    WM_RBUTTONUP, WM_SIZE, WM_TIMER,
};

#[derive(Debug)]
pub struct SizeData {
    wparam: WPARAM,
    lparam: LPARAM,
}

#[derive(Debug)]
pub struct TimerData {
    wparam: WPARAM,
    lparam: LPARAM,
}

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

pub struct ControlData<'a> {
    /// Control-defined notification code
    pub code: u16,
    /// Control identifier
    pub id: u16,
    /// Handle to the control window
    pub window: window::Window<'a>,
}

#[derive(Debug)]
pub struct ColorData {
    wparam: WPARAM,
    lparam: LPARAM,
}

#[derive(Debug)]
pub enum Message {
    Create,
    Size(SizeData),
    Destroy,
    Close,
    InitDialog,
    Paint,
    Timer(TimerData),
    LeftMouseButtonDown(MouseData),
    RightMouseButtonDown(MouseData),
    MiddleMouseButtonDown(MouseData),
    LeftMouseButtonUp(MouseData),
    RightMouseButtonUp(MouseData),
    MiddleMouseButtonUp(MouseData),
    Command(CommandData),
    ControlColorDialog(ColorData),
    ControlColorStatic(ColorData),
    Other {
        msg: UINT,
        wparam: WPARAM,
        lparam: LPARAM,
    },
}

#[derive(Debug)]
pub enum ListBoxMessage {
    SelectionChange,
    Other { code: u16 },
}

// https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-size
impl SizeData {
    /// `true` if the message was sent to all pop-up windows when some other window is maximized.
    pub fn maximize_hide(&self) -> bool {
        self.wparam == SIZE_MAXHIDE
    }

    /// `true` if the window has been maximized.
    pub fn maximized(&self) -> bool {
        self.wparam == SIZE_MAXIMIZED
    }

    /// `true` if the message was sent to all pop-up windows when some other window has been
    /// restored to its former size.
    pub fn maximize_show(&self) -> bool {
        self.wparam == SIZE_MAXSHOW
    }

    /// `true` if the window has been minimized.
    pub fn minimized(&self) -> bool {
        self.wparam == SIZE_MINIMIZED
    }

    /// `true` if the window has been resized, but neither the `minimized` nor `maximized` value applies.
    pub fn restored(&self) -> bool {
        self.wparam == SIZE_RESTORED
    }

    /// The new width of the client area.
    pub fn width(&self) -> u16 {
        LOWORD(self.lparam as u32)
    }

    /// The new height of the client area.
    pub fn height(&self) -> u16 {
        HIWORD(self.lparam as u32)
    }
}

// https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-timer
impl TimerData {
    pub fn timer_id(&self) -> usize {
        self.wparam
    }
}

// https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-lbuttondown
impl MouseData {
    /// The x-coordinate of the cursor. The coordinate is relative to the upper-left corner of the client area.
    pub fn x(&self) -> u16 {
        LOWORD(self.lparam as u32)
    }

    /// The y-coordinate of the cursor. The coordinate is relative to the upper-left corner of the client area.
    pub fn y(&self) -> u16 {
        HIWORD(self.lparam as u32)
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

    /// The raw data emitted by a control.
    pub fn control_data(&self) -> Option<ControlData> {
        if let Some(hwnd) = NonNull::new(self.lparam as HWND) {
            Some(ControlData {
                code: HIWORD(self.wparam as u32),
                id: LOWORD(self.wparam as u32),
                window: window::Window::Borrowed { hwnd },
            })
        } else {
            None
        }
    }
}

impl ControlData<'_> {
    /// Which standard button is responsible for this message, or `None` if it was emitted by
    /// some other custom control.
    pub fn std_button(&self) -> Option<messagebox::Button> {
        match messagebox::Button::from_id(self.id as i32) {
            Ok(Some(button)) => Some(button),
            _ => None,
        }
    }

    /// Interpret the `code` as if it was a notification emitted by a list box.
    pub fn list_box_code(&self) -> ListBoxMessage {
        ListBoxMessage::from_raw(self.code)
    }
}

// https://docs.microsoft.com/en-us/windows/win32/dlgbox/wm-ctlcolordlg
// https://docs.microsoft.com/en-us/windows/win32/controls/wm-ctlcolorstatic
impl ColorData {
    fn hdc(&self) -> HDC {
        self.wparam as HDC
    }

    pub fn set_text_color(&self, r: u8, g: u8, b: u8) -> std::result::Result<(u8, u8, u8), ()> {
        // https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-settextcolor
        let result = unsafe { SetTextColor(self.hdc(), RGB(r, g, b)) };
        if result != CLR_INVALID {
            Ok((GetRValue(result), GetGValue(result), GetBValue(result)))
        } else {
            Err(())
        }
    }

    pub fn set_background_transparency(&self, transparent: bool) -> std::result::Result<bool, ()> {
        // https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setbkmode
        let value = if transparent { TRANSPARENT } else { OPAQUE };

        let result = unsafe { SetBkMode(self.hdc(), value as i32) };
        match result as u32 {
            TRANSPARENT => Ok(true),
            OPAQUE => Ok(false),
            0 => Err(()),
            _ => panic!("invalid return value from SetBkMode"),
        }
    }
}

impl Message {
    pub(crate) fn from_raw(msg: UINT, wparam: WPARAM, lparam: LPARAM) -> Self {
        match msg {
            WM_CREATE => Message::Create,
            WM_SIZE => Message::Size(SizeData { wparam, lparam }),
            WM_DESTROY => Message::Destroy,
            WM_CLOSE => Message::Close,
            WM_INITDIALOG => Message::InitDialog,
            WM_PAINT => Message::Paint,
            WM_TIMER => Message::Timer(TimerData { wparam, lparam }),
            WM_LBUTTONDOWN => Message::LeftMouseButtonDown(MouseData { wparam, lparam }),
            WM_RBUTTONDOWN => Message::RightMouseButtonDown(MouseData { wparam, lparam }),
            WM_MBUTTONDOWN => Message::MiddleMouseButtonDown(MouseData { wparam, lparam }),
            WM_LBUTTONUP => Message::LeftMouseButtonUp(MouseData { wparam, lparam }),
            WM_RBUTTONUP => Message::RightMouseButtonUp(MouseData { wparam, lparam }),
            WM_MBUTTONUP => Message::MiddleMouseButtonUp(MouseData { wparam, lparam }),
            WM_COMMAND => Message::Command(CommandData { wparam, lparam }),
            WM_CTLCOLORDLG => Message::ControlColorDialog(ColorData { wparam, lparam }),
            WM_CTLCOLORSTATIC => Message::ControlColorStatic(ColorData { wparam, lparam }),
            _ => Message::Other {
                msg,
                wparam,
                lparam,
            },
        }
    }
}

impl ListBoxMessage {
    pub(crate) fn from_raw(code: u16) -> Self {
        match code {
            LBN_SELCHANGE => ListBoxMessage::SelectionChange,
            _ => ListBoxMessage::Other { code },
        }
    }
}
