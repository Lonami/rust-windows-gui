//! Shows how to add basic menus to your window. Usually pre-made menu resources are used.
//! These will be in an `.rc` file, which will be compiled and linked into the `.exe`.
//! http://winprog.org/tutorial/menus.html
use std::process::exit;
use winapi_app_windows as win;

const CLASS_NAME: &str = "myWindowClass";

const IDR_MYMENU: u16 = 101;
const IDI_MYICON: u16 = 102;

const ID_FILE_EXIT: u16 = 9001;
const ID_STUFF_GO: u16 = 9002;

fn main() -> win::Result<()> {
    let class = &win::class::build()
        .load_icon(win::icon::Icon::FromResource(IDI_MYICON))?
        .load_cursor(win::cursor::Cursor::Arrow)?
        .background(win::class::Background::Window)
        .menu(IDR_MYMENU)
        .load_small_icon(win::icon::Icon::FromResource(IDI_MYICON))?
        .register(CLASS_NAME)
        .expect("window registration failed");

    let window = win::window::build()
        .set_message_callback(main_window_callback)
        .add_extended_style(win::window::ExtendedStyle::ClientEdge)
        .add_style(win::window::Style::OverlappedWindow)
        .size(240, 120)
        .create(class, "A Menu")
        .expect("window creation failed");

    window.show_default();
    window.update().unwrap();

    exit(win::message_loop())
}

fn main_window_callback(
    window: &win::window::Window,
    message: win::message::Message,
) -> Option<isize> {
    use win::message::Message;

    match message {
        Message::Command(info) => {
            if let Some(menu_id) = info.menu_id() {
                match menu_id {
                    ID_FILE_EXIT => {
                        window.close().unwrap();
                    }
                    ID_STUFF_GO => {
                        win::messagebox::message_box("Woo!", "You clicked Go!", &[]).unwrap();
                    }
                    _ => {}
                }
            }
        }
        Message::Close => {
            window.destroy().unwrap();
        }
        Message::Destroy => {
            win::post_quit_message(0);
        }
        _ => return None,
    }

    Some(0)
}
