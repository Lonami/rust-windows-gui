//! (Mostly) the simplest windows program you can write that actually creates a functional window.
//! http://winprog.org/tutorial/simple_window.html
use std::process::exit;
use winapi_app_windows as win;

const CLASS_NAME: &str = "myWindowClass";

fn main() -> win::Result<()> {
    // Step 1: Registering the Window Class
    let class = &match win::class::build()
        .load_icon(win::icon::Icon::Application)?
        .load_cursor(win::cursor::Cursor::Arrow)?
        .background(win::class::Background::Window)
        .load_small_icon(win::icon::Icon::Application)?
        .register(CLASS_NAME)
    {
        Ok(cls) => cls,
        Err(e) => {
            win::messagebox::message_box(
                "Error!",
                &format!("Window Registration Failed! {}", e),
                &[win::messagebox::Config::IconWarning],
            )
            .unwrap();
            exit(0);
        }
    };

    // Step 2: Creating the Window
    let window = match win::window::build()
        .set_message_callback(main_window_callback)
        .add_extended_style(win::window::ExtendedStyle::ClientEdge)
        .add_style(win::window::Style::OverlappedWindow)
        .size(240, 120)
        .create(class, "The title of my window")
    {
        Ok(win) => win,
        Err(e) => {
            win::messagebox::message_box(
                "Error!",
                &format!("Window Creation Failed! {}", e),
                &[win::messagebox::Config::IconWarning],
            )
            .unwrap();
            exit(0);
        }
    };

    window.show_default();
    window.update().unwrap();

    // Step 3: The Message Loop
    exit(win::message_loop())
}

// Step 4: the Window Procedure
fn main_window_callback(
    window: &win::window::Window,
    message: win::message::Message,
) -> Option<isize> {
    use win::message::Message;

    match message {
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
