use std::ptr::NonNull;
use winapi::shared::windef::{HBRUSH, HBRUSH__, HGDIOBJ};
use winapi::um::wingdi::{
    CreateSolidBrush, DeleteObject, GetStockObject, LTGRAY_BRUSH, RGB, WHITE_BRUSH,
};

#[derive(Debug)]
pub struct Brush {
    brush: NonNull<HBRUSH__>,
    stock: bool,
}

impl Brush {
    pub fn new_solid_rgb(r: u8, g: u8, b: u8) -> Option<Self> {
        let result = unsafe { CreateSolidBrush(RGB(r, g, b)) };
        NonNull::new(result).map(|brush| Brush {
            brush,
            stock: false,
        })
    }

    /// Value to be returned from a dialog procedure to use this brush.
    pub fn dlg_proc_value(&self) -> isize {
        self.as_ptr() as isize
    }

    pub(crate) fn as_ptr(&self) -> HBRUSH {
        self.brush.as_ptr()
    }
}

pub fn white() -> Result<Brush, ()> {
    let result = unsafe { GetStockObject(WHITE_BRUSH as i32) };
    NonNull::new(result as HBRUSH)
        .ok_or(())
        .map(|brush| Brush { brush, stock: true })
}

pub fn light_gray() -> Result<Brush, ()> {
    let result = unsafe { GetStockObject(LTGRAY_BRUSH as i32) };
    NonNull::new(result as HBRUSH)
        .ok_or(())
        .map(|brush| Brush { brush, stock: true })
}

impl Drop for Brush {
    fn drop(&mut self) {
        if self.stock {
            return;
        }

        let result = unsafe { DeleteObject(self.brush.as_ptr() as HGDIOBJ) };
        if result == 0 {
            panic!("invalid handle or still selected into a DC");
        }
    }
}
