//! The simplest Win32 program.
//! http://winprog.org/tutorial/start.html
use minimal_windows_gui as win;

// For a "Win32 GUI" project the entry point is the following:
//
//     int WINAPI WinMain(HINSTANCE hInstance, HINSTANCE hPrevInstance, LPSTR lpCmdLine, int nCmdShow)
//
// In Rust we start with `main()` instead, and don't have access to those parameters.
//
// However, the base address of the memory image of the executable `hInstance` can be obtained
// with `GetModuleHandle`, `hPrevInstance` is always 0, `lpCmdLine` can be obtained with either
// `GetCommandLine` or `std::env::args`, and `nCmdShow` be obtained from `GetStartupInfo`, so
// if those are needed they can be obtained.
fn main() {
    win::messagebox::message_box("Note", "Goodbye, cruel world!", &[]).unwrap();
}
