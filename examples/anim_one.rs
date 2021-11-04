//! Timers and Animation.
//! http://winprog.org/tutorial/animation.html
use std::cell::Cell;
use std::num::NonZeroUsize;
use std::process::exit;
use std::time::Duration;
use winapi_app_windows as win;

const CLASS_NAME: &str = "myWindowClass";
const ID_TIMER: usize = 1;

const BALL_MOVE_DELTA: i32 = 2;

struct Global {
    width: i32,
    height: i32,
    x: i32,
    y: i32,

    dx: i32,
    dy: i32,

    ball: win::gdi::Bitmap,
    mask: win::gdi::Bitmap,
}

thread_local! {
    static GLOBAL: Cell<Option<Global>> = Cell::new(None);
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
        .size(320, 240)
        .create(class, "An Animation Program")
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
            let ball = win::gdi::bitmap::from_file(r"examples\bmp_one\ball.bmp")
                .expect("could not load ball");
            let info = ball.info().unwrap();
            let mask = ball.set_color_transparent((0, 0, 0)).unwrap();

            window
                .set_timer(
                    NonZeroUsize::new(ID_TIMER).unwrap(),
                    Duration::from_millis(50),
                )
                .unwrap();

            GLOBAL.with(|cell| {
                cell.set(Some(Global {
                    ball,
                    width: info.width(),
                    height: info.height(),
                    x: 0,
                    y: 0,
                    dx: BALL_MOVE_DELTA,
                    dy: BALL_MOVE_DELTA,
                    mask,
                }))
            });
        }
        Message::Paint => {
            let paint = window.paint().unwrap();

            GLOBAL.with(|cell| {
                let mut global = cell.take().unwrap();
                let rect = window.get_rect().unwrap();
                draw_ball(&mut global, &paint, rect);
                cell.set(Some(global));
            });
        }
        Message::Timer(_timer) => {
            let paint = window.repaint().unwrap();

            GLOBAL.with(|cell| {
                let mut global = cell.take().unwrap();
                let rect = window.get_rect().unwrap();
                update_ball(&mut global, rect.width(), rect.height());
                draw_ball(&mut global, &paint, rect);
                cell.set(Some(global));
            });
        }
        Message::Close => {
            window.destroy().unwrap();
        }
        Message::Destroy => {
            window
                .kill_timer(NonZeroUsize::new(ID_TIMER).unwrap())
                .unwrap();
            GLOBAL.with(|cell| drop(cell.take().take()));
            win::post_quit_message(0);
        }
        _ => return None,
    }

    Some(0)
}

fn draw_ball(global: &mut Global, paint: &win::gdi::Canvas, rect: win::rect::Rect) {
    paint
        .fill_rect(rect.clone(), win::gdi::brush::white().unwrap())
        .unwrap();

    let ball_rect = win::rect::Rect::new(global.width, global.height).at(global.x, global.y);
    let canvas = paint.try_clone().unwrap();

    let canvas = canvas.bind(&global.mask).unwrap();
    paint
        .bitwise()
        .region(ball_rect.clone())
        .and(&canvas)
        .unwrap();
    let canvas = canvas.bind(&global.ball).unwrap();
    paint
        .bitwise()
        .region(ball_rect.clone())
        .or(&canvas)
        .unwrap();
}

fn update_ball(global: &mut Global, width: i32, height: i32) {
    global.x += global.dx;
    global.y += global.dy;

    if global.x < 0 {
        global.x = 0;
        global.dx = BALL_MOVE_DELTA;
    } else if global.x + global.width > width {
        global.x = width - global.width;
        global.dx = -BALL_MOVE_DELTA;
    }

    if global.y < 0 {
        global.y = 0;
        global.dy = BALL_MOVE_DELTA;
    } else if global.y + global.height > height {
        global.y = height - global.height;
        global.dy = -BALL_MOVE_DELTA;
    }
}
