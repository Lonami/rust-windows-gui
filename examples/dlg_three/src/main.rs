//! Additional dialog messages without using a main window.
//! http://winprog.org/tutorial/dlgfaq.html
use std::cell::Cell;
use std::process::exit;
use winapi_app_windows as win;

const IDD_MAIN: u16 = 101;

thread_local! {
    static BACKGROUND_BRUSH: Cell<Option<win::brush::Brush>> = Cell::new(None);
}

fn main() -> win::Result<()> {
    exit(win::dialog::show(IDD_MAIN, dialog_callback).unwrap() as i32)
}

fn dialog_callback(dialog: &win::window::Window, message: win::message::Message) -> isize {
    use win::message::Message;
    use win::messagebox::Button;

    match message {
        Message::InitDialog => {
            let brush = win::brush::Brush::new_solid_rgb(0, 0, 0).unwrap();
            BACKGROUND_BRUSH.with(|cell| cell.set(Some(brush)));

            dialog.set_icon(win::icon::Icon::Application).unwrap();
            dialog.set_small_icon(win::icon::Icon::Application).unwrap();
        }
        Message::Close => {
            dialog.end_dialog(0).unwrap();
        }
        Message::ControlColorDialog(_info) => {
            let mut result = 0;
            BACKGROUND_BRUSH.with(|cell| {
                let brush = cell.take();
                result = brush.as_ref().unwrap().dlg_proc_value();
                cell.set(brush);
            });
            return result;
        }
        Message::ControlColorStatic(info) => {
            info.set_text_color(255, 255, 255).unwrap();
            info.set_background_transparency(true).unwrap();

            let mut result = 0;
            BACKGROUND_BRUSH.with(|cell| {
                let brush = cell.take();
                result = brush.as_ref().unwrap().dlg_proc_value();
                cell.set(brush);
            });
            return result;
        }
        Message::Command(info) => match info.control_data().map(|c| c.std_button()).flatten() {
            Some(Button::Ok) => dialog.end_dialog(0).unwrap(),
            _ => {}
        },
        Message::Destroy => {
            BACKGROUND_BRUSH.with(|cell| drop(cell.take().take()));
        }
        _ => return 0,
    }

    1
}
