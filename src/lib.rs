#![cfg(windows)]
pub mod class;
pub mod cursor;
pub mod dialog;
pub mod font;
pub mod gdi;
pub mod icon;
pub mod menu;
pub mod message;
pub mod messagebox;
pub mod rect;
pub mod toolbar;
pub mod window;

use once_cell::sync::Lazy;
use std::ffi::CString;
use std::ptr::{self, NonNull};
use std::{collections::HashMap, sync::Mutex};
use winapi::shared::minwindef::{BOOL, HINSTANCE, MAX_PATH};
use winapi::shared::ntdef::LPSTR;
use winapi::um::commctrl::InitCommonControls;
use winapi::um::libloaderapi::{GetModuleFileNameA, GetModuleHandleA};
use winapi::um::winuser::{
    DispatchMessageA, GetMessageA, PostQuitMessage, TranslateMessage, LPMSG, MSG,
};

pub use std::io::{Error, Result};
pub type MessageCallback = fn(&window::Window, message::Message) -> Option<isize>;
pub type DialogCallback = fn(&window::Window, message::Message) -> isize;

// We want to wrap user functions to provide them with a safer interface.
//
// The wrappers can't be lambdas because they need to be "extern system" and kept alive for as
// long as the class lives.
//
// We don't know how many functions they will need ahead of time, so we can't define that many
// static functions either.
//
// The only solution is to have a single static wrapper function that queries a global map (this
// map) to determine what to call based on the window.
//
// Because messages may be emitted before the pointer is obtained, a special value of 0 is used
// to indicate "newly created", and is used as a fallback.
static HWND_TO_CALLBACK: Lazy<Mutex<HashMap<usize, MessageCallback>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

static HWND_TO_DLG_CALLBACK: Lazy<Mutex<HashMap<usize, DialogCallback>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Obtains the `hInstance` parameter from `WinMain`.
pub(crate) fn base_instance() -> HINSTANCE {
    unsafe { GetModuleHandleA(std::ptr::null()) }
}

/// Registers and initializes certain common control window classes.
/// This method must be called early in the program if common controls are used.
pub fn init_common_controls() {
    unsafe {
        InitCommonControls();
    }
}

/// Retrieves the fully qualified path for the file that contains the specified module.
/// The module must have been loaded by the current process.
pub fn module_file_name() -> Result<CString> {
    let module = base_instance();
    let mut buffer = vec![0u8; MAX_PATH];

    let result =
        unsafe { GetModuleFileNameA(module, buffer.as_mut_ptr() as LPSTR, buffer.len() as u32) };

    if result == 0 {
        Err(Error::last_os_error())
    } else {
        buffer.truncate(result as usize);
        Ok(CString::new(buffer)?)
    }
}

/// Indicates to the system that a thread has made a request to terminate (quit).
/// It is typically used in response to a `Destroy` message.
///
/// The application exit code is used as the wParam parameter of the `Quit` message.
pub fn post_quit_message(exit_code: i32) {
    unsafe { PostQuitMessage(exit_code) }
}

pub fn message_loop() -> i32 {
    unsafe {
        let mut msg: MSG = std::mem::zeroed();
        while GetMessageA(&mut msg as LPMSG, ptr::null_mut(), 0, 0) > 0 {
            TranslateMessage(&mut msg as LPMSG);
            // This effectively looks up the window corresponding to the message's window handle
            // and calls its window procedure. Alternatively `GetWindowLong` can be used to do
            // the same, but manually (http://winprog.org/tutorial/message_loop.html).
            DispatchMessageA(&mut msg as LPMSG);
        }
        msg.wParam as i32
    }
}

/// Checks the resulting return value of a function. If it's `true`, `Ok` is returned. Otherwise,
/// the last OS error is returned in the `Err` variant.
pub(crate) fn ok_or_last_err(result: BOOL) -> Result<()> {
    if result != 0 {
        Ok(())
    } else {
        Err(Error::last_os_error())
    }
}

pub(crate) fn non_null_or_err<T>(value: *mut T) -> Result<NonNull<T>> {
    NonNull::new(value).ok_or_else(|| Error::last_os_error())
}
