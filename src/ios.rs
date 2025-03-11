use super::*;
use objc2::{ClassType, msg_send_id, msg_send, sel};
use objc2::runtime::{Bool, Object};
use objc2_foundation::{NSArray, NSAutoreleasePool, NSObject, NSString};
use objc2_uikit::{UIAlertAction, UIAlertController, UIApplication, UIColor, UIDevice, UIViewController};

// Helper function to get root view controller
fn get_root_view_controller() -> Option<*mut Object> {
    unsafe {
        let app: *mut Object = msg_send![UIApplication::sharedApplication(), delegate];
        let window: *mut Object = msg_send![app, window];
        let root_vc: *mut Object = msg_send![window, rootViewController];
        
        if root_vc.is_null() {
            None
        } else {
            Some(root_vc)
        }
    }
}

// Helper to convert RGB components to UIColor
fn rgb_to_uicolor(r: u8, g: u8, b: u8) -> *mut Object {
    let r_float = r as f64 / 255.0;
    let g_float = g as f64 / 255.0;
    let b_float = b as f64 / 255.0;
    
    unsafe {
        msg_send![UIColor::class(), colorWithRed:r_float green:g_float blue:b_float alpha:1.0]
    }
}

// Helper to run UIAlertController and wait for completion
fn run_alert_controller<F>(controller: *mut Object, completion: F) 
where F: FnOnce(*mut Object) + Send + 'static {
    let semaphore = std::sync::Arc::new(std::sync::Mutex::new(false));
    let result_container = std::sync::Arc::new(std::sync::Mutex::new(None::<*mut Object>));
    
    let s_clone = semaphore.clone();
    let r_clone = result_container.clone();
    
    unsafe {
        let root_vc = match get_root_view_controller() {
            Some(vc) => vc,
            None => return completion(std::ptr::null_mut()),
        };
        
        let queue = dispatch_get_main_queue();
        
        dispatch_async(queue, move || {
            let _pool = NSAutoreleasePool::new();
            
            msg_send![root_vc, presentViewController:controller animated:true completion:nil];
            
            let action_handler = move |action: *mut Object| {
                let mut result = r_clone.lock().unwrap();
                *result = Some(action);
                let mut finished = s_clone.lock().unwrap();
                *finished = true;
            };
            
            // Store action handler
            objc_set_associated_object(controller, "action_handler", Box::new(action_handler));
        });
    }
    
    // Wait for completion
    loop {
        if *semaphore.lock().unwrap() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    
    // Get result and call completion
    let result = result_container.lock().unwrap().unwrap_or(std::ptr::null_mut());
    completion(result);
}

// Create UIAlertAction with handler
fn create_alert_action(title: &str, style: u64, tag: u64) -> *mut Object {
    unsafe {
        let title_str = NSString::from_str(title);
        let action: *mut Object = msg_send_id![UIAlertAction::class(), 
            actionWithTitle:title_str
            style:style
            handler:^(action: *mut Object) {
                // Set user tag on the action for identification
                objc_set_associated_object(action, "user_tag", Box::new(tag));
                
                // Get controller and call stored handler
                let controller = objc_get_associated_object::<*mut Object>(action, "controller");
                if let Some(controller) = controller {
                    let handler = objc_get_associated_object::<Box<dyn Fn(*mut Object)>>(controller, "action_handler");
                    if let Some(handler) = handler {
                        handler(action);
                    }
                }
            }
        ];
        
        action
    }
}

// Get tag from action
fn get_action_tag(action: *mut Object) -> Option<u64> {
    unsafe {
        objc_get_associated_object::<u64>(action, "user_tag")
    }
}

pub fn message_box_ok(msg_box: &MessageBox) {
    let title = msg_box.dialog.title();
    let message = msg_box.dialog.message();
    
    unsafe {
        let _pool = NSAutoreleasePool::new();
        
        let alert = UIAlertController::alertControllerWithTitle_message_preferredStyle(
            NSString::from_str(title),
            NSString::from_str(message),
            0 // UIAlertControllerStyleAlert
        );
        
        let ok_action = create_alert_action("OK", 0, 1); // UIAlertActionStyleDefault
        msg_send![alert, addAction:ok_action];
        
        run_alert_controller(alert, |_| {});
    }
}

pub fn message_box_ok_cancel(msg_box: &MessageBox, default: OkCancel) -> OkCancel {
    let title = msg_box.dialog.title();
    let message = msg_box.dialog.message();
    let mut result = OkCancel::Cancel;
    
    unsafe {
        let _pool = NSAutoreleasePool::new();
        
        let alert = UIAlertController::alertControllerWithTitle_message_preferredStyle(
            NSString::from_str(title),
            NSString::from_str(message),
            0 // UIAlertControllerStyleAlert
        );
        
        let cancel_action = create_alert_action("Cancel", 1, 0); // UIAlertActionStyleCancel
        let ok_action = create_alert_action("OK", 0, 1); // UIAlertActionStyleDefault
        
        msg_send![alert, addAction:cancel_action];
        msg_send![alert, addAction:ok_action];
        
        // Set preferred action based on default
        if default == OkCancel::Ok {
            msg_send![alert, setPreferredAction:ok_action];
        }
        
        run_alert_controller(alert, |action| {
            if let Some(tag) = get_action_tag(action) {
                if tag == 1 {
                    result = OkCancel::Ok;
                }
            }
        });
    }
    
    result
}

pub fn message_box_yes_no(msg_box: &MessageBox, default: YesNo) -> YesNo {
    let title = msg_box.dialog.title();
    let message = msg_box.dialog.message();
    let mut result = YesNo::No;
    
    unsafe {
        let _pool = NSAutoreleasePool::new();
        
        let alert = UIAlertController::alertControllerWithTitle_message_preferredStyle(
            NSString::from_str(title),
            NSString::from_str(message),
            0 // UIAlertControllerStyleAlert
        );
        
        let no_action = create_alert_action("No", 0, 0); // UIAlertActionStyleDefault
        let yes_action = create_alert_action("Yes", 0, 1); // UIAlertActionStyleDefault
        
        msg_send![alert, addAction:no_action];
        msg_send![alert, addAction:yes_action];
        
        // Set preferred action based on default
        if default == YesNo::Yes {
            msg_send![alert, setPreferredAction:yes_action];
        } else {
            msg_send![alert, setPreferredAction:no_action];
        }
        
        run_alert_controller(alert, |action| {
            if let Some(tag) = get_action_tag(action) {
                if tag == 1 {
                    result = YesNo::Yes;
                }
            }
        });
    }
    
    result
}

pub fn message_box_yes_no_cancel(msg_box: &MessageBox, default: YesNoCancel) -> YesNoCancel {
    let title = msg_box.dialog.title();
    let message = msg_box.dialog.message();
    let mut result = YesNoCancel::Cancel;
    
    unsafe {
        let _pool = NSAutoreleasePool::new();
        
        let alert = UIAlertController::alertControllerWithTitle_message_preferredStyle(
            NSString::from_str(title),
            NSString::from_str(message),
            0 // UIAlertControllerStyleAlert
        );
        
        let cancel_action = create_alert_action("Cancel", 1, 0); // UIAlertActionStyleCancel
        let no_action = create_alert_action("No", 0, 2); // UIAlertActionStyleDefault
        let yes_action = create_alert_action("Yes", 0, 1); // UIAlertActionStyleDefault
        
        msg_send![alert, addAction:cancel_action];
        msg_send![alert, addAction:no_action];
        msg_send![alert, addAction:yes_action];
        
        // Set preferred action based on default
        match default {
            YesNoCancel::Yes => msg_send![alert, setPreferredAction:yes_action],
            YesNoCancel::No => msg_send![alert, setPreferredAction:no_action],
            YesNoCancel::Cancel => {}
        }
        
        run_alert_controller(alert, |action| {
            if let Some(tag) = get_action_tag(action) {
                result = match tag {
                    1 => YesNoCancel::Yes,
                    2 => YesNoCancel::No,
                    _ => YesNoCancel::Cancel,
                };
            }
        });
    }
    
    result
}

pub fn input_box(input: &InputBox) -> Option<String> {
    let title = input.dialog.title();
    let message = input.dialog.message();
    let default_value = input.default_value().unwrap_or("");
    let is_password = input.is_password();
    let mut result: Option<String> = None;
    
    unsafe {
        let _pool = NSAutoreleasePool::new();
        
        let alert = UIAlertController::alertControllerWithTitle_message_preferredStyle(
            NSString::from_str(title),
            NSString::from_str(message),
            0 // UIAlertControllerStyleAlert
        );
        
        // Add text field
        msg_send![alert, addTextFieldWithConfigurationHandler:^(textField: *mut Object) {
            msg_send![textField, setText:NSString::from_str(default_value)];
            
            if is_password {
                msg_send![textField, setSecureTextEntry:Bool::YES];
            }
        }];
        
        let cancel_action = create_alert_action("Cancel", 1, 0); // UIAlertActionStyleCancel
        let ok_action = create_alert_action("OK", 0, 1); // UIAlertActionStyleDefault
        
        msg_send![alert, addAction:cancel_action];
        msg_send![alert, addAction:ok_action];
        
        run_alert_controller(alert, |action| {
            if let Some(tag) = get_action_tag(action) {
                if tag == 1 {
                    let textfields: *mut NSArray = msg_send![alert, textFields];
                    let count: usize = msg_send![textfields, count];
                    
                    if count > 0 {
                        let textfield: *mut Object = msg_send![textfields, objectAtIndex:0];
                        let text: *mut NSString = msg_send![textfield, text];
                        result = Some(text.to_string());
                    }
                }
            }
        });
    }
    
    result
}

// For file operations, iOS is very restrictive and requires document picker controller
// These will provide limited functionality in iOS

pub fn save_file_dialog(dialog: &FileDialog) -> Option<String> {
    // iOS doesn't allow direct access to file system
    // Return None for iOS - a proper implementation would require
    // UIDocumentPickerViewController with UIDocumentSaveMode
    None
}

pub fn open_file_dialog(dialog: &FileDialog) -> Option<Vec<String>> {
    // iOS doesn't allow direct access to file system
    // Return None for iOS - a proper implementation would require
    // UIDocumentPickerViewController with UIDocumentPickerMode
    None
}

pub fn select_folder_dialog(dialog: &FileDialog) -> Option<String> {
    // iOS doesn't allow direct access to file system
    // Return None for iOS - a proper implementation would require
    // UIDocumentPickerViewController for directories
    None
}

pub fn color_chooser_dialog(chooser: &ColorChooser) -> Option<(String, [u8; 3])> {
    // iOS doesn't have a built-in color picker before iOS 14
    // This would require a custom implementation
    // Return None for now
    None
}

pub fn notification(notification: &Notification) -> bool {
    let title = notification.title();
    let message = notification.message();
    
    unsafe {
        let _pool = NSAutoreleasePool::new();
        
        // For iOS, we need UNUserNotificationCenter, but for simplicity
        // we'll just show an alert for this example
        let alert = UIAlertController::alertControllerWithTitle_message_preferredStyle(
            NSString::from_str(title),
            NSString::from_str(message),
            0 // UIAlertControllerStyleAlert
        );
        
        let ok_action = create_alert_action("OK", 0, 1);
        msg_send![alert, addAction:ok_action];
        
        run_alert_controller(alert, |_| {});
    }
    
    true
}