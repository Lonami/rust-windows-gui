//! A [`Canvas`] is a device-context. A device-context can either represent something on-screen,
//! such as a window, or be an in-memory buffer (something useful to avoid flickering).
//!
//! A device-context always has an object selected onto it (by default, "none", which in reality
//! is a 1x1 compatible bitmap). An object can only be used if it's currently selected. This means
//! that it is necessary to create multiple device-context if one wishes to work with multiple
//! objects at once. To do this, [`Canvas::try_clone`] can be used.
//!
//! The objects that can be used to paint on a canvas all implement the [`Canvas`] trait.
use super::{brush, Bitmap, Paint};
use crate::{rect, window};

use std::fmt;
use std::marker::PhantomData;
use std::mem;
use std::ptr::{self, NonNull};
use winapi::shared::windef::{HDC__, HGDIOBJ, LPRECT};
use winapi::um::wingdi::{
    BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, GetDeviceCaps, SelectObject,
    SetBkColor, CLR_INVALID, HGDI_ERROR, HORZRES, RGB, SRCAND, SRCCOPY, SRCINVERT, SRCPAINT,
    VERTRES,
};
use winapi::um::winuser::{BeginPaint, EndPaint, FillRect, GetDC, ReleaseDC, PAINTSTRUCT};

#[derive(Debug)]
pub struct Canvas<'w, 'p>
where
    'w: 'p,
{
    hdc: NonNull<HDC__>,
    mode: Mode<'w>,
    selection: Selection<'p>,
}

enum Mode<'w> {
    /// The "default" paint mode, used upon receiving a paint message.
    ///
    /// Obtained via `BeginPaint`. It must be dropped with `EndPaint`.
    Paint {
        window: &'w window::Window<'w>,
        info: PAINTSTRUCT,
    },
    /// Obtained via `GetDC`. It must be dropped with `ReleaseDC`.
    BorrowedDc { window: &'w window::Window<'w> },
    /// Obtained via `CreateCompatibleDC`. It must be dropped with `DeleteDC`.
    OwnedDc,
    /// The data has been moved out somewhere else, so the drop does not need to clean it up.
    Moved,
}

#[derive(Debug)]
enum Selection<'p> {
    Default,
    Custom {
        lifetime: PhantomData<&'p ()>,
        dc_object: HGDIOBJ,
    },
}

pub struct Bitwise<'c, 'w, 'p> {
    canvas: &'c Canvas<'w, 'p>,
    rect: rect::Rect,
    src_x: i32,
    src_y: i32,
}

impl<'w, 'p> Canvas<'w, 'p>
where
    'w: 'p,
{
    // Canvas creation (both borrowed and owned).

    /// Attempt to create a new, owned version of a canvas compatible with the current screen.
    pub fn from_current_screen() -> Result<Self, ()> {
        let result = unsafe { CreateCompatibleDC(ptr::null_mut()) };
        NonNull::new(result).ok_or(()).map(|hdc| Self {
            hdc,
            mode: Mode::OwnedDc,
            selection: Selection::Default,
        })
    }

    /// Attempt to create a new canvas in order to directly paint on the specified window.
    pub fn from_window(window: &'w window::Window) -> Result<Self, ()> {
        let mut info = unsafe { mem::zeroed() };
        let result = unsafe { BeginPaint(window.hwnd_ptr(), &mut info) };
        NonNull::new(result).ok_or(()).map(|hdc| Canvas {
            hdc,
            mode: Mode::Paint { window, info },
            selection: Selection::Default,
        })
    }

    /// Attempt to create a new canvas to act as a buffer with the same settings as the window.
    pub fn from_window_settings(window: &'w window::Window) -> Result<Self, ()> {
        let result = unsafe { GetDC(window.hwnd_ptr()) };
        NonNull::new(result).ok_or(()).map(|hdc| Canvas {
            hdc,
            mode: Mode::BorrowedDc { window },
            selection: Selection::Default,
        })
    }

    /// Attempt to create a new, owned version of this canvas.
    pub fn try_clone(&self) -> Result<Self, ()> {
        let result = unsafe { CreateCompatibleDC(self.hdc.as_ptr()) };
        NonNull::new(result).ok_or(()).map(|hdc| Self {
            hdc,
            mode: Mode::OwnedDc,
            selection: Selection::Default,
        })
    }

    /// Create a new bitmap compatible with this device-context.
    pub fn create_bitmap(&self, width: i32, height: i32) -> Result<Bitmap, ()> {
        let result = unsafe { CreateCompatibleBitmap(self.hdc.as_ptr(), width, height) };
        NonNull::new(result)
            .map(|bitmap| Bitmap { bitmap })
            .ok_or(())
    }

    // Device capabilities.
    //
    // See also https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getdevicecaps.

    /// Width, in pixels, of the device-context.
    pub fn width(&self) -> i32 {
        unsafe { GetDeviceCaps(self.hdc.as_ptr(), HORZRES) }
    }

    /// Height, in raster lines, of the device-context.
    pub fn height(&self) -> i32 {
        unsafe { GetDeviceCaps(self.hdc.as_ptr(), VERTRES) }
    }

    /// Miscellaneous painting operations.

    pub fn fill_rect(&self, rect: rect::Rect, brush: brush::Brush) -> Result<(), ()> {
        let mut rect = rect.0;
        let result = unsafe { FillRect(self.hdc.as_ptr(), &mut rect as LPRECT, brush.as_ptr()) };
        if result != 0 {
            Ok(())
        } else {
            Err(())
        }
    }

    /// Set the background color of the device-context.
    pub fn set_background(&self, (r, g, b): (u8, u8, u8)) -> Result<(), ()> {
        let result = unsafe { SetBkColor(self.hdc.as_ptr(), RGB(r, g, b)) };
        if result == CLR_INVALID {
            Err(())
        } else {
            Ok(())
        }
    }

    /// Bind a different object to the canvas.
    ///
    /// Objects must be bound to the canvas before operations can be performed with them.
    /// If you need to operate with multiple objects at once, you may need a new buffer,
    /// which can be obtained through [`Self::try_clone`].
    ///
    /// There can only be one object bound at a time, hence why this method moves the canvas.
    ///
    /// The default object may be "special", for example, painting to it may render on screen.
    pub fn bind<'q, P>(mut self, object: &'q P) -> Result<Canvas<'w, 'q>, Canvas<'w, 'p>>
    where
        P: Paint,
        'w: 'q,
    {
        let result = unsafe { SelectObject(self.hdc.as_ptr(), object.as_gdi_obj()) };
        if result.is_null() || result == HGDI_ERROR {
            Err(self)
        } else {
            // Make sure to set "moved" data on self so that it's the drop does not affect our new canvas.
            let hdc = self.hdc;
            let mode = mem::replace(&mut self.mode, Mode::Moved);
            let selection = mem::replace(&mut self.selection, Selection::Default);

            // It's a new canvas, as the lifetimes are different (we can't repurpose self to avoid moving).
            Ok(Canvas::<'w, 'q> {
                hdc: hdc,
                mode: mode,
                selection: Selection::Custom {
                    lifetime: PhantomData::<&'q ()>,
                    dc_object: match selection {
                        Selection::Default => result,
                        // When rebinding, we must still keep the original DC object. We can safely
                        // ignore the one SelectObject returned as it's the previous selection which
                        // the caller already has access to.
                        Selection::Custom { dc_object, .. } => dc_object,
                    },
                },
            })
        }
    }

    /// Bind back the default object.
    ///
    /// Does nothing if the default object was already bound.
    pub fn bind_default(mut self) -> Result<Canvas<'w, 'static>, Canvas<'w, 'p>> {
        match self.selection {
            Selection::Default => {}
            Selection::Custom { dc_object, .. } => {
                let result = unsafe { SelectObject(self.hdc.as_ptr(), dc_object) };
                if result.is_null() || result == HGDI_ERROR {
                    return Err(self);
                }
                // self will be dropped and we don't want it to re-select custom back.
                self.selection = Selection::Default;
            }
        }

        Ok(Canvas::<'w, 'static> {
            hdc: self.hdc,
            // self will be dropped so make sure the DC we now use isn't deleted.
            mode: mem::replace(&mut self.mode, Mode::Moved),
            selection: Selection::<'static>::Default,
        })
    }

    // Bit-block transfer operations.
    //
    // See also https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-bitblt.

    /// Begin a bitwise operation, which can be further configured before executing.
    pub fn bitwise<'c>(&'c self) -> Bitwise<'c, 'w, 'p> {
        Bitwise {
            canvas: self,
            rect: rect::Rect::new(self.width(), self.height()),
            src_x: 0,
            src_y: 0,
        }
    }
}

impl<'c, 'w, 'p> Bitwise<'c, 'w, 'p> {
    /// Set the rectangular region where the bitwise operation will be applied.
    pub fn region(mut self, rect: rect::Rect) -> Self {
        self.rect = rect;
        self
    }

    /// Offset the source of the operation by this amount before applying it.
    pub fn offset_source(mut self, x: i32, y: i32) -> Self {
        self.src_x = x;
        self.src_y = y;
        self
    }

    /// Apply the bitwise AND operation of the given source canvas into self (the destination).
    pub fn and(self, source: &Canvas) -> Result<(), ()> {
        self.transfer(source, SRCAND)
    }

    /// Apply the bitwise OR operation of the given source canvas into self (the destination).
    pub fn or(self, source: &Canvas) -> Result<(), ()> {
        self.transfer(source, SRCPAINT)
    }

    /// Apply the bitwise XOR operation of the given source canvas into self (the destination).
    pub fn xor(self, source: &Canvas) -> Result<(), ()> {
        self.transfer(source, SRCINVERT)
    }

    /// Apply the bitwise SET operation of the given source canvas into self (the destination).
    pub fn set(self, source: &Canvas) -> Result<(), ()> {
        self.transfer(source, SRCCOPY)
    }

    fn transfer(self, source: &Canvas, raster_op: u32) -> Result<(), ()> {
        let result = unsafe {
            BitBlt(
                self.canvas.hdc.as_ptr(),
                self.rect.x(),
                self.rect.y(),
                self.rect.width(),
                self.rect.height(),
                source.hdc.as_ptr(),
                self.src_x,
                self.src_y,
                raster_op,
            )
        };

        if result != 0 {
            Ok(())
        } else {
            Err(())
        }
    }
}

impl Drop for Canvas<'_, '_> {
    fn drop(&mut self) {
        match self.selection {
            Selection::Default => {}
            Selection::Custom { dc_object, .. } => {
                let result = unsafe { SelectObject(self.hdc.as_ptr(), dc_object) };
                if result.is_null() || result == HGDI_ERROR {
                    panic!("failed to return selected object");
                }
            }
        }

        let result = match &mut self.mode {
            Mode::Paint { window, info } => unsafe { EndPaint(window.hwnd_ptr(), info) },
            Mode::BorrowedDc { window } => unsafe {
                ReleaseDC(window.hwnd_ptr(), self.hdc.as_ptr())
            },
            Mode::OwnedDc => unsafe { DeleteDC(self.hdc.as_ptr()) },
            Mode::Moved => 1,
        };

        if result == 0 {
            panic!(
                "failed to drop {} canvas",
                match self.mode {
                    Mode::Paint { .. } => "painting",
                    Mode::BorrowedDc { .. } => "borrowed",
                    Mode::OwnedDc => "owned",
                    Mode::Moved => "moved",
                }
            )
        }
    }
}

impl fmt::Debug for Mode<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Paint { window, .. } => f
                .debug_struct("Paint")
                .field("window", window)
                .finish_non_exhaustive(),
            Self::BorrowedDc { window } => f
                .debug_struct("BorrowedDc")
                .field("window", window)
                .finish(),
            Self::OwnedDc => f.debug_struct("OwnedDc").finish(),
            Self::Moved => f.debug_struct("Moved").finish(),
        }
    }
}
