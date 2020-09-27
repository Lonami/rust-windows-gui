use crate::{base_instance, window, DialogCallback, Error, Result};
use std::ffi::CString;
use std::ptr;
use winapi::shared::minwindef::MAX_PATH;
use winapi::um::commdlg::{
    GetOpenFileNameA, GetSaveFileNameA, LPOPENFILENAMEA, OFN_ALLOWMULTISELECT, OFN_CREATEPROMPT,
    OFN_DONTADDTORECENT, OFN_ENABLEHOOK, OFN_ENABLEINCLUDENOTIFY, OFN_ENABLESIZING,
    OFN_ENABLETEMPLATE, OFN_ENABLETEMPLATEHANDLE, OFN_EXPLORER, OFN_EXTENSIONDIFFERENT,
    OFN_FILEMUSTEXIST, OFN_FORCESHOWHIDDEN, OFN_HIDEREADONLY, OFN_LONGNAMES, OFN_NOCHANGEDIR,
    OFN_NODEREFERENCELINKS, OFN_NOLONGNAMES, OFN_NONETWORKBUTTON, OFN_NOREADONLYRETURN,
    OFN_NOTESTFILECREATE, OFN_NOVALIDATE, OFN_OVERWRITEPROMPT, OFN_PATHMUSTEXIST, OFN_READONLY,
    OFN_SHAREAWARE, OFN_SHOWHELP, OPENFILENAMEA,
};
use winapi::um::winnt::{LPCSTR, LPSTR};
use winapi::um::winuser::{DialogBoxParamA, MAKEINTRESOURCEA};

/// Creates a modal dialog box from a dialog box template resource. The function does not
/// return control until the specified callback function terminates the modal dialog box
/// by calling the `Window::end_dialog` function.
pub fn show(resource: u16, callback: DialogCallback) -> Result<isize> {
    let hinstance = base_instance();
    let resource = MAKEINTRESOURCEA(resource);

    // Can't know what the dialog's handle is beforehand. The special value 0 will be
    // replaced with the right value as soon as the init dialog message arrives.
    crate::HWND_TO_DLG_CALLBACK
        .lock()
        .unwrap()
        .insert(0, callback);

    let result = unsafe {
        DialogBoxParamA(
            hinstance,
            resource,
            ptr::null_mut(),
            Some(window::dlg_proc_wrapper),
            0,
        )
    };

    // In the code at http://winprog.org/tutorial/dlgfaq.html, DialogBox returns 0 as well,
    // which according to the official documentation "If the function fails because the
    // hWndParent parameter is invalid, the return value is zero. The function returns zero
    // in this case for compatibility with previous versions of Windows.".
    // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-dialogboxparama
    //
    // It seems safe to ignore 0 as there being an error.
    match result {
        -1 => Err(Error::last_os_error()),
        n => Ok(n),
    }
}

#[repr(u32)]
pub enum OpenFileConfig {
    /// The File Name list box allows multiple selections. If you also set the `Explorer` flag,
    /// the dialog box uses the Explorer-style user interface; otherwise, it uses the old-style
    /// user interface.
    AllowMultiSelect = OFN_ALLOWMULTISELECT,

    /// If the user specifies a file that does not exist, this flag causes the dialog box to
    /// prompt the user for permission to create the file. If the user chooses to create the
    /// file, the dialog box closes and the function returns the specified name; otherwise,
    /// the dialog box remains open. If you use this flag with the `AllowMultiSelect` flag, the
    /// dialog box allows the user to specify only one nonexistent file.
    CreatePrompt = OFN_CREATEPROMPT,

    /// Prevents the system from adding a link to the selected file in the file system directory
    /// that contains the user's most recently used documents.
    DontAddToRecent = OFN_DONTADDTORECENT,

    /// Enables the hook function specified in the hook member.
    EnableHook = OFN_ENABLEHOOK,

    /// Causes the dialog box to send `IncludeItem` notification messages to your hook procedure
    /// when the user opens a folder. The dialog box sends a notification for each item in the
    /// newly opened folder. These messages enable you to control which items the dialog box
    /// displays in the folder's item list.
    EnableIncludeNotify = OFN_ENABLEINCLUDENOTIFY,

    /// Enables the Explorer-style dialog box to be resized using either the mouse or the
    /// keyboard. By default, the Explorer-style Open and Save As dialog boxes allow the dialog
    /// box to be resized regardless of whether this flag is set. This flag is necessary only if
    /// you provide a hook procedure or custom template. The old-style dialog box does not permit
    /// resizing.
    EnableSizing = OFN_ENABLESIZING,

    /// The template member is a pointer to the name of a dialog template resource in the module
    /// identified by the instance member. If the `Explorer` flag is set, the system uses the
    /// specified template to create a dialog box that is a child of the default Explorer-style
    /// dialog box. If the `Explorer` flag is not set, the system uses the template to create
    /// an old-style dialog box that replaces the default dialog box.
    EnableTemplate = OFN_ENABLETEMPLATE,

    /// The instance member identifies a data block that contains a preloaded dialog box template.
    /// The system ignores template name if this flag is specified. If the `Explorer` flag is set,
    /// the system uses the specified template to create a dialog box that is a child of the
    /// default Explorer-style dialog box. If the `Explorer` flag is not set, the system uses the
    /// template to create an old-style dialog box that replaces the default dialog box.
    EnableTemplateHandle = OFN_ENABLETEMPLATEHANDLE,

    /// Indicates that any customizations made to the Open or Save As dialog box use the
    /// Explorer-style customization methods. For more information, see Explorer-Style Hook
    /// Procedures and Explorer-Style Custom Templates. By default, the Open and Save As dialog
    /// boxes use the Explorer-style user interface regardless of whether this flag is set. This
    /// flag is necessary only if you provide a hook procedure or custom template, or set the
    /// `AllowMultiSelect` flag. If you want the old-style user interface, omit the `Explorer`
    /// flag and provide a replacement old-style template or hook procedure. If you want the old
    /// style but do not need a custom template or hook procedure, simply provide a hook procedure
    /// that always returns `false`.
    Explorer = OFN_EXPLORER,

    /// The user typed a file name extension that differs from the extension specified by
    /// `set_default_ext`. The function does not use this flag if no default extension was set.
    ExtensionDifferent = OFN_EXTENSIONDIFFERENT,

    /// The user can type only names of existing files in the File Name entry field. If this
    /// flag is specified and the user enters an invalid name, the dialog box procedure displays
    /// a warning in a message box. If this flag is specified, the `PathMustExist` flag is also
    /// used. This flag can be used in an Open dialog box. It cannot be used with a Save As dialog
    /// box.
    FileMustExist = OFN_FILEMUSTEXIST,

    /// Forces the showing of system and hidden files, thus overriding the user setting to show or
    /// not show hidden files. However, a file that is marked both system and hidden is not shown.
    ForceShowHidden = OFN_FORCESHOWHIDDEN,

    /// Hides the Read Only check box.
    HideReadonly = OFN_HIDEREADONLY,

    /// For old-style dialog boxes, this flag causes the dialog box to use long file names. If
    /// this flag is not specified, or if the `AllowMultiSelect` flag is also set, old-style
    /// dialog boxes use short file names (8.3 format) for file names with spaces.
    /// Explorer-style dialog boxes ignore this flag and always display long file names.
    LongNames = OFN_LONGNAMES,

    /// Restores the current directory to its original value if the user changed the directory
    /// while searching for files. This flag is ineffective for GetOpenFileName.
    NoChangeDir = OFN_NOCHANGEDIR,

    /// Directs the dialog box to return the path and file name of the selected shortcut (.LNK)
    /// file. If this value is not specified, the dialog box returns the path and file name of the
    /// file referenced by the shortcut.
    NoDereferenceLinks = OFN_NODEREFERENCELINKS,

    /// For old-style dialog boxes, this flag causes the dialog box to use short file names (8.3
    /// format). Explorer-style dialog boxes ignore this flag and always display long file names.
    NoLongNames = OFN_NOLONGNAMES,

    /// Hides and disables the Network button.
    NoNetworkButton = OFN_NONETWORKBUTTON,

    /// The returned file does not have the Read Only check box selected and is not in a
    /// write-protected directory.
    NoReadonlyReturn = OFN_NOREADONLYRETURN,

    /// The file is not created before the dialog box is closed. This flag should be specified if
    /// the application saves the file on a create-nonmodify network share. When an application
    /// specifies this flag, the library does not check for write protection, a full disk, an
    /// open drive door, or network protection. Applications using this flag must perform file
    /// operations carefully, because a file cannot be reopened once it is closed.
    NoTestFileCreate = OFN_NOTESTFILECREATE,

    /// The common dialog boxes allow invalid characters in the returned file name. Typically,
    /// the calling application uses a hook procedure that checks the file name by using the
    /// `FileOkString` message. If the text box in the edit control is empty or contains nothing
    /// but spaces, the lists of files and directories are updated. If the text box in the edit
    /// control contains anything else, nFileOffset and nFileExtension are set to values
    /// generated by parsing the text. No default extension is added to the text, nor is text
    /// copied to the buffer specified by file title. If the value specified by file offset is
    /// less than zero, the file name is invalid. Otherwise, the file name is valid, and
    /// file extension and file offset can be used as if the `NoValidate` flag had not been
    /// specified.
    NoValidate = OFN_NOVALIDATE,

    /// Causes the Save As dialog box to generate a message box if the selected file already
    /// exists. The user must confirm whether to overwrite the file.
    OverwritePrompt = OFN_OVERWRITEPROMPT,

    /// The user can type only valid paths and file names. If this flag is used and the user types
    /// an invalid path and file name in the File Name entry field, the dialog box function
    /// displays a warning in a message box.
    PathMustExist = OFN_PATHMUSTEXIST,

    /// Causes the Read Only check box to be selected initially when the dialog box is created.
    /// This flag indicates the state of the Read Only check box when the dialog box is closed.
    Readonly = OFN_READONLY,

    /// Specifies that if a call to the OpenFile function fails because of a network sharing
    /// violation, the error is ignored and the dialog box returns the selected file name. If
    /// this flag is not set, the dialog box notifies your hook procedure when a network sharing
    /// violation occurs for the file name specified by the user. If you set the `Explorer` flag,
    /// the dialog box sends the CDN_SHAREVIOLATION message to the hook procedure. If you do not
    /// set `Explorer`, the dialog box sends the `ShareViString` registered message to the hook
    /// procedure.
    Shareware = OFN_SHAREAWARE,

    /// Causes the dialog box to display the Help button. The hwndOwner member must specify the
    /// window to receive the `HelpMsgString` registered messages that the dialog box sends when
    /// the user clicks the Help button. An Explorer-style dialog box sends a `Help` notification
    /// message to your hook procedure when the user clicks the Help button.
    ShowHelp = OFN_SHOWHELP,
}

pub struct OpenFileBuilder<'a> {
    owner: &'a window::Window<'a>,
    filter: Vec<u8>,
    file: Vec<u8>,
    flags: u32,
    default_ext: Option<CString>,
}

impl<'a> OpenFileBuilder<'a> {
    pub(crate) fn new(owner: &'a window::Window<'a>) -> Self {
        Self {
            owner,
            filter: vec![],
            file: vec![0; MAX_PATH],
            flags: 0,
            default_ext: None,
        }
    }

    /// The first string in each pair is a display string that describes the filter (for example,
    /// "Text Files"), and the second string specifies the filter pattern (for example, ".TXT").
    ///
    /// To specify multiple filter patterns for a single display string, use a semicolon to
    /// separate the patterns (for example, ".TXT;.DOC;.BAK"). A pattern string can be a
    /// combination of valid file name characters and the asterisk (*) wildcard character.
    ///
    /// Do not include spaces in the pattern string.
    pub fn set_filters(mut self, filters: &[(&str, &str)]) -> Self {
        self.filter.clear();
        for &(display, pattern) in filters.into_iter() {
            self.filter.extend(display.as_bytes().iter().copied());
            self.filter.push(0u8);
            self.filter.extend(pattern.as_bytes().iter().copied());
            self.filter.push(0u8);
        }
        self.filter.push(0u8);
        self
    }

    /// The default extension. `ask_open_path` and `ask_save_path` append this extension to the
    /// file name if the user fails to type an extension. This string can be any length, but only
    /// the first three characters are appended. The string should not contain a period (.). If
    /// this member is NULL and the user fails to type an extension, no extension is appended.
    pub fn set_default_ext(mut self, extension: &str) -> Self {
        self.default_ext = Some(CString::new(extension).unwrap());
        self
    }

    /// Adds additional configuration options.
    pub fn add_config(mut self, config: OpenFileConfig) -> Self {
        self.flags |= config as u32;
        self
    }

    fn structure(&mut self) -> OPENFILENAMEA {
        OPENFILENAMEA {
            lStructSize: std::mem::size_of::<OPENFILENAMEA>() as u32,
            hwndOwner: self.owner.hwnd_ptr(),
            hInstance: ptr::null_mut(),
            lpstrFilter: self.filter.as_ptr() as LPCSTR,
            lpstrCustomFilter: ptr::null_mut(),
            nMaxCustFilter: 0,
            nFilterIndex: 0,
            lpstrFile: self.file.as_mut_ptr() as LPSTR,
            nMaxFile: MAX_PATH as u32,
            lpstrFileTitle: ptr::null_mut(),
            nMaxFileTitle: 0,
            lpstrInitialDir: ptr::null(),
            lpstrTitle: ptr::null(),
            Flags: self.flags,
            nFileOffset: 0,
            nFileExtension: 0,
            lpstrDefExt: self
                .default_ext
                .as_ref()
                .map(|e| e.as_ptr())
                .unwrap_or_else(ptr::null),
            lCustData: 0,
            lpfnHook: None,
            lpTemplateName: ptr::null(),
            pvReserved: ptr::null_mut(),
            dwReserved: 0,
            FlagsEx: 0,
        }
    }

    /// Creates an Open dialog box that lets the user specify the drive, directory, and the name
    /// of a file or set of files to be opened.
    pub fn ask_open_path(mut self) -> Option<String> {
        // https://docs.microsoft.com/en-us/windows/win32/api/commdlg/nf-commdlg-getopenfilenamea
        let mut buffer = self.structure();
        let result = unsafe { GetOpenFileNameA(&mut buffer as LPOPENFILENAMEA) };
        if result != 0 {
            self.file
                .truncate(self.file.iter().position(|&c| c == 0).unwrap_or(0));
            Some(CString::new(self.file).unwrap().into_string().unwrap())
        } else {
            None
        }
    }

    /// Creates a Save dialog box that lets the user specify the drive, directory, and name of a
    /// file to save.
    pub fn ask_save_path(mut self) -> Option<String> {
        // https://docs.microsoft.com/en-us/windows/win32/api/commdlg/nf-commdlg-getsavefilenamea
        let mut buffer = self.structure();
        let result = unsafe { GetSaveFileNameA(&mut buffer as LPOPENFILENAMEA) };
        if result != 0 {
            self.file
                .truncate(self.file.iter().position(|&c| c == 0).unwrap_or(0));
            Some(CString::new(self.file).unwrap().into_string().unwrap())
        } else {
            None
        }
    }
}
