//! Using files and the common dialogs.
//! http://winprog.org/tutorial/app_two.html
use std::fs::File;
use std::io::{Read, Write};
use std::process::exit;
use winapi_app_windows as win;

const CLASS_NAME: &str = "myWindowClass";

const IDC_MAIN_EDIT: u16 = 101;

const ID_FILE_EXIT: u16 = 40001;
const ID_FILE_OPEN: u16 = 40002;
const ID_FILE_SAVEAS: u16 = 40003;
const ID_FILE_NEW: u16 = 40004;

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
            let edit_ctl = win::window::build()
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
                .expect("edit box creation failed");

            let font = win::font::get_default().unwrap();
            edit_ctl.set_font(font);

            // Created after so that its automatic ID does not collide with other controls.
            // The original tutorial uses a resource file but for simplicity we want to avoid
            // the additional complexity.
            // TODO figure out how/if it's possible to set a child ID to a menu like this.
            let menu = win::menu::Menu::new().unwrap();
            let submenu = win::menu::Menu::new_popup().unwrap();
            submenu.append_item("&New", ID_FILE_NEW).unwrap();
            submenu.append_item("&Open...", ID_FILE_OPEN).unwrap();
            submenu.append_item("Save &As...", ID_FILE_SAVEAS).unwrap();
            submenu.append_separator().unwrap();
            submenu.append_item("E&xit", ID_FILE_EXIT).unwrap();
            menu.append_menu("&File", submenu).unwrap();
            window.set_menu(menu).unwrap();
        }
        Message::Size(info) => {
            let edit_ctl = window.get_dialog_item(IDC_MAIN_EDIT).unwrap();
            edit_ctl
                .set_rect(0, 0, info.width() as i32, info.height() as i32)
                .unwrap();
        }
        Message::Command(info) => match info.menu_id().unwrap_or(0) {
            ID_FILE_EXIT => {
                window.close().unwrap();
            }
            ID_FILE_NEW => {
                let edit_ctl = window.get_dialog_item(IDC_MAIN_EDIT).unwrap();
                edit_ctl.set_text("");
            }
            ID_FILE_OPEN => {
                do_open(window);
            }
            ID_FILE_SAVEAS => {
                do_save(window);
            }
            _ => {}
        },
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

// The whole file creation, obtaining the file size, allocating a large enough buffer, reading
// and writing, could of course be abstracted away by this library. However, Rust's standard
// library already provides wonderful abstractions to do this, which means this library does
// not need to provide abstractions for those (and so it doesn't).

fn do_open(window: &win::window::Window) {
    use win::dialog::OpenFileConfig as Config;

    if let Some(file) = window
        .open_file()
        .set_filters(&[("Text Files (*.txt)", "*.txt"), ("All Files (*.*)", "*.*")])
        .set_default_ext("txt")
        .add_config(Config::Explorer)
        .add_config(Config::FileMustExist)
        .add_config(Config::HideReadonly)
        .ask_open_path()
    {
        let mut file = File::open(file).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let edit_ctl = window.get_dialog_item(IDC_MAIN_EDIT).unwrap();
        edit_ctl.set_text(&contents);
    }
}

fn do_save(window: &win::window::Window) {
    use win::dialog::OpenFileConfig as Config;

    if let Some(file) = window
        .open_file()
        .set_filters(&[("Text Files (*.txt)", "*.txt"), ("All Files (*.*)", "*.*")])
        .set_default_ext("txt")
        .add_config(Config::Explorer)
        .add_config(Config::PathMustExist)
        .add_config(Config::HideReadonly)
        .add_config(Config::OverwritePrompt)
        .ask_save_path()
    {
        let edit_ctl = window.get_dialog_item(IDC_MAIN_EDIT).unwrap();
        let contents = edit_ctl.get_text();

        let mut file = File::create(file).unwrap();
        file.write_all(contents.as_bytes()).unwrap();
    }
}
