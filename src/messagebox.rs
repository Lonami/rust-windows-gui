use crate::{Error, Result};
use std::ffi::CString;
use std::ptr;
use winapi::shared::minwindef::UINT;
use winapi::um::winuser::{
    MessageBoxA, IDABORT, IDCANCEL, IDCONTINUE, IDIGNORE, IDNO, IDOK, IDRETRY, IDTRYAGAIN, IDYES,
    MB_ABORTRETRYIGNORE, MB_APPLMODAL, MB_CANCELTRYCONTINUE, MB_DEFAULT_DESKTOP_ONLY,
    MB_DEFBUTTON2, MB_DEFBUTTON3, MB_DEFBUTTON4, MB_HELP, MB_ICONERROR, MB_ICONINFORMATION,
    MB_ICONQUESTION, MB_ICONWARNING, MB_OKCANCEL, MB_RETRYCANCEL, MB_RIGHT, MB_RTLREADING,
    MB_SERVICE_NOTIFICATION, MB_SETFOREGROUND, MB_SYSTEMMODAL, MB_TASKMODAL, MB_TOPMOST, MB_YESNO,
    MB_YESNOCANCEL,
};

/// Message box type configuration as defined in https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messagebox.
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum Config {
    /// The message box contains three push buttons: Abort, Retry, and Ignore.
    AbortRetryIgnore = MB_ABORTRETRYIGNORE,

    /// The message box contains three push buttons: Cancel, Try Again, Continue. Use this message box type instead of `AbortRetryIgnore`.
    CancelTryContinue = MB_CANCELTRYCONTINUE,

    /// Adds a Help button to the message box. When the user clicks the Help button or presses F1, the system sends a `Help` message to the owner.
    Help = MB_HELP,

    /// The message box contains two push buttons: OK and Cancel.
    OkCancel = MB_OKCANCEL,

    /// The message box contains two push buttons: Retry and Cancel.
    RetryCancel = MB_RETRYCANCEL,

    /// The message box contains two push buttons: Yes and No.
    YesNo = MB_YESNO,

    /// The message box contains three push buttons: Yes, No, and Cancel.
    YesNoCancel = MB_YESNOCANCEL,

    /// An exclamation-point icon appears in the message box.
    IconWarning = MB_ICONWARNING,

    /// An icon consisting of a lowercase letter i in a circle appears in the message box.
    IconInformation = MB_ICONINFORMATION,

    /// A question-mark icon appears in the message box. The question-mark message icon is no longer recommended because it does not clearly represent a specific type of message and because the phrasing of a message as a question could apply to any message type. In addition, users can confuse the message symbol question mark with Help information. Therefore, do not use this question mark message symbol in your message boxes. The system continues to support its inclusion only for backward compatibility.
    IconQuestion = MB_ICONQUESTION,

    /// A stop-sign icon appears in the message box.
    IconError = MB_ICONERROR,

    /// The second button is the default button.
    DefaultButton2 = MB_DEFBUTTON2,

    /// The third button is the default button.
    DefaultButton3 = MB_DEFBUTTON3,

    /// The fourth button is the default button.
    DefaultButton4 = MB_DEFBUTTON4,

    /// The user must respond to the message box before continuing work in the window identified by the hWnd parameter. However, the user can move to the windows of other threads and work in those windows. Depending on the hierarchy of windows in the application, the user may be able to move to other windows within the thread. All child windows of the parent of the message box are automatically disabled, but pop-up windows are not. `ApplicationModal` is the default if neither `SystemModal` nor `TaskModal` is specified.
    ApplicationModal = MB_APPLMODAL,

    /// Same as `ApplicationModal` except that the message box has the `TopMost` style. Use system-modal message boxes to notify the user of serious, potentially damaging errors that require immediate attention (for example, running out of memory). This flag has no effect on the user's ability to interact with windows other than those associated with hWnd.
    SystemModal = MB_SYSTEMMODAL,

    /// Same as `ApplicationModal` except that all the top-level windows belonging to the current thread are disabled if the hWnd parameter is NULL. Use this flag when the calling application or library does not have a window handle available but still needs to prevent input to other windows in the calling thread without suspending other threads.
    TaskModal = MB_TASKMODAL,

    /// Same as desktop of the interactive window station. For more information, see Window Stations. If the current input desktop is not the default desktop, MessageBox does not return until the user switches to the default desktop.
    DefaultDesktopOnly = MB_DEFAULT_DESKTOP_ONLY,

    /// The text is right-justified.
    Right = MB_RIGHT,

    /// Displays message and caption text using right-to-left reading order on Hebrew and Arabic systems.
    RtlReading = MB_RTLREADING,

    /// The message box becomes the foreground window. Internally, the system calls the SetForegroundWindow function for the message box.
    SetForeground = MB_SETFOREGROUND,

    /// The message box is created with the `TopMost` window style.
    TopMost = MB_TOPMOST,

    /// The caller is a service notifying the user of an event. The function displays a message box on the current active desktop, even if there is no user logged on to the computer. Terminal Services: If the calling thread has an impersonation token, the function directs the message box to the session specified in the impersonation token. If this flag is set, the hWnd parameter must be NULL. This is so that the message box can appear on a desktop other than the desktop corresponding to the hWnd. For information on security considerations in regard to using this flag, see Interactive Services. In particular, be aware that this flag can produce interactive content on a locked desktop and should therefore be used for only a very limited set of scenarios, such as resource exhaustion.
    ServiceNotification = MB_SERVICE_NOTIFICATION,
}

/// Indicates which button was selected.
///
/// If a message box has a Cancel button, the function returns the `Cancel` value if either the
/// ESC key is pressed or the Cancel button is selected. If the message box has no Cancel button,
/// pressing ESC will no effect - unless an `Ok` button is present. If an `Ok` button is
/// displayed and the user presses ESC, the return value will be `Ok`.
#[derive(Debug)]
#[repr(i32)]
pub enum Button {
    Abort = IDABORT,
    Cancel = IDCANCEL,
    Continue = IDCONTINUE,
    Ignore = IDIGNORE,
    No = IDNO,
    Ok = IDOK,
    Retry = IDRETRY,
    TryAgain = IDTRYAGAIN,
    Yes = IDYES,
}

/// If no config is provided, the message box defaults to containing one push button: OK.
/// The first button is the default button
pub fn message_box(caption: &str, text: &str, config: &[Config]) -> Result<Button> {
    let caption = CString::new(caption)?;
    let text = CString::new(text)?;
    let result = unsafe {
        // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messagebox
        MessageBoxA(
            ptr::null_mut(),
            text.as_ptr(),
            caption.as_ptr(),
            config.into_iter().fold(0, |acc, x| acc | *x as UINT),
        )
    };

    Ok(match result {
        0 => return Err(Error::last_os_error()),
        IDABORT => Button::Abort,
        IDCANCEL => Button::Cancel,
        IDCONTINUE => Button::Continue,
        IDIGNORE => Button::Ignore,
        IDNO => Button::No,
        IDOK => Button::Ok,
        IDRETRY => Button::Retry,
        IDTRYAGAIN => Button::TryAgain,
        IDYES => Button::Yes,
        other => panic!(format!("invalid return code from message box: {}", other)),
    })
}
