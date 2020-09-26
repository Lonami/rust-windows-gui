use crate::{base_instance, non_null_or_err, Result};
use std::path::Path;
use std::ptr::{self, NonNull};
use widestring::U16CString;
use winapi::shared::windef::{HICON, HICON__};
use winapi::um::winnt::LPCWSTR;
use winapi::um::winuser::{
    LoadIconW, LoadImageW, IDI_APPLICATION, IDI_ERROR, IDI_INFORMATION, IDI_QUESTION, IDI_SHIELD,
    IDI_WARNING, IDI_WINLOGO, IMAGE_ICON, LR_LOADFROMFILE, MAKEINTRESOURCEW,
};

pub struct FileData {
    path: U16CString,
}

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
    /// Custom icons stored somewhere in the disk.
    FromFile(FileData),
}

impl Icon {
    /// Creates a new icon from in-disk file.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        Icon::FromFile(FileData {
            path: U16CString::from_os_str(path.as_ref().as_os_str()).unwrap(),
        })
    }

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
            Icon::FromFile(data) => data.path.as_ptr(),
        }
    }

    pub(crate) fn load(&self) -> Result<NonNull<HICON__>> {
        let handle = if matches!(self, Icon::FromResource(_)) {
            base_instance()
        } else {
            ptr::null_mut()
        };

        let result = unsafe { LoadIconW(handle, self.value()) };
        non_null_or_err(result)
    }

    pub(crate) fn load_small(&self) -> Result<NonNull<HICON__>> {
        self.load_size(16)
    }

    pub(crate) fn load_large(&self) -> Result<NonNull<HICON__>> {
        self.load_size(32)
    }

    fn load_size(&self, size: i32) -> Result<NonNull<HICON__>> {
        match self {
            Icon::FromResource(_) => {
                let result =
                    unsafe { LoadImageW(base_instance(), self.value(), IMAGE_ICON, size, size, 0) };

                let result = result as HICON;
                non_null_or_err(result)
            }
            Icon::FromFile(_) => {
                let result = unsafe {
                    LoadImageW(
                        ptr::null_mut(),
                        self.value(),
                        IMAGE_ICON,
                        size,
                        size,
                        LR_LOADFROMFILE,
                    )
                };

                let result = result as HICON;
                non_null_or_err(result)
            }
            _ => self.load(),
        }
    }
}
