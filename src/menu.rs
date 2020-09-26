use crate::{Error, Result};
use std::ffi::CString;
use std::ptr::NonNull;
use winapi::shared::windef::{HMENU, HMENU__};
use winapi::um::winuser::{AppendMenuA, CreateMenu, CreatePopupMenu, MF_POPUP, MF_STRING};

pub struct Menu {
    menu: NonNull<HMENU__>,
}

impl Menu {
    pub(crate) fn as_ptr(&self) -> HMENU {
        self.menu.as_ptr()
    }

    /// Creates a menu. The menu is initially empty, but it can be filled with menu items by using
    /// the InsertMenuItem, AppendMenu, and InsertMenu functions.
    pub fn new() -> Result<Self> {
        let result = unsafe { CreateMenu() };

        if let Some(menu) = NonNull::new(result) {
            Ok(Menu { menu })
        } else {
            Err(Error::last_os_error())
        }
    }

    /// Creates a drop-down menu, submenu, or shortcut menu. The menu is initially empty. You can
    /// insert or append menu items by using the InsertMenuItem function. You can also use the
    /// InsertMenu function to insert menu items and the AppendMenu function to append menu items.
    pub fn new_popup() -> Result<Self> {
        let result = unsafe { CreatePopupMenu() };

        if let Some(menu) = NonNull::new(result) {
            Ok(Menu { menu })
        } else {
            Err(Error::last_os_error())
        }
    }

    /// Appends a new item to the end of the specified menu bar, drop-down menu, submenu, or
    /// shortcut menu. You can use this function to specify the content, appearance, and behavior
    /// of the menu item.
    pub fn append_item(&self, name: &str, value: u16) -> Result<()> {
        let name = CString::new(name)?;
        let result =
            unsafe { AppendMenuA(self.menu.as_ptr(), MF_STRING, value as usize, name.as_ptr()) };

        if result != 0 {
            Ok(())
        } else {
            Err(Error::last_os_error())
        }
    }

    /// Appends a new menu to the end of the specified menu bar, drop-down menu, submenu, or
    /// shortcut menu. You can use this function to specify the content, appearance, and behavior
    /// of the menu item.
    pub fn append_menu(&self, name: &str, value: Menu) -> Result<()> {
        let name = CString::new(name)?;
        let result = unsafe {
            AppendMenuA(
                self.menu.as_ptr(),
                MF_STRING | MF_POPUP,
                value.as_ptr() as usize,
                name.as_ptr(),
            )
        };

        if result != 0 {
            Ok(())
        } else {
            Err(Error::last_os_error())
        }
    }
}
