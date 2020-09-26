//! Dialogs are just windows, which can be defined as dialog resources.
//! http://winprog.org/tutorial/dialogs.html
use std::process::exit;
use winapi_app_windows as win;

const CLASS_NAME: &str = "myWindowClass";

const IDR_MYMENU: u16 = 101;
const IDD_ABOUT: u16 = 102;

const ID_FILE_EXIT: u16 = 40001;
const ID_HELP_ABOUT: u16 = 40002;

const DLG_OK: isize = 1;
const DLG_CANCEL: isize = 2;

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
        Message::Command(info) => {
            if let Some(menu_id) = info.menu_id() {
                match menu_id {
                    ID_FILE_EXIT => {
                        window.close().unwrap();
                    }
                    ID_HELP_ABOUT => {
                        let msg = match window.show_dialog(IDD_ABOUT, about_dialog_callback) {
                            Ok(DLG_OK) => "Dialog exited with IDOK.",
                            Ok(DLG_CANCEL) => "Dialog exited with IDCANCEL.",
                            Ok(_) | Err(_) => "Dialog failed!",
                        };

                        win::messagebox::message_box(
                            "Notice",
                            msg,
                            &[win::messagebox::Config::IconInformation],
                        )
                        .unwrap();
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

fn about_dialog_callback(dialog: &win::window::Window, message: win::message::Message) -> bool {
    use win::message::Message;
    use win::messagebox::Button;

    match message {
        Message::InitDialog => {}
        Message::Command(info) => match info.control_button() {
            Some(Button::Ok) => dialog.end_dialog(DLG_OK).unwrap(),
            Some(Button::Cancel) => dialog.end_dialog(DLG_CANCEL).unwrap(),
            _ => {}
        },
        _ => return false,
    }

    true
}
