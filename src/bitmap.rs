use crate::base_instance;
use std::mem;
use std::ptr::NonNull;
use winapi::shared::minwindef::LPVOID;
use winapi::shared::windef::{HBITMAP__, HGDIOBJ};
use winapi::um::wingdi::{DeleteObject, GetObjectA, BITMAP};
use winapi::um::winnt::HANDLE;
use winapi::um::winuser::{LoadBitmapA, MAKEINTRESOURCEA};

pub struct Bitmap {
    bitmap: NonNull<HBITMAP__>,
}

pub struct Info {
    info: BITMAP,
    _size: usize,
}

pub fn load(resource: u16) -> Result<Bitmap, ()> {
    let result = unsafe { LoadBitmapA(base_instance(), MAKEINTRESOURCEA(resource)) };
    NonNull::new(result)
        .map(|bitmap| Bitmap { bitmap })
        .ok_or(())
}

impl Bitmap {
    /// Retrieves information for the specified graphics object.
    pub fn info(&self) -> Result<Info, ()> {
        let mut info: BITMAP = unsafe { mem::zeroed() };
        let result = unsafe {
            GetObjectA(
                self.bitmap.as_ptr() as HANDLE,
                mem::size_of::<BITMAP>() as i32,
                &mut info as *mut BITMAP as LPVOID,
            )
        };
        if result != 0 {
            Ok(Info {
                info,
                _size: result as usize,
            })
        } else {
            Err(())
        }
    }

    pub(crate) fn as_gdi_obj(&self) -> HGDIOBJ {
        self.bitmap.as_ptr() as HGDIOBJ
    }
}

impl Info {
    /// The width, in pixels, of the bitmap. The width is greater than zero.
    pub fn width(&self) -> i32 {
        self.info.bmWidth
    }

    /// The height, in pixels, of the bitmap. The height is greater than zero.
    pub fn height(&self) -> i32 {
        self.info.bmHeight
    }

    /// The number of bytes in each scan line. This value is divisible by 2, because the system
    /// assumes that the bit values of a bitmap form an array that is word aligned.
    pub fn stride(&self) -> i32 {
        self.info.bmWidthBytes
    }

    /// The count of color planes.
    pub fn planes(&self) -> u16 {
        self.info.bmPlanes
    }

    /// The number of bits required to indicate the color of a pixel.
    pub fn bits_per_pixel(&self) -> u16 {
        self.info.bmBitsPixel
    }
}

impl Drop for Bitmap {
    fn drop(&mut self) {
        let result = unsafe { DeleteObject(self.bitmap.as_ptr() as HGDIOBJ) };
        if result == 0 {
            panic!("failed to delete bitmap, it might still be in use");
        }
    }
}
