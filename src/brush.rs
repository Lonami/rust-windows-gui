use std::ptr::NonNull;
use winapi::shared::windef::{HBRUSH__, HGDIOBJ};
use winapi::um::wingdi::{CreateSolidBrush, DeleteObject, RGB};

pub struct Brush {
    brush: NonNull<HBRUSH__>,
}

impl Brush {
    pub fn new_solid_rgb(r: u8, g: u8, b: u8) -> Option<Self> {
        let result = unsafe { CreateSolidBrush(RGB(r, g, b)) };
        NonNull::new(result).map(|brush| Brush { brush })
    }

    /// Value to be returned from a dialog procedure to use this brush.
    pub fn dlg_proc_value(&self) -> isize {
        self.brush.as_ptr() as isize
    }
}

impl Drop for Brush {
    fn drop(&mut self) {
        let result = unsafe { DeleteObject(self.brush.as_ptr() as HGDIOBJ) };
        if result == 0 {
            panic!("invalid handle or still selected into a DC");
        }
    }
}
