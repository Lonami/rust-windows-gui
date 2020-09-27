use winapi::shared::windef::HFONT;
use winapi::um::wingdi::{GetStockObject, DEFAULT_GUI_FONT};

pub struct Font {
    font: HFONT,
}

pub fn get_default() -> Result<Font, ()> {
    let result = unsafe { GetStockObject(DEFAULT_GUI_FONT as i32) };
    if result.is_null() {
        Err(())
    } else {
        Ok(Font {
            font: result as HFONT,
        })
    }
}

impl Font {
    pub(crate) fn as_ptr(&self) -> HFONT {
        self.font
    }
}
