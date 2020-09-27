//! Window classes https://docs.microsoft.com/en-us/windows/win32/winmsg/about-window-classes.
//! Additionally contains methods to reference system classes.
use crate::{base_instance, cursor, icon, message, window, Error, Result};
use std::ffi::CString;
use std::num::NonZeroU16;
use std::ptr::{self, NonNull};
use winapi::shared::minwindef::{LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::windef::{HBRUSH, HCURSOR, HICON, HWND};
use winapi::um::winnt::{LPCSTR, LPSTR};
use winapi::um::winuser::{
    DefWindowProcA, RegisterClassExA, UnregisterClassA, COLOR_3DDKSHADOW, COLOR_3DFACE,
    COLOR_3DHIGHLIGHT, COLOR_3DLIGHT, COLOR_3DSHADOW, COLOR_ACTIVEBORDER, COLOR_ACTIVECAPTION,
    COLOR_APPWORKSPACE, COLOR_BTNTEXT, COLOR_CAPTIONTEXT, COLOR_DESKTOP,
    COLOR_GRADIENTACTIVECAPTION, COLOR_GRADIENTINACTIVECAPTION, COLOR_GRAYTEXT, COLOR_HIGHLIGHT,
    COLOR_HIGHLIGHTTEXT, COLOR_HOTLIGHT, COLOR_INACTIVEBORDER, COLOR_INACTIVECAPTION,
    COLOR_INACTIVECAPTIONTEXT, COLOR_INFOBK, COLOR_INFOTEXT, COLOR_MENU, COLOR_MENUBAR,
    COLOR_MENUHILIGHT, COLOR_MENUTEXT, COLOR_SCROLLBAR, COLOR_WINDOW, COLOR_WINDOWFRAME,
    COLOR_WINDOWTEXT, CS_BYTEALIGNCLIENT, CS_BYTEALIGNWINDOW, CS_CLASSDC, CS_DBLCLKS,
    CS_DROPSHADOW, CS_GLOBALCLASS, CS_HREDRAW, CS_NOCLOSE, CS_OWNDC, CS_PARENTDC, CS_SAVEBITS,
    CS_VREDRAW, MAKEINTRESOURCEA, WNDCLASSEXA,
};

/// Class styles as defined in https://docs.microsoft.com/en-us/windows/win32/winmsg/window-class-styles.
#[repr(u32)]
pub enum Style {
    /// Aligns the window's client area on a byte boundary (in the x direction). This style affects the width of the window and its horizontal placement on the display.
    ByteAlignClient = CS_BYTEALIGNCLIENT as UINT,

    /// Aligns the window on a byte boundary (in the x direction). This style affects the width of the window and its horizontal placement on the display.
    ByteAlignWindow = CS_BYTEALIGNWINDOW as UINT,

    /// Allocates one device context to be shared by all windows in the class. Because window classes are process specific, it is possible for multiple threads of an application to create a window of the same class. It is also possible for the threads to attempt to use the device context simultaneously. When this happens, the system allows only one thread to successfully finish its drawing operation.
    ClassDc = CS_CLASSDC as UINT,

    /// Sends a double-click message to the window procedure when the user double-clicks the mouse while the cursor is within a window belonging to the class.
    DoubleClicks = CS_DBLCLKS as UINT,

    /// Enables the drop shadow effect on a window. The effect is turned on and off through SPI_SETDROPSHADOW. Typically, this is enabled for small, short-lived windows such as menus to emphasize their Z-order relationship to other windows. Windows created from a class with this style must be top-level windows; they may not be child windows.
    DropShadow = CS_DROPSHADOW as UINT,

    /// Indicates that the window class is an application global class. For more information, see the "Application Global Classes" section of About Window Classes.
    GlobalClass = CS_GLOBALCLASS as UINT,

    /// Redraws the entire window if a movement or size adjustment changes the width of the client area.
    HorizontalRedraw = CS_HREDRAW as UINT,

    /// Disables Close on the window menu.
    NoClose = CS_NOCLOSE as UINT,

    /// Allocates a unique device context for each window in the class.
    Wndc = CS_OWNDC as UINT,

    /// PARENTDC enhances an application's performance. = Sets the clipping rectangle of the child window to that of the parent window so that the child can draw on the parent. A window with the PARENTDC style bit receives a regular device context from the system's cache of device contexts. It does not give the child the parent's device context or device context settings. Specifying PARENTDC enhances an application's performance.,
    ParentDc = CS_PARENTDC as UINT,

    /// Saves, as a bitmap, the portion of the screen image obscured by a window of this class. When the window is removed, the system uses the saved bitmap to restore the screen image, including other windows that were obscured. Therefore, the system does not send WM_PAINT messages to windows that were obscured if the memory used by the bitmap has not been discarded and if other screen actions have not invalidated the stored image.
    ///
    /// This style is useful for small windows (for example, menus or dialog boxes) that are displayed briefly and then removed before other screen activity takes place. This style increases the time required to display the window, because the system must first allocate memory to store the bitmap.
    SaveBits = CS_SAVEBITS as UINT,

    /// Redraws the entire window if a movement or size adjustment changes the height of the client area.
    VerticalRedraw = CS_VREDRAW as UINT,
}

// https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor
#[repr(i32)]
pub enum Background {
    /// Dark shadow for three-dimensional display elements.
    DarkShadow3D = COLOR_3DDKSHADOW,

    /// Face color for three-dimensional display elements and for dialog box backgrounds.
    /// The associated foreground color is `ButtonText`.
    /// Equivalent to `ButtonFace`.
    Face3D = COLOR_3DFACE,

    /// Highlight color for three-dimensional display elements (for edges facing the light source.)
    /// Equivalent to `ButtonHighlight`.
    Highlight3D = COLOR_3DHIGHLIGHT,

    /// Light color for three-dimensional display elements (for edges facing the light source.)
    Light3D = COLOR_3DLIGHT,

    /// Shadow color for three-dimensional display elements (for edges facing away from the light source).
    /// Equivalent to `ButtonShadow`.
    Shadow3D = COLOR_3DSHADOW,

    /// Active window border.
    ActiveBorder = COLOR_ACTIVEBORDER,

    /// Active window title bar. The associated foreground color is `CaptionText`.
    /// Specifies the left side color in the color gradient of an active window's title bar if
    /// the gradient effect is enabled.
    ActiveCaption = COLOR_ACTIVECAPTION,

    /// Background color of multiple document interface (MDI) applications.
    AppWorkspace = COLOR_APPWORKSPACE,

    /// Text on push buttons. The associated background color is `ButtonFace`.
    ButtonText = COLOR_BTNTEXT,

    /// Text in caption, size box, and scroll bar arrow box. The associated background color is `ActiveCaption`.
    CaptionText = COLOR_CAPTIONTEXT,

    /// Desktop. Equivalent to `Background`.
    Desktop = COLOR_DESKTOP,

    /// Right side color in the color gradient of an active window's title bar. `ActiveCaption`
    /// specifies the left side color.
    GradientActiveCaption = COLOR_GRADIENTACTIVECAPTION,

    /// Right side color in the color gradient of an inactive window's title bar. `InactiveCaption` specifies the left side color.
    GradientInactiveCaption = COLOR_GRADIENTINACTIVECAPTION,

    /// Grayed (disabled) text. This color is set to 0 if the current display driver does not support a solid gray color.
    GrayText = COLOR_GRAYTEXT,

    /// Item(s) selected in a control. The associated foreground color is `HighlightText`.
    Highlight = COLOR_HIGHLIGHT,

    /// Text of item(s) selected in a control. The associated background color is `Highlight`.
    HighlightText = COLOR_HIGHLIGHTTEXT,

    /// Color for a hyperlink or hot-tracked item. The associated background color is `Window`.
    HotLight = COLOR_HOTLIGHT,

    /// Inactive window border.
    InactiveBorder = COLOR_INACTIVEBORDER,

    /// Inactive window caption. The associated foreground color is `InactiveCaptionText`.
    /// Specifies the left side color in the color gradient of an inactive window's title bar
    /// if the gradient effect is enabled.
    InactiveCaption = COLOR_INACTIVECAPTION,

    /// Color of text in an inactive caption. The associated background color is `InactiveCaption`.
    InactiveCaptionText = COLOR_INACTIVECAPTIONTEXT,

    /// Background color for tooltip controls. The associated foreground color is `InfoText`.
    InfoBackground = COLOR_INFOBK,

    /// Text color for tooltip controls. The associated background color is `InfoBackground`.
    InfoText = COLOR_INFOTEXT,

    /// Menu background. The associated foreground color is `MenuText`.
    Menu = COLOR_MENU,

    /// The color used to highlight menu items when the menu appears as a flat menu.
    /// The highlighted menu item is outlined with `Highlight`.
    MenuHighlight = COLOR_MENUHILIGHT,

    /// The background color for the menu bar when menus appear as flat menus. However, `Menu`
    /// continues to specify the background color of the menu popup.
    MenuBar = COLOR_MENUBAR,

    /// Text in menus. The associated background color is `Menu`.
    MenuText = COLOR_MENUTEXT,

    /// Scroll bar gray area.
    ScrollBar = COLOR_SCROLLBAR,

    /// Window background. The associated foreground colors are `WindowText` and `HotLite`.
    Window = COLOR_WINDOW,

    /// Window frame.
    WindowFrame = COLOR_WINDOWFRAME,

    /// Text in windows. The associated background color is `Window`.
    WindowText = COLOR_WINDOWTEXT,
}

pub struct Builder {
    style: UINT,
    icon: HICON,
    cursor: HCURSOR,
    background: HBRUSH,
    menu: LPSTR,
    icon_small: HICON,
}

pub enum Class {
    Owned {
        class_name: CString,
        atom: NonZeroU16,
    },
    Static {
        class_name: &'static [u8],
    },
}

static BUTTON: Class = Class::Static {
    class_name: b"Button\0",
};
static COMBO_BOX: Class = Class::Static {
    class_name: b"ComboBox\0",
};
static EDIT_CONTROL: Class = Class::Static {
    class_name: b"Edit\0",
};
static LIST_BOX: Class = Class::Static {
    class_name: b"ListBox\0",
};
static MDI_CLIENT: Class = Class::Static {
    class_name: b"MDIClient\0",
};
static SCROLL_BAR: Class = Class::Static {
    class_name: b"ScrollBar\0",
};
static STATIC: Class = Class::Static {
    class_name: b"Static\0",
};

// um/CommCtrl.h
static TOOLBAR: Class = Class::Static {
    class_name: b"ToolbarWindow32\0",
};
static RE_BAR: Class = Class::Static {
    class_name: b"ReBarWindow32\0",
};
static STATUS: Class = Class::Static {
    class_name: b"msctls_statusbar32\0",
};

pub unsafe extern "system" fn wnd_proc_wrapper(
    handle: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if let Some(hwnd) = NonNull::new(handle) {
        let lock = crate::HWND_TO_CALLBACK.lock().unwrap();

        if let Some(&callback) = lock.get(&(handle as usize)).or_else(|| lock.get(&0)) {
            let window = window::Window::Borrowed { hwnd };
            let message = message::Message::from_raw(msg, wparam, lparam);

            // Callback may cause additional messages, leading to a deadlock unless dropped.
            drop(lock);
            if let Some(result) = callback(&window, message) {
                return result;
            }
        }
    }

    DefWindowProcA(handle, msg, wparam, lparam)
}

impl Builder {
    /// Adds a new class style.
    pub fn add_style(mut self, style: Style) -> Self {
        self.style |= style as UINT;
        self
    }

    /// Replaces the default class icon with a custom one. For this, the icon has to be loaded,
    /// which may fail.
    pub fn load_icon(mut self, icon: icon::Icon) -> Result<Self> {
        self.icon = icon.load()?.as_ptr();
        Ok(self)
    }

    /// Replaces the default cursor icon with a custom one. For this, the cursor has to be loaded,
    /// which may fail.
    pub fn load_cursor(mut self, cursor: cursor::Cursor) -> Result<Self> {
        self.cursor = cursor.load()?.as_ptr();
        Ok(self)
    }

    /// Sets the background brush to a standard system color.
    pub fn background(mut self, background: Background) -> Self {
        // "A color value must be one of the following standard system colors
        // (the value 1 must be added to the chosen color)""
        self.background = (background as i32 + 1) as HBRUSH;
        self
    }

    /// Sets the menu resource constant to use. This should be the same value as the one used
    /// in the resource file `.rc`.
    pub fn menu(mut self, menu: u16) -> Self {
        self.menu = MAKEINTRESOURCEA(menu);
        self
    }

    /// Replaces the default class icon with a custom one. For this, the icon has to be loaded,
    /// which may fail.
    pub fn load_small_icon(mut self, icon: icon::Icon) -> Result<Self> {
        self.icon_small = icon.load_small()?.as_ptr();
        Ok(self)
    }

    /// Registers a new window class.
    pub fn register(self, name: &str) -> Result<Class> {
        let class_name = CString::new(name)?;

        let atom = unsafe {
            // CreateWindowExA without a class will fail with 0x57f (ERROR_CANNOT_FIND_WND_CLASS).
            // https://docs.microsoft.com/en-us/windows/win32/debug/system-error-codes--1300-1699-
            //
            // For the method itself see:
            // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexa
            RegisterClassExA(&WNDCLASSEXA {
                cbSize: std::mem::size_of::<WNDCLASSEXA>() as u32,
                style: self.style,
                lpfnWndProc: Some(wnd_proc_wrapper),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: base_instance(),
                hIcon: self.icon,
                hCursor: self.cursor,
                hbrBackground: self.background,
                lpszMenuName: self.menu,
                lpszClassName: class_name.as_ptr() as LPCSTR,
                hIconSm: self.icon_small,
            })
        };

        if let Some(atom) = NonZeroU16::new(atom) {
            Ok(Class::Owned { class_name, atom })
        } else {
            Err(Error::last_os_error())
        }
    }
}

impl Class {
    pub(crate) fn class_name_ptr(&self) -> LPCSTR {
        match self {
            Class::Owned { atom, .. } => {
                // The atom must be in the low-order word of lpClassName; the high-order word must be zero.
                atom.get() as usize as LPCSTR
            }
            Class::Static { class_name } => class_name.as_ptr() as LPCSTR,
        }
    }
}

impl Drop for Class {
    fn drop(&mut self) {
        match self {
            Class::Owned { .. } => {
                let result =
                    unsafe { UnregisterClassA(self.class_name_ptr(), std::ptr::null_mut()) };

                if result == 0 {
                    panic!(format!(
                        "class deleted by other means or some window still alive: {}",
                        Error::last_os_error()
                    ))
                }
            }
            Class::Static { .. } => {}
        }
    }
}

/// Creates a builder to define a new application-defined class.
pub fn build() -> Builder {
    Builder {
        style: 0,
        icon: ptr::null_mut(),
        cursor: ptr::null_mut(),
        background: COLOR_WINDOW as HBRUSH,
        menu: ptr::null_mut(),
        icon_small: ptr::null_mut(),
    }
}

/// The system class for a button.
pub fn button() -> &'static Class {
    &BUTTON
}

/// The system class for a combo box.
pub fn combo_box() -> &'static Class {
    &COMBO_BOX
}

/// The system class for an edit control.
pub fn edit_control() -> &'static Class {
    &EDIT_CONTROL
}

/// The system class for a list box.
pub fn list_box() -> &'static Class {
    &LIST_BOX
}

/// The system class for an MDI client window.
pub fn mdi_client() -> &'static Class {
    &MDI_CLIENT
}

/// The system class for a scroll bar.
pub fn scroll_bar() -> &'static Class {
    &SCROLL_BAR
}

/// The system class for a static control.
pub fn static_control() -> &'static Class {
    &STATIC
}

/// The common control class for a tool bar.
pub fn toolbar() -> &'static Class {
    &TOOLBAR
}

/// The system class for a "re-bar".
pub fn re_bar() -> &'static Class {
    &RE_BAR
}

/// The system class for a status bar.
pub fn status_bar() -> &'static Class {
    &STATUS
}
