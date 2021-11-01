//! Transparent Bitmaps.
//! http://winprog.org/tutorial/transparency.html
use std::cell::Cell;
use std::process::exit;
use winapi_app_windows as win;

const CLASS_NAME: &str = "myWindowClass";

thread_local! {
    static BALL_MASK: Cell<Option<(win::bitmap::Bitmap, win::bitmap::Bitmap)>> = Cell::new(None);
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
            let ball =
                win::bitmap::from_file(r"examples\bmp_one\ball.bmp").expect("could not load ball");

            let mask = create_mask(&ball, (0, 0, 0)).expect("could not create mask");

            BALL_MASK.with(|cell| cell.set(Some((ball, mask))));
        }
        Message::Paint => {
            let paint = window.paint().unwrap();

            BALL_MASK.with(|cell| {
                let (ball, mask) = cell.take().unwrap();

                let rect = window.get_rect().unwrap();
                paint
                    .fill_rect(rect, win::brush::light_gray().unwrap())
                    .unwrap();

                let info = ball.info().unwrap();
                let (w, h) = (info.width(), info.height());
                let rect = win::rect::Rect::new(w, h);

                paint
                    .copy_bitmap_to_rect(rect.clone(), &mask, 0, 0)
                    .unwrap();
                paint
                    .and_bitmap_to_rect(rect.at(w, 0), &mask, 0, 0)
                    .unwrap();
                paint
                    .and_bitmap_to_rect(rect.at(w * 2, h * 2), &mask, 0, 0)
                    .unwrap();

                paint
                    .copy_bitmap_to_rect(rect.at(0, h), &ball, 0, 0)
                    .unwrap();
                paint
                    .paint_bitmap_to_rect(rect.at(w, h), &ball, 0, 0)
                    .unwrap();
                paint
                    .paint_bitmap_to_rect(rect.at(w * 2, h * 2), &ball, 0, 0)
                    .unwrap();

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

fn create_mask(
    color: &win::bitmap::Bitmap,
    transparent: (u8, u8, u8),
) -> Result<win::bitmap::Bitmap, ()> {
    let info = color.info()?;
    let mask = win::bitmap::new(info.width(), info.height(), 1, 1)?;
    mask.into_mask(color, (info.width(), info.height()), transparent)?;
    Ok(mask)
}
