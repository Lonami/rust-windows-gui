use crate::{base_instance, non_null_or_err, paint};
use std::ffi::CString;
use std::mem;
use std::ptr::{self, NonNull};
use winapi::shared::minwindef::LPVOID;
use winapi::shared::windef::{HBITMAP, HBITMAP__, HGDIOBJ};
use winapi::um::wingdi::{
    BitBlt, CreateBitmap, DeleteObject, GetObjectA, SetBkColor, BITMAP, CLR_INVALID, RGB, SRCCOPY,
    SRCINVERT,
};
use winapi::um::winnt::{HANDLE, LPCSTR};
use winapi::um::winuser::{
    LoadBitmapA, LoadImageA, IMAGE_BITMAP, LR_LOADFROMFILE, MAKEINTRESOURCEA,
};

pub struct Bitmap {
    bitmap: NonNull<HBITMAP__>,
}

pub struct Info {
    info: BITMAP,
    _size: usize,
}

pub fn new(width: i32, height: i32, planes: u32, bit_count: u32) -> Result<Bitmap, ()> {
    // Documentation claims:
    // > This function can return the following value:
    // > ERROR_INVALID_BITMAP | The calculated size of the bitmap is less than zero.
    //
    // However the code seems nowhere to be found. Instead assert here.
    assert!(width >= 0);
    assert!(height >= 0);
    let result = unsafe { CreateBitmap(width, height, planes, bit_count, ptr::null()) };
    NonNull::new(result)
        .map(|bitmap| Bitmap { bitmap })
        .ok_or(())
}

pub fn load(resource: u16) -> Result<Bitmap, ()> {
    let result = unsafe { LoadBitmapA(base_instance(), MAKEINTRESOURCEA(resource)) };
    NonNull::new(result)
        .map(|bitmap| Bitmap { bitmap })
        .ok_or(())
}

pub fn from_file(path: &str) -> crate::Result<Bitmap> {
    let path = CString::new(path)?;
    let result = unsafe {
        LoadImageA(
            ptr::null_mut(),
            path.as_ptr() as LPCSTR,
            IMAGE_BITMAP,
            0,
            0,
            LR_LOADFROMFILE,
        )
    };
    non_null_or_err(result as HBITMAP).map(|bitmap| Bitmap { bitmap })
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

    // TODO highly-specific?
    // A better abstraction would own objects and turn them into "in-use objects" that can be
    // converted back into the original object after its use is over (creating a new DC).
    pub fn into_mask(
        &self,
        template: &Bitmap,
        size: (i32, i32),
        transparent: (u8, u8, u8),
    ) -> Result<(), ()> {
        let hdc = paint::HDC::new()?;
        let hdc2 = paint::HDC::new()?;

        let _guard = hdc.select_object(template)?;
        let _guard2 = hdc2.select_object(self)?;

        let result = unsafe {
            SetBkColor(
                hdc.as_ptr(),
                RGB(transparent.0, transparent.1, transparent.2),
            )
        };
        if result == CLR_INVALID {
            return Err(());
        }

        let result = unsafe {
            BitBlt(
                hdc2.as_ptr(),
                0,
                0,
                size.0,
                size.1,
                hdc.as_ptr(),
                0,
                0,
                SRCCOPY,
            )
        };
        if result == 0 {
            return Err(());
        }
        let result = unsafe {
            BitBlt(
                hdc.as_ptr(),
                0,
                0,
                size.0,
                size.1,
                hdc2.as_ptr(),
                0,
                0,
                SRCINVERT,
            )
        };
        if result == 0 {
            return Err(());
        }

        Ok(())
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
