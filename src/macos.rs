use super::*;
use std::ffi::{CString, c_void};
use std::path::Path;
use std::sync::Once;

use objc2::rc::Retained;
use objc2::{class, msg_send, Message};
use objc2_foundation::{
    MainThreadMarker, NSArray, NSObject, NSString, NSURL
};
use objc2_core_foundation::{
    CGRect, CGPoint, CGSize
};
use objc2_app_kit::{
    NSAlert, NSAlertStyle, NSApplication, NSButton, NSFloatingWindowLevel, NSModalResponse, NSModalResponseOK, NSRunningApplication, NSSavePanel, NSTextField

};

// Dynamic library loading for Cocoa framework
static INIT: Once = Once::new();
static mut COCOA_LIB: Option<CocoaFunctions> = None;

struct CocoaFunctions {
    // Handle to dynamically loaded library
    _lib_handle: *mut c_void,
}

impl Drop for CocoaFunctions {
    fn drop(&mut self) {
        if !self._lib_handle.is_null() {
            unsafe {
                dlclose(self._lib_handle);
            }
        }
    }
}

// Ensure we can call dlopen/dlsym/dlclose
#[link(name = "dl")]
extern "C" {
    fn dlopen(filename: *const std::os::raw::c_char, flag: i32) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const std::os::raw::c_char) -> *mut c_void;
    fn dlclose(handle: *mut c_void) -> i32;
}

fn get_cocoa_functions() -> &'static CocoaFunctions {
    unsafe {
        INIT.call_once(|| {
            // Typical flags for dlopen
            const RTLD_NOW: i32 = 2;
            const RTLD_GLOBAL: i32 = 8;

            // Load the Cocoa framework
            let framework_path = CString::new("/System/Library/Frameworks/Cocoa.framework/Cocoa").unwrap();
            let handle = dlopen(framework_path.as_ptr(), RTLD_NOW | RTLD_GLOBAL);
            
            if handle.is_null() {
                panic!("Could not dlopen Cocoa.framework");
            }

            COCOA_LIB = Some(CocoaFunctions {
                _lib_handle: handle,
            });
        });

        COCOA_LIB.as_ref().unwrap()
    }
}

// Ensure macOS UI operations happen on the main thread
fn ensure_main_thread<F, R>(f: F) -> R
where
    F: Fn(MainThreadMarker) -> R + Send + Sync + 'static,
    R: Send + 'static,
{
    dispatch2::run_on_main(|mtm| f(mtm))
}

// Convert Rust string to NSString
fn to_ns_string(s: &str) -> objc2::rc::Retained<NSString> {
    NSString::from_str(s)
}

// NSAlert helpers
fn create_alert(title: &str, message: &str, icon: MessageBoxIcon) -> objc2::rc::Retained<NSAlert> {
    let _ = get_cocoa_functions(); // Ensure Cocoa is loaded
    
    let alert_style = match icon {
        MessageBoxIcon::Info => NSAlertStyle::Informational,
        MessageBoxIcon::Warning => NSAlertStyle::Warning,
        MessageBoxIcon::Error => NSAlertStyle::Critical,
        MessageBoxIcon::Question => NSAlertStyle::Informational,
    };

    unsafe {
        let alert: objc2::rc::Retained<NSAlert> = msg_send![class!(NSAlert), new];
        
        let ns_title = to_ns_string(title);
        let _: () = msg_send![&alert, setMessageText: &*ns_title];
        
        let ns_message = to_ns_string(message);
        let _: () = msg_send![&alert, setInformativeText: &*ns_message];
        
        let _: () = msg_send![&alert, setAlertStyle: alert_style];
        
        alert
    }
}

fn run_alert(mtm: MainThreadMarker, alert: &NSAlert) -> NSModalResponse {
    unsafe {
        alert.window().setLevel(NSFloatingWindowLevel);
        NSApplication::sharedApplication(mtm).activate();
        let response: NSModalResponse = alert.runModal();
        println!("e");
        response
    }
}

// Implementation of public functions

pub struct MessageBoxConfig<'a> {
    pub title: &'a str,
    pub message: &'a str,
    pub icon: MessageBoxIcon,
    pub buttons: &'a [&'a str],
    pub default_button: Option<usize>,
}

pub fn message_box(config: MessageBoxConfig) -> Option<usize> {
    let title = config.title.to_string();
    let message = config.message.to_string();
    let buttons: Vec<String> = config.buttons.iter().map(|&s| s.to_string()).collect();
    let default_button = config.default_button;
    let icon = config.icon;
    
    if buttons.is_empty() {
        return None;
    }
    
    ensure_main_thread(move |mtm| {
        let _ = get_cocoa_functions();
        
        unsafe {
            let alert = create_alert(&title, &message, icon);
            
            // Add buttons in original order
            for button_text in buttons.iter() {
                let ns_button = to_ns_string(button_text);
                let _: () = msg_send![&alert, addButtonWithTitle: &*ns_button];
            }
            
            // Set default button if specified
            if let Some(default_idx) = default_button {
                if default_idx < buttons.len() {
                    let buttons_array: objc2::rc::Retained<NSArray<NSObject>> = msg_send![&alert, buttons];
                    let default_btn: objc2::rc::Retained<NSButton> = msg_send![&buttons_array, objectAtIndex: default_idx];
                    
                    // Set key equivalent "\r" (Return key) for the default button
                    let key_return = to_ns_string("\r");
                    let _: () = msg_send![&default_btn, setKeyEquivalent: &*key_return];
                }
            }
            
            let response = run_alert(mtm, &alert);
            
            // Convert response (1000, 1001, etc.) to button index
            if response >= 1000 && response < (1000 + buttons.len() as isize) {
                Some((response - 1000) as usize)
            } else {
                None
            }
        }
    })
}

// Refactored existing message box functions
pub fn message_box_ok(title: &str, message: &str, icon: MessageBoxIcon) {
    let config = MessageBoxConfig {
        title,
        message,
        icon,
        buttons: &["OK"],
        default_button: Some(0),
    };
    
    let _ = message_box(config);
}

pub fn message_box_ok_cancel(title: &str, message: &str, icon: MessageBoxIcon, default: OkCancel) -> OkCancel {
    let default_button = match default {
        OkCancel::Ok => Some(1),
        OkCancel::Cancel => Some(0),
    };
    
    let config = MessageBoxConfig {
        title,
        message,
        icon,
        buttons: &["Cancel", "OK"],
        default_button,
    };
    
    match message_box(config) {
        Some(0) => OkCancel::Cancel,
        Some(1) => OkCancel::Ok,
        _ => OkCancel::Cancel,
    }
}

pub fn message_box_yes_no(title: &str, message: &str, icon: MessageBoxIcon, default: YesNo) -> YesNo {
    let default_button = match default {
        YesNo::Yes => Some(1),
        YesNo::No => Some(0),
    };
    
    let config = MessageBoxConfig {
        title,
        message,
        icon,
        buttons: &["No", "Yes"],
        default_button,
    };
    
    match message_box(config) {
        Some(0) => YesNo::No,
        Some(1) => YesNo::Yes,
        _ => YesNo::No,
    }
}

pub fn message_box_yes_no_cancel(title: &str, message: &str, icon: MessageBoxIcon, default: YesNoCancel) -> YesNoCancel {
    let default_button = match default {
        YesNoCancel::Yes => Some(2),
        YesNoCancel::No => Some(1),
        YesNoCancel::Cancel => Some(0),
    };
    
    let config = MessageBoxConfig {
        title,
        message,
        icon,
        buttons: &["Cancel", "No", "Yes"],
        default_button,
    };
    
    match message_box(config) {
        Some(0) => YesNoCancel::Cancel,
        Some(1) => YesNoCancel::No,
        Some(2) => YesNoCancel::Yes,
        _ => YesNoCancel::Cancel,
    }
}

pub fn input_box(title: &str, message: &str, default: Option<&str>) -> Option<String> {

    let title = title.to_string();
    let message = message.to_string();
    let default = default.map(|s| s.to_string());
    
    ensure_main_thread(move |mtm| {
        let _ = get_cocoa_functions();
        
        unsafe {
            // Create alert with text field
            let alert = create_alert(&title, &message, MessageBoxIcon::Info);

            // Add buttons
            let cancel_button = to_ns_string("Cancel");
            alert.addButtonWithTitle(&cancel_button);

            let ok_button = to_ns_string("OK");
            alert.addButtonWithTitle(&ok_button);

            // Add text field
            alert.setShowsHelp(false);

            let frame = CGRect::new(
                CGPoint::new(0.0, 0.0), 
                CGSize::new(200.0, 24.0)
            );

            let default = default.as_deref().unwrap_or("");
            let ns_default = to_ns_string(default);
            let text_field = NSTextField::textFieldWithString(&ns_default, mtm);
            text_field.setFrame(frame);

            alert.setAccessoryView(Some(&text_field));

            let response = run_alert(mtm, &alert);
            
            if response == 1001 {  // OK button
                let value: objc2::rc::Retained<NSString> = msg_send![&text_field, stringValue];
                Some(value.to_string())
            } else {
                None
            }
        }
    })
}

pub fn save_file_dialog(title: &str, path: &str, filter_patterns: &[&str], description: &str) -> Option<String> {
    let title = title.to_string();
    let path = path.to_string();
    let filter_patterns: Vec<String> = filter_patterns.iter().map(|&s| s.to_string()).collect();
    let description = description.to_string();
    
    ensure_main_thread(move |mtm| {
        let _ = get_cocoa_functions();
        
        unsafe {
            // Create save panel
            let save_panel = NSSavePanel::savePanel(mtm);
            
            // Configure panel
            let ns_title = to_ns_string(&title);
            save_panel.setTitle(Some(&ns_title));
            
            // Set initial directory if provided
            if !path.is_empty() {
                if let Some(dir) = Path::new(&path).parent() {
                    if let Some(dir_str) = dir.to_str() {
                        let ns_dir = to_ns_string(dir_str);
                        let url = NSURL::fileURLWithPath(&ns_dir);
                        let _: () = save_panel.setDirectoryURL(Some(&url));
                    }
                }
                
                // Set default filename
                if let Some(filename) = Path::new(&path).file_name() {
                    if let Some(filename_str) = filename.to_str() {
                        let ns_filename = to_ns_string(filename_str);
                        let _: () = save_panel.setNameFieldStringValue(&ns_filename); 
                    }
                }
            }
            
            // Setup file type filtering
            if !filter_patterns.is_empty() {

                let allowed_types: Vec<objc2::rc::Retained<NSString>> = filter_patterns
                    .iter()
                    .map(|p| {
                        // Extract extension from pattern (*.ext -> ext)
                        let ext = p.trim_start_matches("*.");
                        to_ns_string(ext)
                    })
                    .collect();
                
                let ns_array: objc2::rc::Retained<NSArray<NSString>> = 
                    NSArray::from_retained_slice(&allowed_types);
                
                save_panel.setAllowedFileTypes(Some(&ns_array));
                save_panel.setAllowsOtherFileTypes(false);
            }

            save_panel.setLevel(NSFloatingWindowLevel);
            NSApplication::sharedApplication(mtm).activate();
            
            let response: NSModalResponse = save_panel.runModal();
                        
            if response == NSModalResponseOK {
                let url: objc2::rc::Retained<NSObject> = msg_send![&save_panel, URL];
                let path: objc2::rc::Retained<NSString> = msg_send![&url, path];
                Some(path.to_string())
            } else {
                None
            }
        }
    })
}

pub fn open_file_dialog(title: &str, path: &str, filter_patterns: &[&str], description: &str, 
                    allow_multi: bool) -> Option<Vec<String>> {
    let title = title.to_string();
    let path = path.to_string();
    let filter_patterns: Vec<String> = filter_patterns.iter().map(|&s| s.to_string()).collect();
    let description = description.to_string();
    
    ensure_main_thread(move |mtm| {
        let _ = get_cocoa_functions();
        
        unsafe {
            // Create open panel
            let open_panel: objc2::rc::Retained<NSObject> = msg_send![class!(NSOpenPanel), openPanel];
            
            // Configure panel
            let ns_title = to_ns_string(&title);
            let _: () = msg_send![&open_panel, setTitle: &*ns_title];
            let _: () = msg_send![&open_panel, setCanChooseFiles: true];
            let _: () = msg_send![&open_panel, setCanChooseDirectories: false];
            let _: () = msg_send![&open_panel, setAllowsMultipleSelection: allow_multi];
            
            // Set initial directory if provided
            if !path.is_empty() {
                let ns_path = to_ns_string(&path);
                let url: objc2::rc::Retained<NSObject> = msg_send![class!(NSURL), fileURLWithPath: &*ns_path];
                let _: () = msg_send![&open_panel, setDirectoryURL: &*url];
            }
            
            // Setup file type filtering
            if !filter_patterns.is_empty() {
                let allowed_types: Vec<objc2::rc::Retained<NSString>> = filter_patterns
                    .iter()
                    .map(|p| {
                        // Extract extension from pattern (*.ext -> ext)
                        let ext = p.trim_start_matches("*.");
                        to_ns_string(ext)
                    })
                    .collect();
                
                let ns_array: objc2::rc::Retained<NSArray<NSString>> = 
                    NSArray::from_retained_slice(&allowed_types);
                let _: () = msg_send![&open_panel, setAllowedFileTypes: &*ns_array];
            }
            
            // Show panel and get result
            let mtm = MainThreadMarker::new().unwrap();
            let _app: objc2::rc::Retained<NSApplication> = 
                NSApplication::sharedApplication(mtm);
            let response: NSModalResponse = msg_send![&open_panel, runModal];
            
            if response == NSModalResponseOK {
                let urls: objc2::rc::Retained<NSArray<NSObject>> = msg_send![&open_panel, URLs];
                let count: usize = msg_send![&urls, count];
                
                let mut files = Vec::with_capacity(count);
                for i in 0..count {
                    let url: objc2::rc::Retained<NSObject> = msg_send![&urls, objectAtIndex: i];
                    let path: objc2::rc::Retained<NSString> = msg_send![&url, path];
                    files.push(path.to_string());
                }
                
                Some(files)
            } else {
                None
            }
        }
    })
}

pub fn select_folder_dialog(title: &str, path: &str) -> Option<String> {
    let title = title.to_string();
    let path = path.to_string();
    
    ensure_main_thread(move |mtm| {
        let _ = get_cocoa_functions();
        
        unsafe {
            // Create open panel
            let open_panel: objc2::rc::Retained<NSObject> = msg_send![class!(NSOpenPanel), openPanel];
            
            // Configure panel
            let ns_title = to_ns_string(&title);
            let _: () = msg_send![&open_panel, setTitle: &*ns_title];
            let _: () = msg_send![&open_panel, setCanChooseFiles: false];
            let _: () = msg_send![&open_panel, setCanChooseDirectories: true];
            let _: () = msg_send![&open_panel, setAllowsMultipleSelection: false];
            
            // Set initial directory if provided
            if !path.is_empty() {
                let ns_path = to_ns_string(&path);
                let url: objc2::rc::Retained<NSObject> = msg_send![class!(NSURL), fileURLWithPath: &*ns_path];
                let _: () = msg_send![&open_panel, setDirectoryURL: &*url];
            }
            
            // Show panel and get result
            let mtm = MainThreadMarker::new().unwrap();
            let _app: objc2::rc::Retained<NSApplication> = 
                NSApplication::sharedApplication(mtm);
            let response: NSModalResponse = msg_send![&open_panel, runModal];
            
            if response == NSModalResponseOK {
                let url: objc2::rc::Retained<NSObject> = msg_send![&open_panel, URL];
                let path: objc2::rc::Retained<NSString> = msg_send![&url, path];
                Some(path.to_string())
            } else {
                None
            }
        }
    })
}

pub fn color_chooser_dialog(title: &str, default: DefaultColorValue) -> Option<(String, [u8; 3])> {
    let title = title.to_string();
    let default_owned = match default {
        DefaultColorValue::Hex(hex) => DefaultColorValue::Hex(hex.to_string()),
        DefaultColorValue::RGB(rgb) => DefaultColorValue::RGB(rgb),
    };
    
    ensure_main_thread(move |mtm| {
        let _ = get_cocoa_functions();
        
        unsafe {
            // Get default color values
            let default_rgb = match &default_owned {
                DefaultColorValue::Hex(hex) => super::hex_to_rgb(hex),
                DefaultColorValue::RGB(rgb) => *rgb,
            };
            
            // Create color panel
            let color_panel: objc2::rc::Retained<NSObject> = msg_send![class!(NSColorPanel), sharedColorPanel];
            let _: () = msg_send![&color_panel, setShowsAlpha: false];
            
            // Set initial color
            let r = default_rgb[0] as f64 / 255.0;
            let g = default_rgb[1] as f64 / 255.0;
            let b = default_rgb[2] as f64 / 255.0;
            
            let color: objc2::rc::Retained<NSObject> = msg_send![class!(NSColor), 
                                                   colorWithSRGBRed: r, green: g, blue: b, alpha: 1.0];
            let _: () = msg_send![&color_panel, setColor: &*color];
            
            // Set custom title
            let ns_title = to_ns_string(&title);
            let _: () = msg_send![&color_panel, setTitle: &*ns_title];
            
            // Show panel modally (this is a bit tricky in AppKit)
            let _: () = msg_send![&color_panel, orderFront: std::ptr::null_mut::<NSObject>()];
            
            // Create a custom modal runloop
            let mtm = MainThreadMarker::new().unwrap();
            let app: objc2::rc::Retained<NSApplication> = 
                NSApplication::sharedApplication(mtm);
            
            let result: NSModalResponse = msg_send![&app, runModalForWindow: &*color_panel];
            
            if result == NSModalResponseOK {
                // Get selected color
                let selected_color: objc2::rc::Retained<NSObject> = msg_send![&color_panel, color];
                
                // Get RGB components
                let mut r: f64 = 0.0;
                let mut g: f64 = 0.0;
                let mut b: f64 = 0.0;
                let mut a: f64 = 0.0;
                
                let _: () = msg_send![&selected_color, getRed: &mut r, green: &mut g, blue: &mut b, alpha: &mut a];
                
                let rgb = [
                    (r * 255.0) as u8,
                    (g * 255.0) as u8,
                    (b * 255.0) as u8,
                ];
                
                let hex = super::rgb_to_hex(&rgb);
                Some((hex, rgb))
            } else {
                None
            }
        }
    })
}