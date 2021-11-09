//! Transparent Bitmaps.
//! http://winprog.org/tutorial/transparency.html
use std::cell::Cell;
use std::process::exit;
use minimal_windows_gui as win;

const CLASS_NAME: &str = "myWindowClass";

thread_local! {
    static BALL_MASK: Cell<Option<(win::gdi::Bitmap, win::gdi::Bitmap)>> = Cell::new(None);
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
        .size(240, 160)
        .create(class, "Another Bitmap Program")
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
            // In contrast with the original tutorial, load from file to avoid needing `.rc` files.
            let ball_bmp = win::gdi::bitmap::from_file(r"examples\bmp_one\ball.bmp")
                .expect("could not load ball");
            let mask_bmp = ball_bmp.set_color_transparent((0, 0, 0)).unwrap();
            BALL_MASK.with(|cell| cell.set(Some((ball_bmp, mask_bmp))));
        }
        Message::Paint => {
            let paint = window.paint().unwrap();

            BALL_MASK.with(|cell| {
                let (ball, mask) = cell.take().unwrap();

                let rect = window.get_rect().unwrap();
                paint
                    .fill_rect(rect, win::gdi::brush::light_gray().unwrap())
                    .unwrap();

                let info = ball.info().unwrap();
                let (w, h) = (info.width(), info.height());
                let rect = win::rect::Rect::new(w, h);

                {
                    let canvas = paint.try_clone().unwrap().bind(&mask).unwrap();
                    paint.bitwise().region(rect.at(0, 0)).set(&canvas).unwrap();
                    paint.bitwise().region(rect.at(w, 0)).and(&canvas).unwrap();
                    paint
                        .bitwise()
                        .region(rect.at(w * 2, h * 2))
                        .and(&canvas)
                        .unwrap();

                    let canvas = canvas.bind(&ball).unwrap();
                    paint.bitwise().region(rect.at(0, h)).set(&canvas).unwrap();
                    paint.bitwise().region(rect.at(w, h)).or(&canvas).unwrap();
                    paint
                        .bitwise()
                        .region(rect.at(w * 2, h * 2))
                        .or(&canvas)
                        .unwrap();
                }

                cell.set(Some((ball, mask)));
            });
        }
        Message::Close => {
            window.destroy().unwrap();
        }
        Message::Destroy => {
            BALL_MASK.with(|cell| drop(cell.take().take()));
            win::post_quit_message(0);
        }
        _ => return None,
    }

    Some(0)
}
