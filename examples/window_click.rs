//! Add the capability to show the user what the name of our program is when they click on our window.
//! http://winprog.org/tutorial/window_click.html
use minimal_windows_gui as win;
use std::process::exit;

const CLASS_NAME: &str = "myWindowClass";

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
        Message::LeftMouseButtonDown(_info) => {
            let file_name = win::module_file_name().unwrap();
            win::messagebox::message_box(
                "This program is:",
                &file_name.into_string().unwrap(),
                &[win::messagebox::Config::IconInformation],
            )
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
