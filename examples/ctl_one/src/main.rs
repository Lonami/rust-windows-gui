//! Dialogs are just windows, which can be defined as dialog resources.
//! http://winprog.org/tutorial/dialogs.html
use std::process::exit;
use minimal_windows_gui as win;

const IDD_MAIN: u16 = 101;

const IDC_TEXT: u16 = 1000;
const IDC_NUMBER: u16 = 1001;
const IDC_LIST: u16 = 1002;
const IDC_ADD: u16 = 1003;
const IDC_CLEAR: u16 = 1004;
const IDC_REMOVE: u16 = 1005;
const IDC_SHOWCOUNT: u16 = 1006;

fn main() -> win::Result<()> {
    exit(win::dialog::show(IDD_MAIN, dialog_callback).unwrap() as i32)
}

fn dialog_callback(dialog: &win::window::Window, message: win::message::Message) -> isize {
    use win::message::Message;

    match message {
        Message::InitDialog => {
            let text_ctl = dialog.get_dialog_item(IDC_TEXT).unwrap();
            let number_ctl = dialog.get_dialog_item(IDC_NUMBER).unwrap();

            text_ctl.set_text("This is a string");
            number_ctl.set_text("5");
        }
        Message::Command(info) => {
            if let Some(control) = info.control_data() {
                handle_command(dialog, control).unwrap();
            }
        }
        Message::Close => {
            dialog.end_dialog(0).unwrap();
        }
        _ => return 0,
    }

    1
}

// Separate function to avoid a bit of rightwards drift.
fn handle_command(
    dialog: &win::window::Window,
    control: win::message::ControlData,
) -> win::Result<()> {
    let number_ctl = dialog.get_dialog_item(IDC_NUMBER)?;
    let text_ctl = dialog.get_dialog_item(IDC_TEXT)?;
    let list_ctl = dialog.get_dialog_item(IDC_LIST)?;
    let show_count_ctl = dialog.get_dialog_item(IDC_SHOWCOUNT)?;

    match control.id {
        IDC_ADD => match number_ctl.get_text().parse::<u32>() {
            Ok(n_times) => {
                let len = text_ctl.get_text_len();
                if len > 0 {
                    let text = text_ctl.get_text();
                    for _ in 0..n_times {
                        let index = list_ctl.add_string_item(&text).unwrap();
                        list_ctl.set_item_data(index, n_times as isize).unwrap();
                    }
                } else {
                    win::messagebox::message_box("Warning", "You didn't enter anything!", &[])?;
                }
            }
            Err(_) => {
                win::messagebox::message_box("Warning", "Couldn't translate that number :(", &[])?;
            }
        },
        IDC_REMOVE => match list_ctl.get_selection_count() {
            Ok(0) => {
                win::messagebox::message_box("Warning", "No items selected.", &[])?;
            }
            Ok(_) => {
                for index in list_ctl.get_selected_items().unwrap().into_iter().rev() {
                    list_ctl.delete_string_item(index).unwrap();
                }
            }
            Err(_) => {
                win::messagebox::message_box("Warning", "Error counting items :(", &[])?;
            }
        },
        IDC_LIST
            if matches!(
                control.list_box_code(),
                win::message::ListBoxMessage::SelectionChange
            ) =>
        {
            match list_ctl.get_selection_count() {
                Ok(1) => match list_ctl.get_selected_items() {
                    Ok(items) => {
                        let data = list_ctl.get_item_data(items[0] as usize).unwrap();
                        show_count_ctl.set_text(&data.to_string());
                    }
                    Err(_) => {
                        win::messagebox::message_box(
                            "Warning",
                            "Error getting selected item :(",
                            &[],
                        )?;
                    }
                },
                Ok(_) => {
                    show_count_ctl.set_text("-");
                }
                Err(_) => {
                    win::messagebox::message_box("Warning", "Error counting items :(", &[])?;
                }
            }
        }
        IDC_CLEAR => {
            list_ctl.clear_content();
        }
        _ => {}
    }
    Ok(())
}
