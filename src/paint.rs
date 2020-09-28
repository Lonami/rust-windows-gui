use crate::{bitmap, window};
use std::mem;
use std::ptr::NonNull;
use winapi::shared::windef::{HDC__, HGDIOBJ};
use winapi::um::wingdi::{BitBlt, CreateCompatibleDC, DeleteDC, SelectObject, HGDI_ERROR, SRCCOPY};
use winapi::um::winuser::{BeginPaint, EndPaint, PAINTSTRUCT};

pub struct Paint<'a> {
    window: &'a window::Window<'a>,
    paint: PAINTSTRUCT,
    hdc: NonNull<HDC__>,
}

struct HDC {
    hdc: NonNull<HDC__>,
}

struct SwappedObject<'a> {
    hdc: &'a HDC,
    old_handle: HGDIOBJ,
}

impl<'a> Paint<'a> {
    pub(crate) fn new(window: &'a window::Window) -> Result<Self, ()> {
        let mut paint = unsafe { mem::zeroed() };
        let result = unsafe { BeginPaint(window.hwnd_ptr(), &mut paint) };
        NonNull::new(result)
            .ok_or(())
            .map(|hdc| Paint { window, paint, hdc })
    }

    pub fn copy_bitmap_to_rect(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        bmp: &bitmap::Bitmap,
        src_x: i32,
        src_y: i32,
    ) -> Result<(), ()> {
        // Any function is fallible so RAII is used a lot to ensure objects are freed.
        let hdc_mem = HDC::new_compatible_dc(self.hdc)?;

        // Hold onto this object while the operaiton on the temporary DC is made.
        let _old_bmp = hdc_mem.select_object(bmp)?;
        let result = unsafe {
            BitBlt(
                self.hdc.as_ptr(),
                x,
                y,
                width,
                height,
                hdc_mem.hdc.as_ptr(),
                src_x,
                src_y,
                SRCCOPY,
            )
        };

        if result != 0 {
            Ok(())
        } else {
            // Can get last OS error here, but HDC don't provide one...
            Err(())
        }
    }
}

impl HDC {
    fn new_compatible_dc(hdc: NonNull<HDC__>) -> Result<Self, ()> {
        let result = unsafe { CreateCompatibleDC(hdc.as_ptr()) };
        NonNull::new(result).ok_or(()).map(|dc| HDC { hdc: dc })
    }

    /// Selects an object into the specified device context (DC).
    /// The new object replaces the previous object of the same type.
    fn select_object<'a, 'b>(&'a self, bmp: &'b bitmap::Bitmap) -> Result<SwappedObject<'a>, ()> {
        let result = unsafe { SelectObject(self.hdc.as_ptr(), bmp.as_gdi_obj()) };
        if result.is_null() || result == HGDI_ERROR {
            Err(())
        } else {
            Ok(SwappedObject {
                hdc: self,
                old_handle: result,
            })
        }
    }
}

impl Drop for Paint<'_> {
    fn drop(&mut self) {
        let _result = unsafe { EndPaint(self.window.hwnd_ptr(), &mut self.paint) };
    }
}

impl Drop for HDC {
    fn drop(&mut self) {
        let result = unsafe { DeleteDC(self.hdc.as_ptr()) };
        if result == 0 {
            panic!("failed to delete dc");
        }
    }
}

impl Drop for SwappedObject<'_> {
    fn drop(&mut self) {
        let result = unsafe { SelectObject(self.hdc.hdc.as_ptr(), self.old_handle) };
        if result.is_null() || result == HGDI_ERROR {
            panic!("failed to delete selected object");
        }
    }
}
