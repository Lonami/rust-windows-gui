//! Wrappers around the Graphics Device Interface.
//!
//! See also https://docs.microsoft.com/en-us/windows/win32/gdi/windows-gdi.
pub mod bitmap;
pub mod brush;
pub mod canvas;

use winapi::shared::windef::HGDIOBJ;

/// The capability of objects that can be used to paint on a canvas.
pub trait Paint {
    /// Interpret self as a GDI object and return a pointer to self.
    fn as_gdi_obj(&self) -> HGDIOBJ;
}

pub use bitmap::Bitmap;
pub use brush::Brush;
pub use canvas::Canvas;
