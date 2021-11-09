//! Shows how to add basic menus and icons to your window during runtime.
//! http://winprog.org/tutorial/menus.html
use std::process::exit;
use minimal_windows_gui as win;

const CLASS_NAME: &str = "myWindowClass";

const ID_FILE_EXIT: u16 = 9001;
const ID_STUFF_GO: u16 = 9002;

fn main() -> win::Result<()> {
    let class = &win::class::build()
        .load_cursor(win::cursor::Cursor::Arrow)?
        .background(win::class::Background::Window)
        .register(CLASS_NAME)
        .expect("window registration failed");

    let window = win::window::build()
        .set_message_callback(main_window_callback)
        .add_extended_style(win::window::ExtendedStyle::ClientEdge)
        .add_style(win::window::Style::OverlappedWindow)
        .size(240, 120)
        .create(class, "A Menu #2")
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
            let menu = win::menu::Menu::new().unwrap();

            let submenu = win::menu::Menu::new_popup().unwrap();
            submenu.append_item("E&xit", ID_FILE_EXIT).unwrap();
            menu.append_menu("&File", submenu).unwrap();

            let submenu = win::menu::Menu::new_popup().unwrap();
            submenu.append_item("&Go", ID_STUFF_GO).unwrap();
            menu.append_menu("&Stuff", submenu).unwrap();

            window.set_menu(menu).unwrap();

            let icon = win::icon::Icon::from_file(r"examples\menu_one\menu_one.ico");
            match window.set_icon(icon) {
                Ok(_) => {}
                Err(_) => {
                    win::messagebox::message_box(
                        "Error",
                        "Could not load large icon! Is the program running from the project root?",
                        &[win::messagebox::Config::IconError],
                    )
                    .unwrap();
                }
            }

            let icon = win::icon::Icon::from_file(r"examples\menu_one\menu_one.ico");
            match window.set_small_icon(icon) {
                Ok(_) => {}
                Err(_) => {
                    win::messagebox::message_box(
                        "Error",
                        "Could not load small icon! Is the program running from the project root?",
                        &[win::messagebox::Config::IconError],
                    )
                    .unwrap();
                }
            }
        }
        Message::Command(info) => {
            if let Some(menu_id) = info.menu_id() {
                match menu_id {
                    ID_FILE_EXIT => {
                        window.close().unwrap();
                    }
                    ID_STUFF_GO => {
                        win::messagebox::message_box("Woo!", "You clicked Go!", &[]).unwrap();
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
