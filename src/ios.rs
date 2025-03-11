use super::*;
use std::ffi::CString;
use std::os::raw::{c_char, c_void};
use std::ptr;
use std::sync::Once;

use uikit_sys::foundation::{
    NSArray, NSAutoreleasePool, NSInteger, NSObject, NSString, BOOL, 
    NSStringEncoding, NSUInteger, NSStringCompareOptions, YES, NO,
};
use uikit_sys::uikit::{
    UIAlertAction, UIAlertActionStyle, UIAlertController, UIAlertControllerStyle,
    UIApplication, UIColor, UITextField, UIViewController,
};
use uikit_sys::dispatch::{dispatch_async, dispatch_get_main_queue};
use uikit_sys::objc::{
    objc_getClass, sel_registerName, objc_msgSend, objc_retain, objc_release,
    id, class, objc_object, SEL, Class,
};

// Safety macros for Objective-C message sending
macro_rules! msg_send {
    ($obj:expr, $sel:expr) => ({
        unsafe {
            let obj = $obj as id;
            let sel = sel_registerName(stringify!($sel).as_ptr() as *const c_char);
            let result = objc_msgSend(obj, sel);
            result
        }
    });
    ($obj:expr, $sel:expr, $($arg:expr),*) => ({
        unsafe {
            let obj = $obj as id;
            let sel = sel_registerName(concat!(stringify!($sel), ":", $(stringify!($($arg)),*)).as_ptr() as *const c_char);
            let result = objc_msgSend(obj, sel, $($arg as id),*);
            result
        }
    });
}

// NSString creation helper
fn ns_string(s: &str) -> id {
    let cstring = CString::new(s).unwrap();
    let string_class: id = unsafe { objc_getClass("NSString\0".as_ptr() as *const c_char) };
    let utf8_encoding: NSStringEncoding = 4; // NSUTF8StringEncoding
    
    unsafe {
        let selector = sel_registerName("stringWithUTF8String:\0".as_ptr() as *const c_char);
        objc_msgSend(string_class, selector, cstring.as_ptr())
    }
}

// Autorelease pool wrapper
struct AutoreleasePool {
    pool: id,
}

impl AutoreleasePool {
    fn new() -> Self {
        let pool_class: id = unsafe { objc_getClass("NSAutoreleasePool\0".as_ptr() as *const c_char) };
        let pool = unsafe {
            let alloc_sel = sel_registerName("alloc\0".as_ptr() as *const c_char);
            let init_sel = sel_registerName("init\0".as_ptr() as *const c_char);
            let alloc_result = objc_msgSend(pool_class, alloc_sel);
            objc_msgSend(alloc_result, init_sel)
        };
        
        Self { pool }
    }
}

impl Drop for AutoreleasePool {
    fn drop(&mut self) {
        unsafe {
            let drain_sel = sel_registerName("drain\0".as_ptr() as *const c_char);
            objc_msgSend(self.pool, drain_sel);
        }
    }
}

// Helper struct to store a Rust callback to be invoked from ObjC
struct AlertCallback {
    callback: Box<dyn FnOnce(i32) + Send + 'static>,
}

impl AlertCallback {
    fn new<F: FnOnce(i32) + Send + 'static>(callback: F) -> *mut c_void {
        let boxed = Box::new(Self {
            callback: Box::new(callback),
        });
        Box::into_raw(boxed) as *mut c_void
    }
    
    fn invoke(context: *mut c_void, result: i32) {
        if !context.is_null() {
            let boxed = unsafe { Box::from_raw(context as *mut AlertCallback) };
            (boxed.callback)(result);
        }
    }
}

// Helper function to get root view controller
fn get_root_view_controller() -> id {
    unsafe {
        let app_class: id = objc_getClass("UIApplication\0".as_ptr() as *const c_char);
        let shared_app_sel = sel_registerName("sharedApplication\0".as_ptr() as *const c_char);
        let app = objc_msgSend(app_class, shared_app_sel);
        
        let delegate_sel = sel_registerName("delegate\0".as_ptr() as *const c_char);
        let delegate = objc_msgSend(app, delegate_sel);
        
        let window_sel = sel_registerName("window\0".as_ptr() as *const c_char);
        let window = objc_msgSend(delegate, window_sel);
        
        let root_vc_sel = sel_registerName("rootViewController\0".as_ptr() as *const c_char);
        objc_msgSend(window, root_vc_sel)
    }
}

// Helper to convert RGB components to UIColor
fn rgb_to_uicolor(r: u8, g: u8, b: u8) -> id {
    let r_float = r as f64 / 255.0;
    let g_float = g as f64 / 255.0;
    let b_float = b as f64 / 255.0;
    
    unsafe {
        let color_class: id = objc_getClass("UIColor\0".as_ptr() as *const c_char);
        let selector = sel_registerName("colorWithRed:green:blue:alpha:\0".as_ptr() as *const c_char);
        objc_msgSend(color_class, selector, r_float, g_float, b_float, 1.0)
    }
}

// Helper to run UIAlertController
fn run_alert_controller(controller: id, completion: impl FnOnce(i32) + Send + 'static) {
    let root_vc = get_root_view_controller();
    if root_vc == ptr::null_mut() {
        completion(-1);
        return;
    }
    
    let context = AlertCallback::new(completion);
    
    unsafe {
        let queue = dispatch_get_main_queue();
        
        dispatch_async(queue, Box::into_raw(Box::new(move || {
            let _pool = AutoreleasePool::new();
            
            let present_sel = sel_registerName("presentViewController:animated:completion:\0".as_ptr() as *const c_char);
            objc_msgSend(root_vc, present_sel, controller, YES as BOOL, ptr::null_mut::<c_void>());
        })) as *mut c_void);
    }
}

// Create UIAlertAction with handler
fn create_alert_action(title: &str, style: UIAlertActionStyle, tag: i32, controller: id, completion: *mut c_void) -> id {
    let title_str = ns_string(title);
    
    unsafe {
        let action_class: id = objc_getClass("UIAlertAction\0".as_ptr() as *const c_char);
        
        extern "C" fn action_handler(action: id, context: *mut c_void) {
            let tag_ptr = unsafe {
                let selector = sel_registerName("tag\0".as_ptr() as *const c_char);
                objc_msgSend(action, selector)
            };
            let tag = tag_ptr as i32;
            
            AlertCallback::invoke(context, tag);
        }
        
        let block_context = completion;
        
        let action_with_title_sel = sel_registerName("actionWithTitle:style:handler:\0".as_ptr() as *const c_char);
        let action: id = objc_msgSend(
            action_class, 
            action_with_title_sel, 
            title_str,
            style as NSInteger,
            action_handler as *mut c_void,
            block_context
        );
        
        // Set tag on action
        let set_tag_sel = sel_registerName("setTag:\0".as_ptr() as *const c_char);
        objc_msgSend(action, set_tag_sel, tag);
        
        action
    }
}

pub fn message_box_ok(msg_box: &MessageBox) {
    let title = msg_box.dialog.title();
    let message = msg_box.dialog.message();
    
    let _pool = AutoreleasePool::new();
    
    unsafe {
        let controller_class: id = objc_getClass("UIAlertController\0".as_ptr() as *const c_char);
        let title_str = ns_string(title);
        let message_str = ns_string(message);
        
        let alert_with_title_sel = sel_registerName("alertControllerWithTitle:message:preferredStyle:\0".as_ptr() as *const c_char);
        let alert: id = objc_msgSend(
            controller_class, 
            alert_with_title_sel, 
            title_str, 
            message_str, 
            UIAlertControllerStyle::Alert as NSInteger
        );
        
        let context = AlertCallback::new(|_| {});
        let ok_action = create_alert_action("OK", UIAlertActionStyle::Default, 1, alert, context);
        
        let add_action_sel = sel_registerName("addAction:\0".as_ptr() as *const c_char);
        objc_msgSend(alert, add_action_sel, ok_action);
        
        run_alert_controller(alert, |_| {});
    }
}

pub fn message_box_ok_cancel(msg_box: &MessageBox, default: OkCancel) -> OkCancel {
    let title = msg_box.dialog.title();
    let message = msg_box.dialog.message();
    let mut result = OkCancel::Cancel;
    
    let _pool = AutoreleasePool::new();
    
    unsafe {
        let controller_class: id = objc_getClass("UIAlertController\0".as_ptr() as *const c_char);
        let title_str = ns_string(title);
        let message_str = ns_string(message);
        
        let alert_with_title_sel = sel_registerName("alertControllerWithTitle:message:preferredStyle:\0".as_ptr() as *const c_char);
        let alert: id = objc_msgSend(
            controller_class, 
            alert_with_title_sel, 
            title_str, 
            message_str, 
            UIAlertControllerStyle::Alert as NSInteger
        );
        
        // Create a semaphore to wait for the alert completion
        let semaphore = std::sync::Arc::new(std::sync::Mutex::new(false));
        let s_clone = semaphore.clone();
        
        let context = AlertCallback::new(move |tag| {
            if tag == 1 {
                result = OkCancel::Ok;
            } else {
                result = OkCancel::Cancel;
            }
            let mut finished = s_clone.lock().unwrap();
            *finished = true;
        });
        
        let cancel_action = create_alert_action("Cancel", UIAlertActionStyle::Cancel, 0, alert, context);
        let ok_action = create_alert_action("OK", UIAlertActionStyle::Default, 1, alert, context);
        
        let add_action_sel = sel_registerName("addAction:\0".as_ptr() as *const c_char);
        objc_msgSend(alert, add_action_sel, cancel_action);
        objc_msgSend(alert, add_action_sel, ok_action);
        
        // Set preferred action based on default
        if default == OkCancel::Ok {
            let set_preferred_action_sel = sel_registerName("setPreferredAction:\0".as_ptr() as *const c_char);
            objc_msgSend(alert, set_preferred_action_sel, ok_action);
        }
        
        run_alert_controller(alert, |_| {});
        
        // Wait for completion
        loop {
            if *semaphore.lock().unwrap() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
    
    result
}

pub fn message_box_yes_no(msg_box: &MessageBox, default: YesNo) -> YesNo {
    let title = msg_box.dialog.title();
    let message = msg_box.dialog.message();
    let mut result = YesNo::No;
    
    let _pool = AutoreleasePool::new();
    
    unsafe {
        let controller_class: id = objc_getClass("UIAlertController\0".as_ptr() as *const c_char);
        let title_str = ns_string(title);
        let message_str = ns_string(message);
        
        let alert_with_title_sel = sel_registerName("alertControllerWithTitle:message:preferredStyle:\0".as_ptr() as *const c_char);
        let alert: id = objc_msgSend(
            controller_class, 
            alert_with_title_sel, 
            title_str, 
            message_str, 
            UIAlertControllerStyle::Alert as NSInteger
        );
        
        // Create a semaphore to wait for the alert completion
        let semaphore = std::sync::Arc::new(std::sync::Mutex::new(false));
        let s_clone = semaphore.clone();
        
        let context = AlertCallback::new(move |tag| {
            if tag == 1 {
                result = YesNo::Yes;
            } else {
                result = YesNo::No;
            }
            let mut finished = s_clone.lock().unwrap();
            *finished = true;
        });
        
        let no_action = create_alert_action("No", UIAlertActionStyle::Default, 0, alert, context);
        let yes_action = create_alert_action("Yes", UIAlertActionStyle::Default, 1, alert, context);
        
        let add_action_sel = sel_registerName("addAction:\0".as_ptr() as *const c_char);
        objc_msgSend(alert, add_action_sel, no_action);
        objc_msgSend(alert, add_action_sel, yes_action);
        
        // Set preferred action based on default
        let set_preferred_action_sel = sel_registerName("setPreferredAction:\0".as_ptr() as *const c_char);
        if default == YesNo::Yes {
            objc_msgSend(alert, set_preferred_action_sel, yes_action);
        } else {
            objc_msgSend(alert, set_preferred_action_sel, no_action);
        }
        
        run_alert_controller(alert, |_| {});
        
        // Wait for completion
        loop {
            if *semaphore.lock().unwrap() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
    
    result
}

pub fn message_box_yes_no_cancel(msg_box: &MessageBox, default: YesNoCancel) -> YesNoCancel {
    let title = msg_box.dialog.title();
    let message = msg_box.dialog.message();
    let mut result = YesNoCancel::Cancel;
    
    let _pool = AutoreleasePool::new();
    
    unsafe {
        let controller_class: id = objc_getClass("UIAlertController\0".as_ptr() as *const c_char);
        let title_str = ns_string(title);
        let message_str = ns_string(message);
        
        let alert_with_title_sel = sel_registerName("alertControllerWithTitle:message:preferredStyle:\0".as_ptr() as *const c_char);
        let alert: id = objc_msgSend(
            controller_class, 
            alert_with_title_sel, 
            title_str, 
            message_str, 
            UIAlertControllerStyle::Alert as NSInteger
        );
        
        // Create a semaphore to wait for the alert completion
        let semaphore = std::sync::Arc::new(std::sync::Mutex::new(false));
        let s_clone = semaphore.clone();
        
        let context = AlertCallback::new(move |tag| {
            result = match tag {
                1 => YesNoCancel::Yes,
                2 => YesNoCancel::No,
                _ => YesNoCancel::Cancel,
            };
            let mut finished = s_clone.lock().unwrap();
            *finished = true;
        });
        
        let cancel_action = create_alert_action("Cancel", UIAlertActionStyle::Cancel, 0, alert, context);
        let no_action = create_alert_action("No", UIAlertActionStyle::Default, 2, alert, context);
        let yes_action = create_alert_action("Yes", UIAlertActionStyle::Default, 1, alert, context);
        
        let add_action_sel = sel_registerName("addAction:\0".as_ptr() as *const c_char);
        objc_msgSend(alert, add_action_sel, cancel_action);
        objc_msgSend(alert, add_action_sel, no_action);
        objc_msgSend(alert, add_action_sel, yes_action);
        
        // Set preferred action based on default
        let set_preferred_action_sel = sel_registerName("setPreferredAction:\0".as_ptr() as *const c_char);
        match default {
            YesNoCancel::Yes => objc_msgSend(alert, set_preferred_action_sel, yes_action),
            YesNoCancel::No => objc_msgSend(alert, set_preferred_action_sel, no_action),
            YesNoCancel::Cancel => { /* Default is cancel */ }
        }
        
        run_alert_controller(alert, |_| {});
        
        // Wait for completion
        loop {
            if *semaphore.lock().unwrap() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
    
    result
}

pub fn input_box(input: &InputBox) -> Option<String> {
    let title = input.dialog.title();
    let message = input.dialog.message();
    let default_value = input.default_value().unwrap_or("");
    let is_password = input.is_password();
    let mut result: Option<String> = None;
    
    let _pool = AutoreleasePool::new();
    
    unsafe {
        let controller_class: id = objc_getClass("UIAlertController\0".as_ptr() as *const c_char);
        let title_str = ns_string(title);
        let message_str = ns_string(message);
        
        let alert_with_title_sel = sel_registerName("alertControllerWithTitle:message:preferredStyle:\0".as_ptr() as *const c_char);
        let alert: id = objc_msgSend(
            controller_class, 
            alert_with_title_sel, 
            title_str, 
            message_str, 
            UIAlertControllerStyle::Alert as NSInteger
        );
        
        // Add text field
        let add_text_field_sel = sel_registerName("addTextFieldWithConfigurationHandler:\0".as_ptr() as *const c_char);
        
        // Configuration block for text field
        extern "C" fn config_text_field(text_field: id, default_value: *mut c_void, is_pwd: BOOL) {
            unsafe {
                let default_str = default_value as id;
                let set_text_sel = sel_registerName("setText:\0".as_ptr() as *const c_char);
                objc_msgSend(text_field, set_text_sel, default_str);
                
                if is_pwd == YES {
                    let set_secure_sel = sel_registerName("setSecureTextEntry:\0".as_ptr() as *const c_char);
                    objc_msgSend(text_field, set_secure_sel, YES);
                }
            }
        }
        
        let default_str = ns_string(default_value);
        objc_msgSend(
            alert, 
            add_text_field_sel, 
            config_text_field as *mut c_void,
            default_str,
            if is_password { YES } else { NO }
        );
        
        // Create a semaphore to wait for the alert completion
        let semaphore = std::sync::Arc::new(std::sync::Mutex::new(false));
        let s_clone = semaphore.clone();
        let result_container = std::sync::Arc::new(std::sync::Mutex::new(None::<String>));
        let r_clone = result_container.clone();
        
        let context = AlertCallback::new(move |tag| {
            if tag == 1 {
                // Get the text field value
                let textfields_sel = sel_registerName("textFields\0".as_ptr() as *const c_char);
                let textfields: id = objc_msgSend(alert, textfields_sel);
                
                let count_sel = sel_registerName("count\0".as_ptr() as *const c_char);
                let count: NSUInteger = objc_msgSend(textfields, count_sel) as NSUInteger;
                
                if count > 0 {
                    let object_at_idx_sel = sel_registerName("objectAtIndex:\0".as_ptr() as *const c_char);
                    let textfield: id = objc_msgSend(textfields, object_at_idx_sel, 0);
                    
                    let text_sel = sel_registerName("text\0".as_ptr() as *const c_char);
                    let text: id = objc_msgSend(textfield, text_sel);
                    
                    if text != ptr::null_mut() {
                        let utf8_sel = sel_registerName("UTF8String\0".as_ptr() as *const c_char);
                        let utf8_str = objc_msgSend(text, utf8_sel) as *const c_char;
                        let text_str = std::ffi::CStr::from_ptr(utf8_str).to_string_lossy().into_owned();
                        
                        let mut result = r_clone.lock().unwrap();
                        *result = Some(text_str);
                    }
                }
            }
            
            let mut finished = s_clone.lock().unwrap();
            *finished = true;
        });
        
        let cancel_action = create_alert_action("Cancel", UIAlertActionStyle::Cancel, 0, alert, context);
        let ok_action = create_alert_action("OK", UIAlertActionStyle::Default, 1, alert, context);
        
        let add_action_sel = sel_registerName("addAction:\0".as_ptr() as *const c_char);
        objc_msgSend(alert, add_action_sel, cancel_action);
        objc_msgSend(alert, add_action_sel, ok_action);
        
        run_alert_controller(alert, |_| {});
        
        // Wait for completion
        loop {
            if *semaphore.lock().unwrap() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        
        // Get the result
        result = result_container.lock().unwrap().clone();
    }
    
    result
}

// iOS doesn't allow direct file system access - returning None for file dialogs

pub fn save_file_dialog(_dialog: &FileDialog) -> Option<String> {
    // iOS doesn't allow direct access to file system
    None
}

pub fn open_file_dialog(_dialog: &FileDialog) -> Option<Vec<String>> {
    // iOS doesn't allow direct access to file system
    None
}

pub fn select_folder_dialog(_dialog: &FileDialog) -> Option<String> {
    // iOS doesn't allow direct access to file system
    None
}

pub fn color_chooser_dialog(_chooser: &ColorChooser) -> Option<(String, [u8; 3])> {
    // iOS doesn't have a built-in color picker before iOS 14
    None
}

pub fn notification(notification: &Notification) -> bool {
    let title = notification.title();
    let message = notification.message();
    
    let _pool = AutoreleasePool::new();
    
    // For iOS, we show an alert for the notification
    unsafe {
        let controller_class: id = objc_getClass("UIAlertController\0".as_ptr() as *const c_char);
        let title_str = ns_string(title);
        let message_str = ns_string(message);
        
        let alert_with_title_sel = sel_registerName("alertControllerWithTitle:message:preferredStyle:\0".as_ptr() as *const c_char);
        let alert: id = objc_msgSend(
            controller_class, 
            alert_with_title_sel, 
            title_str, 
            message_str, 
            UIAlertControllerStyle::Alert as NSInteger
        );
        
        let context = AlertCallback::new(|_| {});
        let ok_action = create_alert_action("OK", UIAlertActionStyle::Default, 1, alert, context);
        
        let add_action_sel = sel_registerName("addAction:\0".as_ptr() as *const c_char);
        objc_msgSend(alert, add_action_sel, ok_action);
        
        run_alert_controller(alert, |_| {});
    }
    
    true
}