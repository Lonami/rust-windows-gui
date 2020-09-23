#![cfg(windows)]
pub mod class;
pub mod cursor;
pub mod icon;
pub mod message;
pub mod messagebox;
pub mod window;

use once_cell::sync::Lazy;
use std::ptr;
use std::{collections::HashMap, sync::Mutex};
use winapi::shared::minwindef::HINSTANCE;
use winapi::um::libloaderapi::GetModuleHandleA;
use winapi::um::winuser::{
    DispatchMessageA, GetMessageA, PostQuitMessage, TranslateMessage, LPMSG, MSG,
};

pub use std::io::Error;
pub type Result<T> = std::io::Result<T>;
pub type MessageCallback = fn(&window::Window, message::Message) -> Option<isize>;

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
static HWND_TO_CALLBACK: Lazy<Mutex<HashMap<usize, MessageCallback>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Obtains the `hInstance` parameter from `WinMain`.
pub(crate) fn base_instance() -> HINSTANCE {
    unsafe { GetModuleHandleA(std::ptr::null()) }
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
            DispatchMessageA(&mut msg as LPMSG);
        }
        msg.wParam as i32
    }
}
