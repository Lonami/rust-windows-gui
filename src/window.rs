use crate::{base_instance, class, Error, MessageCallback, Result};
use std::ffi::CString;
use std::marker::PhantomData;
use std::ptr::NonNull;
use winapi::ctypes::c_int;
use winapi::shared::minwindef::DWORD;
use winapi::shared::windef::{HWND, HWND__};
use winapi::um::winnt::LPCSTR;
use winapi::um::winuser::{
    CreateWindowExA, DestroyWindow, PostMessageA, ShowWindow, UpdateWindow, CW_USEDEFAULT,
    SW_FORCEMINIMIZE, SW_HIDE, SW_MAXIMIZE, SW_MINIMIZE, SW_RESTORE, SW_SHOW, SW_SHOWDEFAULT,
    SW_SHOWMINIMIZED, SW_SHOWMINNOACTIVE, SW_SHOWNA, SW_SHOWNOACTIVATE, SW_SHOWNORMAL, WM_CLOSE,
    WS_BORDER, WS_CAPTION, WS_CHILD, WS_CLIPCHILDREN, WS_CLIPSIBLINGS, WS_DISABLED, WS_DLGFRAME,
    WS_EX_ACCEPTFILES, WS_EX_APPWINDOW, WS_EX_CLIENTEDGE, WS_EX_COMPOSITED, WS_EX_CONTEXTHELP,
    WS_EX_CONTROLPARENT, WS_EX_DLGMODALFRAME, WS_EX_LAYERED, WS_EX_LAYOUTRTL, WS_EX_LEFT,
    WS_EX_LEFTSCROLLBAR, WS_EX_MDICHILD, WS_EX_NOACTIVATE, WS_EX_NOINHERITLAYOUT,
    WS_EX_NOPARENTNOTIFY, WS_EX_NOREDIRECTIONBITMAP, WS_EX_OVERLAPPEDWINDOW, WS_EX_PALETTEWINDOW,
    WS_EX_RIGHT, WS_EX_RTLREADING, WS_EX_STATICEDGE, WS_EX_TOOLWINDOW, WS_EX_TOPMOST,
    WS_EX_TRANSPARENT, WS_EX_WINDOWEDGE, WS_GROUP, WS_HSCROLL, WS_MAXIMIZE, WS_MINIMIZE,
    WS_OVERLAPPED, WS_OVERLAPPEDWINDOW, WS_POPUP, WS_POPUPWINDOW, WS_SYSMENU, WS_TABSTOP,
    WS_THICKFRAME, WS_VISIBLE, WS_VSCROLL,
};

/// Extended window styles as defined in https://docs.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles.
#[repr(u32)]
pub enum ExtendedStyle {
    /// The window accepts drag-drop files.
    AcceptFiles = WS_EX_ACCEPTFILES as DWORD,

    /// Forces a top-level window onto the taskbar when the window is visible.
    AppWindow = WS_EX_APPWINDOW as DWORD,

    /// The window has a border with a sunken edge.
    ClientEdge = WS_EX_CLIENTEDGE as DWORD,

    /// Paints all descendants of a window in bottom-to-top painting order using double-buffering. Bottom-to-top painting order allows a descendent window to have translucency (alpha) and transparency (color-key) effects, but only if the descendent window also has the `Transparent` bit set. Double-buffering allows the window and its descendents to be painted without flicker. This cannot be used if the window has a class style of either `OwnDc` or `ClassDc`. Windows 2000: This style is not supported.
    Composited = WS_EX_COMPOSITED as DWORD,

    /// The title bar of the window includes a question mark. When the user clicks the question mark, the cursor changes to a question mark with a pointer. If the user then clicks a child window, the child receives a WM_HELP message. The child window should pass the message to the parent window procedure, which should call the WinHelp function using the HELP_WM_HELP command. The Help application displays a pop-up window that typically contains help for the child window. `ContextHelp` cannot be used with the `MaximizeBox` or `MinimizeBox` styles.
    ContextHelp = WS_EX_CONTEXTHELP as DWORD,

    /// The window itself contains child windows that should take part in dialog box navigation. If this style is specified, the dialog manager recurses into children of this window when performing navigation operations such as handling the TAB key, an arrow key, or a keyboard mnemonic.
    ControlParent = WS_EX_CONTROLPARENT as DWORD,

    /// The window has a double border; the window can, optionally, be created with a title bar by specifying the `Caption` style in the `style` parameter.
    DialogModalFrame = WS_EX_DLGMODALFRAME as DWORD,

    /// The window is a layered window. This style cannot be used if the window has a class style of either `OwnDc` or `ClassDc`. Windows 8: The `Layered` style is supported for top-level windows and child windows. Previous Windows versions support `Layered` only for top-level windows.
    Layered = WS_EX_LAYERED as DWORD,

    /// If the shell language is Hebrew, Arabic, or another language that supports reading order alignment, the horizontal origin of the window is on the right edge. Increasing horizontal values advance to the left.
    LayoutRtl = WS_EX_LAYOUTRTL as DWORD,

    /// The window has generic left-aligned properties. The window text is displayed using left-to-right reading-order properties (LTR reading). The vertical scroll bar (if present) is to the right of the client area (right scrollbar). This is the default.
    Left = WS_EX_LEFT as DWORD,

    /// If the shell language is Hebrew, Arabic, or another language that supports reading order alignment, the vertical scroll bar (if present) is to the left of the client area. For other languages, the style is ignored.
    LeftScrollbar = WS_EX_LEFTSCROLLBAR as DWORD,

    /// The window is a MDI child window.
    MdiChild = WS_EX_MDICHILD as DWORD,

    /// A top-level window created with this style does not become the foreground window when the user clicks it. The system does not bring this window to the foreground when the user minimizes or closes the foreground window. The window should not be activated through programmatic access or via keyboard navigation by accessible technology, such as Narrator. To activate the window, use the `Window::set_active` or `Window::set_foreground` function. The window does not appear on the taskbar by default. To force the window to appear on the taskbar, use the `AppWindow` style.
    NoActivate = WS_EX_NOACTIVATE as DWORD,

    /// The window does not pass its window layout to its child windows.
    NoInheritLayout = WS_EX_NOINHERITLAYOUT as DWORD,

    /// The child window created with this style does not send the `ParentNotify` message to its parent window when it is created or destroyed.
    NoParentNotify = WS_EX_NOPARENTNOTIFY as DWORD,

    /// The window does not render to a redirection surface. This is for windows that do not have visible content or that use mechanisms other than surfaces to provide their visual.
    NoRedirectionBitmap = WS_EX_NOREDIRECTIONBITMAP as DWORD,

    /// The window is an overlapped window. Equivalent to adding both `WindowEdge` and `ClientEdge`.
    OverlappedWindow = WS_EX_OVERLAPPEDWINDOW as DWORD,

    /// The window is palette window, which is a modeless dialog box that presents an array of commands. Equivalent to adding `WindowEdge`, `ToolWindow` and `TopMost`.
    PaletteWindow = WS_EX_PALETTEWINDOW as DWORD,

    /// The window has generic "right-aligned" properties. This depends on the window class. This style has an effect only if the shell language is Hebrew, Arabic, or another language that supports reading-order alignment; otherwise, the style is ignored. Using the `Right` style for static or edit controls has the same effect as using the SS_RIGHT or ES_RIGHT style, respectively. Using this style with button controls has the same effect as using BS_RIGHT and BS_RIGHTBUTTON styles.
    Right = WS_EX_RIGHT as DWORD,

    /// If the shell language is Hebrew, Arabic, or another language that supports reading-order alignment, the window text is displayed using right-to-left reading-order properties. For other languages, the style is ignored.
    RtlReading = WS_EX_RTLREADING as DWORD,

    /// The window has a three-dimensional border style intended to be used for items that do not accept user input.
    StaticEdge = WS_EX_STATICEDGE as DWORD,

    /// The window is intended to be used as a floating toolbar. A tool window has a title bar that is shorter than a normal title bar, and the window title is drawn using a smaller font. A tool window does not appear in the taskbar or in the dialog that appears when the user presses ALT+TAB. If a tool window has a system menu, its icon is not displayed on the title bar. However, you can display the system menu by right-clicking or by typing ALT+SPACE.
    ToolWindow = WS_EX_TOOLWINDOW as DWORD,

    /// The window should be placed above all non-topmost windows and should stay above them, even when the window is deactivated. To add or remove this style, use the SetWindowPos function.
    TopMost = WS_EX_TOPMOST as DWORD,

    /// The window should not be painted until siblings beneath the window (that were created by the same thread) have been painted. The window appears transparent because the bits of underlying sibling windows have already been painted. To achieve transparency without these restrictions, use the SetWindowRgn function.
    Transparent = WS_EX_TRANSPARENT as DWORD,

    /// The window has a border with a raised edge.
    WindowEdge = WS_EX_WINDOWEDGE as DWORD,
}

/// Window styles as defined in https://docs.microsoft.com/en-us/windows/win32/winmsg/window-styles.
#[repr(u32)]
pub enum Style {
    /// The window has a thin-line border.
    Border = WS_BORDER,

    /// The window has a title bar (includes the `Border` style).
    Caption = WS_CAPTION,

    /// The window is a child window. A window with this style cannot have a menu bar. This style cannot be used with the `Popup` style.
    Child = WS_CHILD,

    /// Excludes the area occupied by child windows when drawing occurs within the parent window. This style is used when creating the parent window.
    ClipChildren = WS_CLIPCHILDREN,

    /// Clips child windows relative to each other; that is, when a particular child window receives a `Paint` message, the `ClipSiblings` style clips all other overlapping child windows out of the region of the child window to be updated. If `ClipSiblings` is not specified and child windows overlap, it is possible, when drawing within the client area of a child window, to draw within the client area of a neighboring child window.
    ClipSiblings = WS_CLIPSIBLINGS,

    /// The window is initially disabled. A disabled window cannot receive input from the user. To change this after a window has been created, use the `Window::enable` function.
    Disabled = WS_DISABLED,

    /// The window has a border of a style typically used with dialog boxes. A window with this style cannot have a title bar.
    DialogFrame = WS_DLGFRAME,

    /// The window is the first control of a group of controls. The group consists of this first control and all controls defined after it, up to the next control with the `Group` style. The first control in each group usually has the `TabStop` style so that the user can move from group to group. The user can subsequently change the keyboard focus from one control in the group to the next control in the group by using the direction keys. You can turn this style on and off to change dialog box navigation. To change this style after a window has been created, use the SetWindowLong function. The window has a minimize button. Cannot be combined with the `ContextHelp` style. The `SysMenu` style must also be specified.
    Group = WS_GROUP,

    /// The window has a horizontal scroll bar.
    HorizontalScroll = WS_HSCROLL,

    /// The window is initially maximized.
    Maximize = WS_MAXIMIZE,

    /// The window is initially minimized. Same as the `Iconic` style.
    Minimize = WS_MINIMIZE,

    /// The window is an overlapped window. An overlapped window has a title bar and a border. Same as the `Tiled` style.
    Overlapped = WS_OVERLAPPED,

    /// The window is an overlapped window. Same as the `TileWindow` style.
    OverlappedWindow = WS_OVERLAPPEDWINDOW,

    /// The window is a pop-up window. This style cannot be used with the `Child` style.
    Popup = WS_POPUP,

    /// The window is a pop-up window. The `Caption` and `PopupWindow` styles must be combined to make the window menu visible.
    PopupWindow = WS_POPUPWINDOW,

    /// The window has a window menu on its title bar. The `Caption` style must also be specified.
    SysMenu = WS_SYSMENU,

    /// The window is a control that can receive the keyboard focus when the user presses the TAB key. Pressing the TAB key changes the keyboard focus to the next control with the `TabStop` style. You can turn this style on and off to change dialog box navigation. To change this style after a window has been created, use the SetWindowLong function. For user-created windows and modeless dialogs to work with tab stops, alter the message loop to call the IsDialogMessage function. The window has a maximize button. Cannot be combined with the `ContextHelp` style. The `SysMenu` style must also be specified.
    TabStop = WS_TABSTOP,

    /// The window has a sizing border. Same as the `SizeBox` style.
    ThickFrame = WS_THICKFRAME,

    /// The window is initially visible. This style can be turned on and off by using the `Window::show` or `Window::set_pos` function.
    Visible = WS_VISIBLE,

    /// The window has a vertical scroll bar.
    VerticalScroll = WS_VSCROLL,
}

/// Window show states as defined in https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow.
#[repr(i32)]
pub enum Show {
    /// Minimizes a window, even if the thread that owns the window is not responding. This flag should only be used when minimizing windows from a different thread.
    ForceMinimize = SW_FORCEMINIMIZE,

    /// Hides the window and activates another window.
    Hide = SW_HIDE,

    /// Activates the window and displays it as a maximized window.
    Maximize = SW_MAXIMIZE,

    /// Minimizes the specified window and activates the next top-level window in the Z order.
    Minimize = SW_MINIMIZE,

    /// Activates and displays the window. If the window is minimized or maximized, the system restores it to its original size and position. An application should specify this flag when restoring a minimized window.
    Restore = SW_RESTORE,

    /// Activates the window and displays it in its current size and position.
    Show = SW_SHOW,

    /// Sets the show state based on the `Show` value specified in the STARTUPINFO structure passed to the CreateProcess function by the program that started the application.
    ShowDefault = SW_SHOWDEFAULT,

    /// Activates the window and displays it as a minimized window.
    ShowMinimized = SW_SHOWMINIMIZED,

    /// Displays the window as a minimized window. This value is similar to `ShowMinimized`, except the window is not activated.
    ShowMinNoActive = SW_SHOWMINNOACTIVE,

    /// Displays the window in its current size and position. This value is similar to `Show`, except that the window is not activated.
    ShowNa = SW_SHOWNA,

    /// Displays a window in its most recent size and position. This value is similar to `ShowNormal`, except that the window is not activated.
    ShowNoActivate = SW_SHOWNOACTIVATE,

    /// Activates and displays a window. If the window is minimized or maximized, the system restores it to its original size and position. An application should specify this flag when displaying the window for the first time.
    ShowNormal = SW_SHOWNORMAL,
}

pub struct Builder {
    extended_style: DWORD,
    style: DWORD,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    callback: Option<MessageCallback>,
}

pub enum Window<'a> {
    Owned {
        _window_name: CString,
        hwnd: NonNull<HWND__>,
        _marker: PhantomData<&'a HWND__>,
    },
    Borrowed {
        hwnd: NonNull<HWND__>,
    },
}

impl Builder {
    /// Adds a new extended window style.
    pub fn add_extended_style(mut self, style: ExtendedStyle) -> Self {
        self.extended_style |= style as DWORD;
        self
    }

    /// Adds a new window style.
    pub fn add_style(mut self, style: Style) -> Self {
        self.style |= style as DWORD;
        self
    }

    /// The initial horizontal position of the window. For an overlapped or pop-up window, the x parameter is the initial x-coordinate of the window's upper-left corner, in screen coordinates. For a child window, x is the x-coordinate of the upper-left corner of the window relative to the upper-left corner of the parent window's client area. If x is kept to its default value, the system selects the default position for the window's upper-left corner and ignores the y parameter. The default value is valid only for overlapped windows; if it is specified for a pop-up or child window, the x and y parameters are set to zero.
    pub fn x(mut self, x: u16) -> Self {
        self.x = x as i32;
        self
    }

    /// The initial vertical position of the window. For an overlapped or pop-up window, the y parameter is the initial y-coordinate of the window's upper-left corner, in screen coordinates. For a child window, y is the initial y-coordinate of the upper-left corner of the child window relative to the upper-left corner of the parent window's client area. For a list box y is the initial y-coordinate of the upper-left corner of the list box's client area relative to the upper-left corner of the parent window's client area.
    ///
    /// If an overlapped window is created with the `Visible` style bit set and the x parameter is set to its default value, then the y parameter determines how the window is shown. If the y parameter is using the default value, then the window manager calls `Window::show` with the `Show` flag after the window has been created. If the y parameter is some other value, then the window manager calls ShowWindow with that value as the nCmdShow parameter.
    pub fn y(mut self, y: u16) -> Self {
        self.y = y as i32;
        self
    }

    /// The initial position of the window (equivalent to modifying both `x` and `y`).
    pub fn pos(mut self, x: u16, y: u16) -> Self {
        self.x = x as i32;
        self.y = y as i32;
        self
    }

    /// The width, in device units, of the window. For overlapped windows, this is the window's width, in screen coordinates, or the default. If the value is the default, the system selects a default width and height for the window; the default width extends from the initial x-coordinates to the right edge of the screen; the default height extends from the initial y-coordinate to the top of the icon area. The default value is valid only for overlapped windows; if the default value is specified for a pop-up or child window, the width and height parameter are set to zero.
    pub fn width(mut self, width: u16) -> Self {
        self.width = width as i32;
        self
    }

    /// The height, in device units, of the window. For overlapped windows, this is the window's height, in screen coordinates. If the width parameter is set to its default value, the system ignores the height.
    pub fn height(mut self, height: u16) -> Self {
        self.height = height as i32;
        self
    }

    /// The initial size of the window (equivalent to modifying both `width` and `height`).
    pub fn size(mut self, width: u16, height: u16) -> Self {
        self.width = width as i32;
        self.height = height as i32;
        self
    }

    /// Callback that will process messages sent to this window.
    pub fn set_message_callback(mut self, callback: MessageCallback) -> Self {
        self.callback = Some(callback);
        self
    }

    pub fn create<'a>(self, class: &'a class::Class, name: &str) -> Result<Window<'a>> {
        let window_name = CString::new(name)?;
        let hwnd = unsafe {
            // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexa
            CreateWindowExA(
                self.extended_style,
                class.class_name_ptr(),
                window_name.as_ptr() as LPCSTR,
                self.style,
                self.x,
                self.y,
                self.width,
                self.height,
                std::ptr::null_mut(), // no parent
                std::ptr::null_mut(), // no class menu
                base_instance(),
                std::ptr::null_mut(), // no creation data
            )
        };

        if let Some(hwnd) = NonNull::new(hwnd) {
            let window = Window::Owned {
                _window_name: window_name,
                hwnd,
                _marker: PhantomData,
            };

            if let Some(callback) = self.callback {
                crate::HWND_TO_CALLBACK
                    .lock()
                    .unwrap()
                    .insert(hwnd.as_ptr() as usize, callback);
            }

            Ok(window)
        } else {
            Err(Error::last_os_error())
        }
    }
}

impl Window<'_> {
    fn hwnd_ptr(&self) -> HWND {
        match self {
            Window::Owned { hwnd, .. } => hwnd.as_ptr(),
            Window::Borrowed { hwnd } => hwnd.as_ptr(),
        }
    }

    /// Sets the show state based on the startup information when the process was created.
    pub fn show_default(&self) -> bool {
        self.set_show_state(Show::ShowDefault)
    }

    /// Sets a custom show state. Returns whether the window was visible before.
    pub fn set_show_state(&self, show: Show) -> bool {
        (unsafe { ShowWindow(self.hwnd_ptr(), show as c_int) }) != 0
    }

    /// Updates the client area of the specified window by sending a `Paint` message to the window if the window's update region is not empty. The function sends a `Paint` message directly to the window procedure of the specified window, bypassing the application queue. If the update region is empty, no message is sent.
    pub fn update(&self) -> std::result::Result<(), ()> {
        let result = unsafe { UpdateWindow(self.hwnd_ptr()) };
        if result != 0 {
            Ok(())
        } else {
            Err(())
        }
    }

    /// Destroys the specified window. The function sends `Destroy` and `NcDestroy` messages to
    /// the window to deactivate it and remove the keyboard focus from it. The function also
    /// destroys the window's menu, flushes the thread message queue, destroys timers, removes
    /// clipboard ownership, and breaks the clipboard viewer chain (if the window is at the top
    /// of the viewer chain).
    ///
    /// If the specified window is a parent or owner window, `destroy` automatically destroys
    /// the associated child or owned windows when it destroys the parent or owner window. The
    /// function first destroys child or owned windows, and then it destroys the parent or owner
    /// window.
    ///
    /// `destroy` also destroys modeless dialog boxes created by the CreateDialog function.
    pub fn destroy(&self) -> Result<()> {
        let result = unsafe { DestroyWindow(self.hwnd_ptr()) };

        if result != 0 {
            Ok(())
        } else {
            Err(Error::last_os_error())
        }
    }

    /// Indicates to the system that a window or an application should terminate.
    pub fn close(&self) -> Result<()> {
        let result = unsafe { PostMessageA(self.hwnd_ptr(), WM_CLOSE, 0, 0) };

        if result != 0 {
            Ok(())
        } else {
            Err(Error::last_os_error())
        }
    }
}

impl Drop for Window<'_> {
    fn drop(&mut self) {
        match self {
            Window::Owned { .. } => {
                match self.destroy() {
                    Ok(_) => {}
                    Err(e) => panic!(format!(
                        "destroying window {:?} failed: {}",
                        self.hwnd_ptr(),
                        e,
                    )),
                };
            }
            Window::Borrowed { .. } => {}
        }
    }
}

pub fn build() -> Builder {
    Builder {
        extended_style: 0,
        style: 0,
        x: CW_USEDEFAULT,
        y: CW_USEDEFAULT,
        width: CW_USEDEFAULT,
        height: CW_USEDEFAULT,
        callback: None,
    }
}
