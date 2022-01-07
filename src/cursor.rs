use crate::{non_null_or_err, Result};
use std::ptr::{self, NonNull};
use winapi::shared::windef::HICON__;
use winapi::um::winnt::LPCWSTR;
use winapi::um::winuser::{
    LoadCursorW, IDC_APPSTARTING, IDC_ARROW, IDC_CROSS, IDC_HAND,
    IDC_HELP, IDC_IBEAM, IDC_ICON, IDC_NO, IDC_SIZE, IDC_SIZEALL, IDC_SIZENESW, IDC_SIZENS,
    IDC_SIZENWSE, IDC_SIZEWE, IDC_UPARROW, IDC_WAIT,
};

/// Built-in cursors as defined in https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw.
pub enum Cursor {
    /// Standard arrow and small hourglass.
    AppStarting,
    /// Standard arrow.
    Arrow,
    /// Crosshair.
    Cross,
    /// Hand.
    Hand,
    /// Arrow and question mark.
    Help,
    /// I-beam.
    Ibeam,
    /// Obsolete for applications marked version 4.0 or later.
    Icon,
    /// Slashed circle.
    No,
    /// Obsolete for applications marked version 4.0 or later. Use `SizeAll`.
    Size,
    /// Four-pointed arrow pointing north, south, east, and west.
    SizeAll,
    /// Double-pointed arrow pointing northeast and southwest.
    SizeNesw,
    /// Double-pointed arrow pointing north and south.
    SizeNs,
    /// Double-pointed arrow pointing northwest and southeast.
    SizeNwse,
    /// Double-pointed arrow pointing west and east.
    SizeWe,
    /// Vertical arrow.
    UpArrow,
    /// Hourglass .
    Wait,
}

impl Cursor {
    // IDC* are defined as pointers which we can't use as values in the enum.
    fn value(&self) -> LPCWSTR {
        match self {
            Cursor::AppStarting => IDC_APPSTARTING,
            Cursor::Arrow => IDC_ARROW,
            Cursor::Cross => IDC_CROSS,
            Cursor::Hand => IDC_HAND,
            Cursor::Help => IDC_HELP,
            Cursor::Ibeam => IDC_IBEAM,
            Cursor::Icon => IDC_ICON,
            Cursor::No => IDC_NO,
            Cursor::Size => IDC_SIZE,
            Cursor::SizeAll => IDC_SIZEALL,
            Cursor::SizeNesw => IDC_SIZENESW,
            Cursor::SizeNs => IDC_SIZENS,
            Cursor::SizeNwse => IDC_SIZENWSE,
            Cursor::SizeWe => IDC_SIZEWE,
            Cursor::UpArrow => IDC_UPARROW,
            Cursor::Wait => IDC_WAIT,
        }
    }

    pub(crate) fn load(&self) -> Result<NonNull<HICON__>> {
        let result = unsafe {
            // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw
            LoadCursorW(ptr::null_mut(), self.value())
        };

        non_null_or_err(result)
    }
}
