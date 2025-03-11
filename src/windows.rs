use super::*;
use std::ffi::{OsStr, OsString};
use std::iter::once;
use std::mem;
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::path::Path;
use std::ptr;

#[allow(non_snake_case)]
#[repr(C)]
struct OPENFILENAMEW {
    lStructSize: u32,
    hwndOwner: *mut std::ffi::c_void,
    hInstance: *mut std::ffi::c_void,
    lpstrFilter: *const u16,
    lpstrCustomFilter: *mut u16,
    nMaxCustFilter: u32,
    nFilterIndex: u32,
    lpstrFile: *mut u16,
    nMaxFile: u32,
    lpstrFileTitle: *mut u16,
    nMaxFileTitle: u32,
    lpstrInitialDir: *const u16,
    lpstrTitle: *const u16,
    Flags: u32,
    nFileOffset: u16,
    nFileExtension: u16,
    lpstrDefExt: *const u16,
    lCustData: usize,
    lpfnHook: *const std::ffi::c_void,
    lpTemplateName: *const u16,
    pvReserved: *mut std::ffi::c_void,
    dwReserved: u32,
    FlagsEx: u32,
}

#[allow(non_snake_case)]
#[repr(C)]
struct BROWSEINFOW {
    hwndOwner: *mut std::ffi::c_void,
    pidlRoot: *mut std::ffi::c_void,
    pszDisplayName: *mut u16,
    lpszTitle: *const u16,
    ulFlags: u32,
    lpfn: *const std::ffi::c_void,
    lParam: isize,
    iImage: i32,
}

#[allow(non_snake_case)]
#[repr(C)]
struct CHOOSECOLORW {
    lStructSize: u32,
    hwndOwner: *mut std::ffi::c_void,
    hInstance: *mut std::ffi::c_void,
    rgbResult: u32,
    lpCustColors: *mut u32,
    Flags: u32,
    lCustData: usize,
    lpfnHook: *const std::ffi::c_void,
    lpTemplateName: *const u16,
}

// Windows notifications structs
#[allow(non_snake_case)]
#[repr(C)]
struct NOTIFYICONDATAW {
    cbSize: u32,
    hWnd: *mut std::ffi::c_void,
    uID: u32,
    uFlags: u32,
    uCallbackMessage: u32,
    hIcon: *mut std::ffi::c_void,
    szTip: [u16; 128],
    dwState: u32,
    dwStateMask: u32,
    szInfo: [u16; 256],
    uVersion: u32,
    szInfoTitle: [u16; 64],
    dwInfoFlags: u32,
    guidItem: [u8; 16], // GUID
    hBalloonIcon: *mut std::ffi::c_void,
}

type HWND = *mut std::ffi::c_void;
type HINSTANCE = *mut std::ffi::c_void;
type LPARAM = isize;
type PIDLIST_ABSOLUTE = *mut std::ffi::c_void;
type HICON = *mut std::ffi::c_void;

const MB_OK: u32 = 0x00000000;
const MB_OKCANCEL: u32 = 0x00000001;
const MB_YESNO: u32 = 0x00000004;
const MB_YESNOCANCEL: u32 = 0x00000003;
const MB_ICONINFORMATION: u32 = 0x00000040;
const MB_ICONWARNING: u32 = 0x00000030;
const MB_ICONERROR: u32 = 0x00000010;
const MB_ICONQUESTION: u32 = 0x00000020;
const MB_DEFBUTTON1: u32 = 0x00000000;
const MB_DEFBUTTON2: u32 = 0x00000100;
const MB_DEFBUTTON3: u32 = 0x00000200;

const OFN_OVERWRITEPROMPT: u32 = 0x00000002;
const OFN_FILEMUSTEXIST: u32 = 0x00001000;
const OFN_PATHMUSTEXIST: u32 = 0x00000800;
const OFN_ALLOWMULTISELECT: u32 = 0x00000200;
const OFN_EXPLORER: u32 = 0x00080000;
const OFN_NOCHANGEDIR: u32 = 0x00000008;

const BIF_RETURNONLYFSDIRS: u32 = 0x00000001;
const BIF_NEWDIALOGSTYLE: u32 = 0x00000040;

const CC_RGBINIT: u32 = 0x00000001;
const CC_FULLOPEN: u32 = 0x00000002;
const CC_ANYCOLOR: u32 = 0x00000100;

// Windows notification constants
const NIM_ADD: u32 = 0x00000000;
const NIM_MODIFY: u32 = 0x00000001;
const NIM_DELETE: u32 = 0x00000002;
const NIF_INFO: u32 = 0x00000010;
const NIIF_INFO: u32 = 0x00000001;
const NIIF_WARNING: u32 = 0x00000002;
const NIIF_ERROR: u32 = 0x00000003;

const IDOK: i32 = 1;
const IDCANCEL: i32 = 2;
const IDYES: i32 = 6;
const IDNO: i32 = 7;

extern "system" {
    fn MessageBoxW(hwnd: HWND, text: *const u16, caption: *const u16, utype: u32) -> i32;
    fn GetOpenFileNameW(lpofn: *mut OPENFILENAMEW) -> i32;
    fn GetSaveFileNameW(lpofn: *mut OPENFILENAMEW) -> i32;
    fn SHBrowseForFolderW(lpbi: *mut BROWSEINFOW) -> PIDLIST_ABSOLUTE;
    fn SHGetPathFromIDListW(pidl: PIDLIST_ABSOLUTE, pszPath: *mut u16) -> i32;
    fn ChooseColorW(lpcc: *mut CHOOSECOLORW) -> i32;
    fn CoTaskMemFree(pv: *mut std::ffi::c_void);
    fn LoadIconW(hInstance: HINSTANCE, lpIconName: *const u16) -> HICON;
    fn Shell_NotifyIconW(dwMessage: u32, lpdata: *mut NOTIFYICONDATAW) -> i32;
}

fn to_wstring(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(once(0)).collect()
}

fn from_wstring(s: &[u16]) -> String {
    let len = s.iter().position(|&c| c == 0).unwrap_or(s.len());
    let os_string = OsString::from_wide(&s[..len]);
    os_string.to_string_lossy().into_owned()
}

pub fn message_box_ok(msg_box: &MessageBox) {
    let title = msg_box.dialog.title();
    let message = msg_box.dialog.message();
    let icon = msg_box.icon();

    let w_title = to_wstring(title);
    let w_message = to_wstring(message);

    let icon_flag = match icon {
        MessageBoxIcon::Info => MB_ICONINFORMATION,
        MessageBoxIcon::Warning => MB_ICONWARNING,
        MessageBoxIcon::Error => MB_ICONERROR,
        MessageBoxIcon::Question => MB_ICONQUESTION,
    };

    unsafe {
        MessageBoxW(
            ptr::null_mut(),
            w_message.as_ptr(),
            w_title.as_ptr(),
            MB_OK | icon_flag,
        );
    }
}

pub fn message_box_ok_cancel(msg_box: &MessageBox, default: OkCancel) -> OkCancel {
    let title = msg_box.dialog.title();
    let message = msg_box.dialog.message();
    let icon = msg_box.icon();

    let w_title = to_wstring(title);
    let w_message = to_wstring(message);

    let icon_flag = match icon {
        MessageBoxIcon::Info => MB_ICONINFORMATION,
        MessageBoxIcon::Warning => MB_ICONWARNING,
        MessageBoxIcon::Error => MB_ICONERROR,
        MessageBoxIcon::Question => MB_ICONQUESTION,
    };

    let default_button = match default {
        OkCancel::Ok => MB_DEFBUTTON1,
        OkCancel::Cancel => MB_DEFBUTTON2,
    };

    let result = unsafe {
        MessageBoxW(
            ptr::null_mut(),
            w_message.as_ptr(),
            w_title.as_ptr(),
            MB_OKCANCEL | icon_flag | default_button,
        )
    };

    match result {
        IDOK => OkCancel::Ok,
        _ => OkCancel::Cancel,
    }
}

pub fn message_box_yes_no(msg_box: &MessageBox, default: YesNo) -> YesNo {
    let title = msg_box.dialog.title();
    let message = msg_box.dialog.message();
    let icon = msg_box.icon();

    let w_title = to_wstring(title);
    let w_message = to_wstring(message);

    let icon_flag = match icon {
        MessageBoxIcon::Info => MB_ICONINFORMATION,
        MessageBoxIcon::Warning => MB_ICONWARNING,
        MessageBoxIcon::Error => MB_ICONERROR,
        MessageBoxIcon::Question => MB_ICONQUESTION,
    };

    let default_button = match default {
        YesNo::Yes => MB_DEFBUTTON1,
        YesNo::No => MB_DEFBUTTON2,
    };

    let result = unsafe {
        MessageBoxW(
            ptr::null_mut(),
            w_message.as_ptr(),
            w_title.as_ptr(),
            MB_YESNO | icon_flag | default_button,
        )
    };

    match result {
        IDYES => YesNo::Yes,
        _ => YesNo::No,
    }
}

pub fn message_box_yes_no_cancel(msg_box: &MessageBox, default: YesNoCancel) -> YesNoCancel {
    let title = msg_box.dialog.title();
    let message = msg_box.dialog.message();
    let icon = msg_box.icon();

    let w_title = to_wstring(title);
    let w_message = to_wstring(message);

    let icon_flag = match icon {
        MessageBoxIcon::Info => MB_ICONINFORMATION,
        MessageBoxIcon::Warning => MB_ICONWARNING,
        MessageBoxIcon::Error => MB_ICONERROR,
        MessageBoxIcon::Question => MB_ICONQUESTION,
    };

    let default_button = match default {
        YesNoCancel::Yes => MB_DEFBUTTON1,
        YesNoCancel::No => MB_DEFBUTTON2,
        YesNoCancel::Cancel => MB_DEFBUTTON3,
    };

    let result = unsafe {
        MessageBoxW(
            ptr::null_mut(),
            w_message.as_ptr(),
            w_title.as_ptr(),
            MB_YESNOCANCEL | icon_flag | default_button,
        )
    };

    match result {
        IDYES => YesNoCancel::Yes,
        IDNO => YesNoCancel::No,
        _ => YesNoCancel::Cancel,
    }
}

pub fn input_box(input: &InputBox) -> Option<String> {
    // For Windows, we'll use a simple message box for now
    // Note: in a real implementation, we should create a proper input dialog
    // This is a basic implementation that shows the prompt and returns the default value

    let title = input.dialog.title();
    let message = input.dialog.message();
    let default = input.default_value().unwrap_or("");
    let is_password = input.is_password();

    // Show a message box with the prompt
    let msg_type = if is_password { "Password" } else { "Input" };

    let prompt = format!("{}\n\n[Default: {}]", message, default);

    let w_title = to_wstring(&format!("{} - {}", title, msg_type));
    let w_message = to_wstring(&prompt);

    let result = unsafe {
        MessageBoxW(
            ptr::null_mut(),
            w_message.as_ptr(),
            w_title.as_ptr(),
            MB_OKCANCEL | MB_ICONQUESTION,
        )
    };

    match result {
        IDOK => Some(default.to_string()),
        _ => None,
    }
}

pub fn save_file_dialog(dialog: &FileDialog) -> Option<String> {
    let title = dialog.dialog.title();
    let path = dialog.path();
    let filter_patterns = dialog.filter_patterns();
    let description = dialog.filter_description();

    let w_title = to_wstring(title);

    // Build filter string
    let mut filter = String::new();
    if !description.is_empty() && !filter_patterns.is_empty() {
        filter.push_str(description);
        filter.push('\0');

        for (i, pattern) in filter_patterns.iter().enumerate() {
            if i > 0 {
                filter.push(';');
            }
            filter.push_str(pattern);
        }
        filter.push('\0');
    }

    // Add "All Files" filter
    filter.push_str("All Files\0*.*\0\0");
    let w_filter = to_wstring(&filter);

    // Prepare buffer for file name
    let mut buffer = vec![0u16; 260]; // MAX_PATH
    if !path.is_empty() {
        let path_w = to_wstring(path);
        let len = path_w.len().min(buffer.len() - 1);
        buffer[..len].copy_from_slice(&path_w[..len]);
    }

    let mut ofn: OPENFILENAMEW = unsafe { mem::zeroed() };
    ofn.lStructSize = mem::size_of::<OPENFILENAMEW>() as u32;
    ofn.hwndOwner = ptr::null_mut();
    ofn.lpstrFilter = if filter_patterns.is_empty() {
        ptr::null()
    } else {
        w_filter.as_ptr()
    };
    ofn.lpstrFile = buffer.as_mut_ptr();
    ofn.nMaxFile = buffer.len() as u32;
    ofn.lpstrTitle = w_title.as_ptr();
    ofn.Flags = OFN_OVERWRITEPROMPT | OFN_PATHMUSTEXIST | OFN_NOCHANGEDIR;

    let result = unsafe { GetSaveFileNameW(&mut ofn) };

    if result != 0 {
        Some(from_wstring(&buffer))
    } else {
        None
    }
}

pub fn open_file_dialog(dialog: &FileDialog) -> Option<Vec<String>> {
    let title = dialog.dialog.title();
    let path = dialog.path();
    let filter_patterns = dialog.filter_patterns();
    let description = dialog.filter_description();
    let allow_multi = dialog.multiple_selection();

    let w_title = to_wstring(title);

    // Build filter string
    let mut filter = String::new();
    if !description.is_empty() && !filter_patterns.is_empty() {
        filter.push_str(description);
        filter.push('\0');

        for (i, pattern) in filter_patterns.iter().enumerate() {
            if i > 0 {
                filter.push(';');
            }
            filter.push_str(pattern);
        }
        filter.push('\0');
    }

    // Add "All Files" filter
    filter.push_str("All Files\0*.*\0\0");
    let w_filter = to_wstring(&filter);

    // Prepare buffer for file name(s)
    let mut buffer = vec![0u16; 32768]; // Large buffer for multiple files
    if !path.is_empty() {
        let path_w = to_wstring(path);
        let len = path_w.len().min(buffer.len() - 1);
        buffer[..len].copy_from_slice(&path_w[..len]);
    }

    let mut ofn: OPENFILENAMEW = unsafe { mem::zeroed() };
    ofn.lStructSize = mem::size_of::<OPENFILENAMEW>() as u32;
    ofn.hwndOwner = ptr::null_mut();
    ofn.lpstrFilter = if filter_patterns.is_empty() {
        ptr::null()
    } else {
        w_filter.as_ptr()
    };
    ofn.lpstrFile = buffer.as_mut_ptr();
    ofn.nMaxFile = buffer.len() as u32;
    ofn.lpstrTitle = w_title.as_ptr();
    ofn.Flags = OFN_FILEMUSTEXIST | OFN_PATHMUSTEXIST | OFN_EXPLORER | OFN_NOCHANGEDIR;

    if allow_multi {
        ofn.Flags |= OFN_ALLOWMULTISELECT;
    }

    let result = unsafe { GetOpenFileNameW(&mut ofn) };

    if result != 0 {
        if allow_multi {
            let mut files = Vec::new();
            let mut start = 0;

            // First part is the directory
            let dir = from_wstring(&buffer[start..]);
            start += dir.len() + 1;

            if buffer[start] == 0 {
                // Only one file selected
                files.push(dir);
            } else {
                // Multiple files, directory followed by filenames
                while start < buffer.len() && buffer[start] != 0 {
                    let filename = from_wstring(&buffer[start..]);
                    if filename.is_empty() {
                        break;
                    }

                    let path = Path::new(&dir).join(filename);
                    files.push(path.to_string_lossy().into_owned());

                    start += filename.len() + 1;
                }
            }

            Some(files)
        } else {
            Some(vec![from_wstring(&buffer)])
        }
    } else {
        None
    }
}

pub fn select_folder_dialog(dialog: &FileDialog) -> Option<String> {
    let title = dialog.dialog.title();
    let path = dialog.path();

    let w_title = to_wstring(title);

    let mut bi: BROWSEINFOW = unsafe { mem::zeroed() };
    bi.hwndOwner = ptr::null_mut();
    bi.lpszTitle = w_title.as_ptr();
    bi.ulFlags = BIF_RETURNONLYFSDIRS | BIF_NEWDIALOGSTYLE;

    let pidl = unsafe { SHBrowseForFolderW(&mut bi) };

    if !pidl.is_null() {
        let mut buffer = vec![0u16; 260]; // MAX_PATH
        let result = unsafe { SHGetPathFromIDListW(pidl, buffer.as_mut_ptr()) };
        unsafe { CoTaskMemFree(pidl) };

        if result != 0 {
            Some(from_wstring(&buffer))
        } else {
            None
        }
    } else {
        None
    }
}

pub fn color_chooser_dialog(chooser: &ColorChooser) -> Option<(String, [u8; 3])> {
    let title = chooser.dialog.title();

    let w_title = to_wstring(title);

    let default_rgb = match chooser.default_color() {
        DefaultColorValue::Hex(hex) => super::hex_to_rgb(hex),
        DefaultColorValue::RGB(rgb) => *rgb,
    };

    let rgb_value = ((default_rgb[0] as u32)
        | ((default_rgb[1] as u32) << 8)
        | ((default_rgb[2] as u32) << 16));

    let mut custom_colors = [0u32; 16];

    let mut cc: CHOOSECOLORW = unsafe { mem::zeroed() };
    cc.lStructSize = mem::size_of::<CHOOSECOLORW>() as u32;
    cc.hwndOwner = ptr::null_mut();
    cc.rgbResult = rgb_value;
    cc.lpCustColors = custom_colors.as_mut_ptr();
    cc.Flags = CC_RGBINIT | CC_FULLOPEN | CC_ANYCOLOR;

    let result = unsafe { ChooseColorW(&mut cc) };

    if result != 0 {
        let r = (cc.rgbResult & 0xFF) as u8;
        let g = ((cc.rgbResult >> 8) & 0xFF) as u8;
        let b = ((cc.rgbResult >> 16) & 0xFF) as u8;

        let rgb = [r, g, b];
        let hex = super::rgb_to_hex(&rgb);

        Some((hex, rgb))
    } else {
        None
    }
}

pub fn notification(notification: &Notification) -> bool {
    let title = notification.title();
    let message = notification.message();

    // Windows doesn't have built-in notification API in the standard library
    // This is a simplified implementation that shows a balloon notification
    // In a real application, you'd want to use the Windows notification API

    // For simplicity, show a message box instead
    let w_title = to_wstring(title);
    let w_message = to_wstring(message);

    let result = unsafe {
        MessageBoxW(
            ptr::null_mut(),
            w_message.as_ptr(),
            w_title.as_ptr(),
            MB_OK | MB_ICONINFORMATION,
        )
    };

    result == IDOK
}
