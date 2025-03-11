use super::*;
use std::ffi::{OsStr, OsString};
use std::iter::once;
use std::mem;
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::path::Path;
use std::ptr;
use ::windows::core::HSTRING;
use ::windows::Data::Xml::Dom::XmlDocument;
use ::windows::Win32::UI::Shell::GetCurrentProcessExplicitAppUserModelID;
use ::windows::Win32::System::Com::CoUninitialize;
use ::windows::UI::Notifications::ToastNotification;
use ::windows::UI::Notifications::ToastNotificationManager;
use ::windows::Win32::System::Com::COINIT_MULTITHREADED;
use ::windows::Win32::System::Com::CoInitializeEx;
use ::windows::Win32::Foundation::HWND;
use ::windows::UI::Notifications::ToastTemplateType;
use ::windows::Win32::System::SystemInformation::GetVersionExW;

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

// type HWND = *mut std::ffi::c_void;
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

/* */
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
            HWND(0),
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
            HWND(0),
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
            HWND(0),
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
            HWND(0),
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
            HWND(0),
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

                    let path = Path::new(&dir).join(&filename);
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
    /*
    if is_windows10_or_newer() {
        match show_toast_notification(notification) {
            Ok(true) => return true,
            Ok(false) => (), // Fall back to message box
            Err(_) => (),    // Fall back to message box
        }
    }
    */
    // Fallback for older Windows versions or if Toast notification fails
    show_legacy_notification(notification)
}

fn is_windows10_or_newer() -> bool {
    use ::windows::Win32::System::SystemInformation::{OSVERSIONINFOW, OSVERSIONINFOEXW};
    
    unsafe {
        let mut version_info: OSVERSIONINFOEXW = mem::zeroed();
        version_info.dwOSVersionInfoSize = mem::size_of::<OSVERSIONINFOEXW>() as u32;
        
        // GetVersionExW is deprecated but still works
        if GetVersionExW(ptr::addr_of_mut!(version_info) as *mut OSVERSIONINFOW).as_bool() {
            return version_info.dwMajorVersion >= 10;
        }
    }
    
    false
}

fn show_legacy_notification(notification: &Notification) -> bool {
    let title = to_wstring(notification.title());
    let message = to_wstring(notification.message());
    
    let result = unsafe {
        MessageBoxW(
            HWND(0),
            message.as_ptr(),
            title.as_ptr(),
            MB_OK | MB_ICONINFORMATION,
        )
    };
    
    result == IDOK
}

/*
fn show_toast_notification(notification: &Notification) -> Result<bool> {
    // Initialize COM
    unsafe { CoInitializeEx(ptr::null(), COINIT_MULTITHREADED)? };
    
    // Ensure proper cleanup
    let _com_uninit = ComUninitializer;
    
    // Get the app's AUMID (App User Model ID)
    let app_id = get_app_user_model_id()?;
    
    // Create toast content
    let toast_xml = create_toast_content(notification)?;
    
    // Get toast notifier
    let toast_manager = ToastNotificationManager::GetDefault()?;
    let notifier = toast_manager.CreateToastNotifierWithId(&app_id)?;
    
    // Create and show notification
    let toast = ToastNotification::CreateToastNotification(&toast_xml)?;
    notifier.Show(&toast)?;
    
    Ok(true)
}

// Helper to automatically uninitialize COM when going out of scope
struct ComUninitializer;

impl Drop for ComUninitializer {
    fn drop(&mut self) {
        unsafe { CoUninitialize() };
    }
}

fn get_app_user_model_id() -> Result<HSTRING> {
    // Try to get the registered AUMID for the current process
    unsafe {
        let mut aumid_ptr = ptr::null_mut();
        let hr = GetCurrentProcessExplicitAppUserModelID(&mut aumid_ptr);
        
        if hr.is_ok() && !aumid_ptr.is_null() {
            // Convert to HSTRING and return
            let aumid = HSTRING::from_wide(
                std::slice::from_raw_parts(
                    aumid_ptr, 
                    wcslen(aumid_ptr)
                )
            )?;
            
            // Free the string allocated by GetCurrentProcessExplicitAppUserModelID
            ::windows::Win32::System::Com::CoTaskMemFree(aumid_ptr as _);
            
            return Ok(aumid);
        }
    }
    
    // Fallback: use the executable name as AUMID
    let exe_name = std::env::current_exe()
        .ok()
        .and_then(|path| path.file_name().map(|name| name.to_string_lossy().to_string()))
        .unwrap_or_else(|| "TinyFileDialogs".to_string());
    
    Ok(HSTRING::from(exe_name))
}

// Count wide characters in a null-terminated string
unsafe fn wcslen(s: *const u16) -> usize {
    let mut len = 0;
    while *s.add(len) != 0 {
        len += 1;
    }
    len
}

fn create_toast_content(notification: &Notification) -> WinResult<XmlDocument> {
    // Create the XML document
    let toast_xml = ToastNotificationManager::GetTemplateContent(ToastTemplateType::ToastText02)?;
    
    // Get text nodes
    let text_nodes = toast_xml.GetElementsByTagName(&HSTRING::from("text"))?;
    
    // Set title
    if let Ok(title_node) = text_nodes.Item(0) {
        title_node.AppendChild(&toast_xml.CreateTextNode(&HSTRING::from(notification.title()))?)?;
    }
    
    // Set message
    if let Ok(message_node) = text_nodes.Item(1) {
        message_node.AppendChild(&toast_xml.CreateTextNode(&HSTRING::from(notification.message()))?)?;
    }
    
    // Add sound if specified
    if let Some(sound_name) = notification.sound() {
        add_sound_element(&toast_xml, sound_name)?;
    }
    
    Ok(toast_xml)
}

fn add_sound_element(toast_xml: &XmlDocument, sound_name: &str) -> WinResult<()> {
    // Create audio element
    let toast_element = toast_xml.DocumentElement()?;
    let audio_element = toast_xml.CreateElement(&HSTRING::from("audio"))?;
    
    // Set sound attribute
    audio_element.SetAttribute(
        &HSTRING::from("src"), 
        &HSTRING::from(format!("ms-winsoundevent:Notification.{}", sound_name))
    )?;
    
    // Append to toast
    toast_element.AppendChild(&audio_element)?;
    
    Ok(())
}

// Add action to open the app when notification is clicked
fn add_launch_action(toast_xml: &XmlDocument, launch_arg: &str) -> WinResult<()> {
    let toast_element = toast_xml.DocumentElement()?;
    let actions_element = toast_xml.CreateElement(&HSTRING::from("actions"))?;
    let action_element = toast_xml.CreateElement(&HSTRING::from("action"))?;
    
    action_element.SetAttribute(&HSTRING::from("activationType"), &HSTRING::from("foreground"))?;
    action_element.SetAttribute(&HSTRING::from("content"), &HSTRING::from("Open"))?;
    action_element.SetAttribute(&HSTRING::from("arguments"), &HSTRING::from(launch_arg))?;
    
    actions_element.AppendChild(&action_element)?;
    toast_element.AppendChild(&actions_element)?;
    
    Ok(())
}

// Optional: Add image to notification
fn add_image_element(toast_xml: &XmlDocument, image_path: &str) -> WinResult<()> {
    // Get binding element
    let binding_elements = toast_xml.GetElementsByTagName(&HSTRING::from("binding"))?;
    if let Ok(binding) = binding_elements.Item(0) {
        // Create image element
        let image_element = toast_xml.CreateElement(&HSTRING::from("image"))?;
        
        // Set attributes
        image_element.SetAttribute(&HSTRING::from("placement"), &HSTRING::from("appLogoOverride"))?;
        
        // Create URI for image
        let image_uri = if image_path.starts_with("http") {
            image_path.to_string()
        } else {
            // Convert local path to URI
            format!("file:///{}", image_path.replace('\\', "/"))
        };
        
        image_element.SetAttribute(&HSTRING::from("src"), &HSTRING::from(image_uri))?;
        
        // Append to binding
        binding.AppendChild(&image_element)?;
    }
    
    Ok(())
}
*/