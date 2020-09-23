use crate::{Error, Result};
use std::ptr::{self, NonNull};
use winapi::shared::windef::HICON__;
use winapi::um::winnt::LPCWSTR;
use winapi::um::winuser::{
    LoadIconW, IDI_APPLICATION, IDI_ERROR, IDI_INFORMATION, IDI_QUESTION, IDI_SHIELD, IDI_WARNING,
    IDI_WINLOGO,
};

/// Built-in icons as defined in https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadicona.
pub enum Icon {
    /// Default application icon.
    Application,
    /// Hand-shaped icon.
    Error,
    /// Asterisk icon.
    Information,
    /// Question mark icon.
    Question,
    /// Security Shield icon.
    Shield,
    /// Exclamation point icon.
    Warning,
    /// Default application icon.
    WinLogo,
}

impl Icon {
    // IDI* are defined as pointers which we can't use as values in the enum.
    fn value(&self) -> LPCWSTR {
        match self {
            Icon::Application => IDI_APPLICATION,
            Icon::Error => IDI_ERROR,
            Icon::Information => IDI_INFORMATION,
            Icon::Question => IDI_QUESTION,
            Icon::Shield => IDI_SHIELD,
            Icon::Warning => IDI_WARNING,
            Icon::WinLogo => IDI_WINLOGO,
        }
    }

    pub(crate) fn load(&self) -> Result<NonNull<HICON__>> {
        let result = unsafe { LoadIconW(ptr::null_mut(), self.value()) };

        if let Some(icon) = NonNull::new(result) {
            Ok(icon)
        } else {
            Err(Error::last_os_error())
        }
    }
}
