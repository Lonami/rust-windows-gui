use crate::{base_instance, window, DialogCallback, Error, Result};
use std::ptr;
use winapi::um::winuser::{DialogBoxParamA, MAKEINTRESOURCEA};

/// Creates a modal dialog box from a dialog box template resource. The function does not
/// return control until the specified callback function terminates the modal dialog box
/// by calling the `Window::end_dialog` function.
pub fn show(resource: u16, callback: DialogCallback) -> Result<isize> {
    let hinstance = base_instance();
    let resource = MAKEINTRESOURCEA(resource);

    // Can't know what the dialog's handle is beforehand. The special value 0 will be
    // replaced with the right value as soon as the init dialog message arrives.
    crate::HWND_TO_DLG_CALLBACK
        .lock()
        .unwrap()
        .insert(0, callback);

    let result = unsafe {
        DialogBoxParamA(
            hinstance,
            resource,
            ptr::null_mut(),
            Some(window::dlg_proc_wrapper),
            0,
        )
    };

    // In the code at http://winprog.org/tutorial/dlgfaq.html, DialogBox returns 0 as well,
    // which according to the official documentation "If the function fails because the
    // hWndParent parameter is invalid, the return value is zero. The function returns zero
    // in this case for compatibility with previous versions of Windows.".
    // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-dialogboxparama
    //
    // It seems safe to ignore 0 as there being an error.
    match result {
        -1 => Err(Error::last_os_error()),
        n => Ok(n),
    }
}
