use winapi::um::commctrl::{
    STD_COPY, STD_CUT, STD_DELETE, STD_FILENEW, STD_FILEOPEN, STD_FILESAVE, STD_FIND, STD_HELP,
    STD_PASTE, STD_PRINT, STD_PRINTPRE, STD_PROPERTIES, STD_REDOW, STD_REPLACE, STD_UNDO, TBBUTTON,
    TBSTATE_ENABLED, TBSTYLE_BUTTON,
};

// For IDB_STD_SMALL_COLOR.
// https://docs.microsoft.com/en-us/windows/win32/controls/toolbar-standard-button-image-index-values
#[repr(i32)]
pub enum Icon {
    /// Copy operation.
    Copy = STD_COPY,

    /// Cut operation.
    Cut = STD_CUT,

    /// Delete operation.
    Delete = STD_DELETE,

    /// New file operation.
    FileNew = STD_FILENEW,

    /// Open file operation.
    FileOpen = STD_FILEOPEN,

    /// Save file operation.
    FileSave = STD_FILESAVE,

    /// Find operation.
    Find = STD_FIND,

    /// Help operation.
    Help = STD_HELP,

    /// Paste operation.
    Paste = STD_PASTE,

    /// Print operation.
    Print = STD_PRINT,

    /// Print preview operation.
    PrintPre = STD_PRINTPRE,

    /// Properties operation.
    Properties = STD_PROPERTIES,

    /// Redo operation.
    Redo = STD_REDOW,

    /// Replace operation.
    Replace = STD_REPLACE,

    /// Undo operation.
    Undo = STD_UNDO,
}

#[repr(transparent)]
pub struct Button {
    _data: TBBUTTON,
}

impl Button {
    pub fn new(id: u16, icon: Icon) -> Self {
        Button {
            _data: TBBUTTON {
                iBitmap: icon as i32,
                idCommand: id as i32,
                fsState: TBSTATE_ENABLED,
                fsStyle: TBSTYLE_BUTTON as u8,
                bReserved: [0; 6],
                dwData: 0,
                iString: 0,
            },
        }
    }
}
