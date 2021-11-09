//! Bitmaps, Device Contexts and BitBlt.
//! http://winprog.org/tutorial/bitmaps.html
use std::cell::Cell;
use std::process::exit;
use minimal_windows_gui as win;

const CLASS_NAME: &str = "myWindowClass";

const IDB_BALL: u16 = 101;

thread_local! {
    static BALL: Cell<Option<win::bitmap::Bitmap>> = Cell::new(None);
}

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
        .create(class, "A Bitmap Program")
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
            let ball = win::bitmap::load(IDB_BALL).expect("could not load IDB_BALL");
            BALL.with(|cell| cell.set(Some(ball)));
        }
        Message::Paint => {
            let paint = window.paint().unwrap();

            BALL.with(|cell| {
                let ball = cell.take().unwrap();
                let info = ball.info().unwrap();
                paint
                    .copy_bitmap_to_rect(0, 0, info.width(), info.height(), &ball, 0, 0)
                    .unwrap();

                cell.set(Some(ball));
            });
        }
        Message::Close => {
            window.destroy().unwrap();
        }
        Message::Destroy => {
            BALL.with(|cell| drop(cell.take().take()));
            win::post_quit_message(0);
        }
        _ => return None,
    }

    Some(0)
}
