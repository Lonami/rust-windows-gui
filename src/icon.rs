use crate::{base_instance, Error, Result};
use std::ptr::{self, NonNull};
use winapi::shared::windef::{HICON, HICON__};
use winapi::um::winnt::LPCWSTR;
use winapi::um::winuser::{
    LoadIconW, LoadImageW, IDI_APPLICATION, IDI_ERROR, IDI_INFORMATION, IDI_QUESTION, IDI_SHIELD,
    IDI_WARNING, IDI_WINLOGO, IMAGE_ICON, MAKEINTRESOURCEW,
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
    /// Custom icons defined in the resource file `.rc`.
    FromResource(u16),
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
            Icon::FromResource(value) => MAKEINTRESOURCEW(*value),
        }
    }

    pub(crate) fn load(&self) -> Result<NonNull<HICON__>> {
        let handle = if matches!(self, Icon::FromResource(_)) {
            base_instance()
        } else {
            ptr::null_mut()
        };

        let result = unsafe { LoadIconW(handle, self.value()) };

        if let Some(icon) = NonNull::new(result) {
            Ok(icon)
        } else {
            Err(Error::last_os_error())
        }
    }

    pub(crate) fn load_small(&self) -> Result<NonNull<HICON__>> {
        if matches!(self, Icon::FromResource(_)) {
            let result =
                unsafe { LoadImageW(base_instance(), self.value(), IMAGE_ICON, 16, 16, 0) };

            let result = result as HICON;
            if let Some(icon) = NonNull::new(result) {
                Ok(icon)
            } else {
                Err(Error::last_os_error())
            }
        } else {
            self.load()
        }
    }
}
