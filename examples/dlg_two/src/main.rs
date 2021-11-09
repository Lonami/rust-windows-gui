//! Modeless dialogs depend on the main message loop to pump the messages as it does for the
//! main window.
//! http://winprog.org/tutorial/modeless_dialogs.html
use std::cell::Cell;
use std::process::exit;
use minimal_windows_gui as win;

const CLASS_NAME: &str = "myWindowClass";

const IDR_MYMENU: u16 = 101;
const IDD_TOOLBAR: u16 = 101;

const IDC_PRESS: u16 = 1000;
const IDC_OTHER: u16 = 1001;

const ID_FILE_EXIT: u16 = 40001;
const ID_DIALOG_SHOW: u16 = 40002;
const ID_DIALOG_HIDE: u16 = 40003;

thread_local! {
    static TOOLBAR_HANDLE: Cell<Option<win::window::Window<'static>>> = Cell::new(None);
}

fn main() -> win::Result<()> {
    let class = &win::class::build()
        .load_icon(win::icon::Icon::Application)?
        .load_cursor(win::cursor::Cursor::Arrow)?
        .background(win::class::Background::Window)
        .menu(IDR_MYMENU)
        .load_small_icon(win::icon::Icon::Application)?
        .register(CLASS_NAME)
        .expect("window registration failed");

    let window = win::window::build()
        .set_message_callback(main_window_callback)
        .add_extended_style(win::window::ExtendedStyle::ClientEdge)
        .add_style(win::window::Style::OverlappedWindow)
        .size(240, 120)
        .create(class, "The title of my window")
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
        Message::Create => {
            match window.create_dialog(IDD_TOOLBAR, tool_dialog_callback) {
                Ok(dialog) => {
                    dialog.show();
                    TOOLBAR_HANDLE.with(|cell| cell.set(Some(dialog)));
                }
                Err(_) => {
                    win::messagebox::message_box(
                        "Warning!",
                        "CreateDialog returned NULL",
                        &[win::messagebox::Config::IconInformation],
                    )
                    .unwrap();
                }
            };
        }
        Message::Command(info) => {
            if let Some(menu_id) = info.menu_id() {
                match menu_id {
                    ID_FILE_EXIT => {
                        window.close().unwrap();
                    }
                    ID_DIALOG_SHOW => {
                        TOOLBAR_HANDLE.with(|cell| {
                            let handle = cell.take();
                            handle.as_ref().unwrap().show();
                            cell.set(handle);
                        });
                    }
                    ID_DIALOG_HIDE => {
                        TOOLBAR_HANDLE.with(|cell| {
                            let handle = cell.take();
                            handle.as_ref().unwrap().hide();
                            cell.set(handle);
                        });
                    }
                    _ => {}
                }
            }
        }
        Message::Close => {
            TOOLBAR_HANDLE.with(|cell| {
                let handle = cell.take();
                handle.unwrap().destroy().unwrap();
            });
            window.destroy().unwrap();
        }
        Message::Destroy => {
            win::post_quit_message(0);
        }
        _ => return None,
    }

    Some(0)
}

fn tool_dialog_callback(_dialog: &win::window::Window, message: win::message::Message) -> isize {
    use win::message::Message;

    match message {
        Message::InitDialog => {}
        Message::Command(info) => match info.control_data().map(|c| c.id) {
            Some(IDC_PRESS) => {
                win::messagebox::message_box(
                    "Hi!",
                    "This is a message",
                    &[win::messagebox::Config::IconWarning],
                )
                .unwrap();
            }
            Some(IDC_OTHER) => {
                win::messagebox::message_box(
                    "Bye!",
                    "This is also a message",
                    &[win::messagebox::Config::IconWarning],
                )
                .unwrap();
            }
            _ => {}
        },
        _ => return 0,
    }

    1
}
