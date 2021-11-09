//! Starting the workings of a text editor.
//! http://winprog.org/tutorial/app_one.html
use std::process::exit;
use minimal_windows_gui as win;

const CLASS_NAME: &str = "myWindowClass";

const IDC_MAIN_EDIT: u16 = 101;

fn main() -> win::Result<()> {
    let class = &win::class::build()
        .load_icon(win::icon::Icon::Application)?
        .load_cursor(win::cursor::Cursor::Arrow)?
        .background(win::class::Background::Window)
        .load_small_icon(win::icon::Icon::Application)?
        .register(CLASS_NAME)
        .expect("window registration failed");

    let window = win::window::build()
        .set_message_callback(main_window_callback)
        .add_extended_style(win::window::ExtendedStyle::ClientEdge)
        .add_style(win::window::Style::OverlappedWindow)
        .size(480, 320)
        .create(class, "Tutorial Application")
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
            let edit_ctl = match win::window::build()
                .add_extended_style(win::window::ExtendedStyle::ClientEdge)
                .add_style(win::window::Style::Visible)
                .add_style(win::window::Style::VerticalScroll)
                .add_style(win::window::Style::HorizontalScroll)
                .add_style(win::window::Style::Multiline)
                .add_style(win::window::Style::AutoVerticalScroll)
                .add_style(win::window::Style::AutoHorizontalScroll)
                .pos(0, 0)
                .size(100, 100)
                .parent(window)
                .set_child_id(IDC_MAIN_EDIT)
                .create(win::class::edit_control(), "")
            {
                Ok(c) => c,
                Err(_) => {
                    win::messagebox::message_box(
                        "Error",
                        "Could not create edit box.",
                        &[win::messagebox::Config::IconError],
                    )
                    .unwrap();
                    exit(1);
                }
            };

            let font = win::font::get_default().unwrap();
            edit_ctl.set_font(font);
        }
        Message::Size(info) => {
            let edit_ctl = window.get_dialog_item(IDC_MAIN_EDIT).unwrap();
            edit_ctl
                .set_rect(win::rect::Rect::new(
                    info.width() as i32,
                    info.height() as i32,
                ))
                .unwrap();
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
