use super::*;
use std::ffi::{CStr, CString};
use std::path::Path;
use std::sync::{Arc, Mutex};
use jni::JNIEnv;
use jni::objects::{JClass, JObject, JString, JValue};
use jni::sys::{jint, jobject, jobjectArray, jsize, jstring};

pub const ANDROID_HELPER_CLASS: &str = include_str!("com.tfd.DialogHelper.java");

thread_local! {
    // Store JNI environment for this thread
    static JNI_ENV: Mutex<Option<Arc<JNIEnv<'static>>>> = Mutex::new(None);
    
    // Activity reference
    static CURRENT_ACTIVITY: Mutex<Option<JObject<'static>>> = Mutex::new(None);
}

// Initialize JNI for the current thread
fn init_jni() -> bool {
    JNI_ENV.with(|env| {
        let mut env_guard = env.lock().unwrap();
        if env_guard.is_none() {
            // In a real implementation, you'd get this from context
            // For now, assume it's initialized externally and cached in thread local
            false
        } else {
            true
        }
    })
}

// Get JNI environment
fn get_env() -> Option<Arc<JNIEnv<'static>>> {
    JNI_ENV.with(|env| {
        env.lock().unwrap().clone()
    })
}

// Get current activity
fn get_activity() -> Option<JObject<'static>> {
    CURRENT_ACTIVITY.with(|activity| {
        activity.lock().unwrap().clone()
    })
}

// Convert Rust string to Java string
fn to_jstring(env: &JNIEnv, s: &str) -> jstring {
    let cstr = CString::new(s).unwrap();
    let jstr = env.new_string(cstr.to_str().unwrap()).unwrap();
    jstr.into_raw()
}

// Convert Java string to Rust string
fn from_jstring(env: &JNIEnv, jstr: jstring) -> String {
    let java_str = unsafe { JString::from_raw(jstr) };
    env.get_string(&java_str).unwrap().into()
}

pub fn message_box_ok(msg_box: &MessageBox) {
    let title = msg_box.dialog.title();
    let message = msg_box.dialog.message();
    let icon = msg_box.icon();
    
    if !init_jni() {
        return;
    }
    
    let env = match get_env() {
        Some(env) => env,
        None => return,
    };
    
    let activity = match get_activity() {
        Some(activity) => activity,
        None => return,
    };
    
    // Get icon resource ID based on icon type
    let icon_res_id = match icon {
        MessageBoxIcon::Info => 3,     // android.R.drawable.ic_dialog_info
        MessageBoxIcon::Warning => 1,  // android.R.drawable.ic_dialog_alert
        MessageBoxIcon::Error => 1,    // android.R.drawable.ic_dialog_alert
        MessageBoxIcon::Question => 3, // android.R.drawable.ic_dialog_info
    };
    
    // Call Android AlertDialog.Builder
    let result = env.call_static_method(
        "com/example/tinyfiledialogs/DialogHelper",
        "showMessageBox",
        "(Landroid/app/Activity;Ljava/lang/String;Ljava/lang/String;I)V",
        &[
            JValue::Object(activity),
            JValue::Object(JObject::from(to_jstring(&env, title))),
            JValue::Object(JObject::from(to_jstring(&env, message))),
            JValue::Int(icon_res_id),
        ],
    );
    
    if result.is_err() {
        // Handle error
        eprintln!("Failed to show message box: {:?}", result.err());
    }
}

pub fn message_box_ok_cancel(msg_box: &MessageBox, default: OkCancel) -> OkCancel {
    let title = msg_box.dialog.title();
    let message = msg_box.dialog.message();
    let icon = msg_box.icon();
    
    if !init_jni() {
        return OkCancel::Cancel;
    }
    
    let env = match get_env() {
        Some(env) => env,
        None => return OkCancel::Cancel,
    };
    
    let activity = match get_activity() {
        Some(activity) => activity,
        None => return OkCancel::Cancel,
    };
    
    // Get icon resource ID
    let icon_res_id = match icon {
        MessageBoxIcon::Info => 3,
        MessageBoxIcon::Warning => 1,
        MessageBoxIcon::Error => 1,
        MessageBoxIcon::Question => 3,
    };
    
    // Convert default to int (0 = Cancel, 1 = OK)
    let default_int = match default {
        OkCancel::Ok => 1,
        OkCancel::Cancel => 0,
    };
    
    // Call Android AlertDialog.Builder with result
    let result = env.call_static_method(
        "com/example/tinyfiledialogs/DialogHelper",
        "showOkCancelDialog",
        "(Landroid/app/Activity;Ljava/lang/String;Ljava/lang/String;II)I",
        &[
            JValue::Object(activity),
            JValue::Object(JObject::from(to_jstring(&env, title))),
            JValue::Object(JObject::from(to_jstring(&env, message))),
            JValue::Int(icon_res_id),
            JValue::Int(default_int),
        ],
    );
    
    match result {
        Ok(jvalue) => {
            let result_int = jvalue.i().unwrap_or(0);
            if result_int == 1 {
                OkCancel::Ok
            } else {
                OkCancel::Cancel
            }
        }
        Err(_) => OkCancel::Cancel,
    }
}

pub fn message_box_yes_no(msg_box: &MessageBox, default: YesNo) -> YesNo {
    let title = msg_box.dialog.title();
    let message = msg_box.dialog.message();
    let icon = msg_box.icon();
    
    if !init_jni() {
        return YesNo::No;
    }
    
    let env = match get_env() {
        Some(env) => env,
        None => return YesNo::No,
    };
    
    let activity = match get_activity() {
        Some(activity) => activity,
        None => return YesNo::No,
    };
    
    // Get icon resource ID
    let icon_res_id = match icon {
        MessageBoxIcon::Info => 3,
        MessageBoxIcon::Warning => 1,
        MessageBoxIcon::Error => 1,
        MessageBoxIcon::Question => 3,
    };
    
    // Convert default to int (0 = No, 1 = Yes)
    let default_int = match default {
        YesNo::Yes => 1,
        YesNo::No => 0,
    };
    
    // Call Android AlertDialog.Builder with result
    let result = env.call_static_method(
        "com/example/tinyfiledialogs/DialogHelper",
        "showYesNoDialog",
        "(Landroid/app/Activity;Ljava/lang/String;Ljava/lang/String;II)I",
        &[
            JValue::Object(activity),
            JValue::Object(JObject::from(to_jstring(&env, title))),
            JValue::Object(JObject::from(to_jstring(&env, message))),
            JValue::Int(icon_res_id),
            JValue::Int(default_int),
        ],
    );
    
    match result {
        Ok(jvalue) => {
            let result_int = jvalue.i().unwrap_or(0);
            if result_int == 1 {
                YesNo::Yes
            } else {
                YesNo::No
            }
        }
        Err(_) => YesNo::No,
    }
}

pub fn message_box_yes_no_cancel(msg_box: &MessageBox, default: YesNoCancel) -> YesNoCancel {
    let title = msg_box.dialog.title();
    let message = msg_box.dialog.message();
    let icon = msg_box.icon();
    
    if !init_jni() {
        return YesNoCancel::Cancel;
    }
    
    let env = match get_env() {
        Some(env) => env,
        None => return YesNoCancel::Cancel,
    };
    
    let activity = match get_activity() {
        Some(activity) => activity,
        None => return YesNoCancel::Cancel,
    };
    
    // Get icon resource ID
    let icon_res_id = match icon {
        MessageBoxIcon::Info => 3,
        MessageBoxIcon::Warning => 1,
        MessageBoxIcon::Error => 1,
        MessageBoxIcon::Question => 3,
    };
    
    // Convert default to int (0 = Cancel, 1 = Yes, 2 = No)
    let default_int = match default {
        YesNoCancel::Yes => 1,
        YesNoCancel::No => 2,
        YesNoCancel::Cancel => 0,
    };
    
    // Call Android AlertDialog.Builder with result
    let result = env.call_static_method(
        "com/example/tinyfiledialogs/DialogHelper",
        "showYesNoCancelDialog",
        "(Landroid/app/Activity;Ljava/lang/String;Ljava/lang/String;II)I",
        &[
            JValue::Object(activity),
            JValue::Object(JObject::from(to_jstring(&env, title))),
            JValue::Object(JObject::from(to_jstring(&env, message))),
            JValue::Int(icon_res_id),
            JValue::Int(default_int),
        ],
    );
    
    match result {
        Ok(jvalue) => {
            let result_int = jvalue.i().unwrap_or(0);
            match result_int {
                1 => YesNoCancel::Yes,
                2 => YesNoCancel::No,
                _ => YesNoCancel::Cancel,
            }
        }
        Err(_) => YesNoCancel::Cancel,
    }
}

pub fn input_box(input: &InputBox) -> Option<String> {
    let title = input.dialog.title();
    let message = input.dialog.message();
    let default_value = input.default_value().unwrap_or("");
    let is_password = input.is_password();
    
    if !init_jni() {
        return None;
    }
    
    let env = match get_env() {
        Some(env) => env,
        None => return None,
    };
    
    let activity = match get_activity() {
        Some(activity) => activity,
        None => return None,
    };
    
    // Call Android Dialog with EditText
    let result = env.call_static_method(
        "com/example/tinyfiledialogs/DialogHelper",
        "showInputDialog",
        "(Landroid/app/Activity;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Z)Ljava/lang/String;",
        &[
            JValue::Object(activity),
            JValue::Object(JObject::from(to_jstring(&env, title))),
            JValue::Object(JObject::from(to_jstring(&env, message))),
            JValue::Object(JObject::from(to_jstring(&env, default_value))),
            JValue::Bool(is_password as jni::sys::jboolean),
        ],
    );
    
    match result {
        Ok(jvalue) => {
            let java_string = jvalue.l().ok()?;
            if java_string.is_null() {
                None
            } else {
                Some(env.get_string(unsafe { JString::from_raw(java_string.into_raw()) }).ok()?.into())
            }
        }
        Err(_) => None,
    }
}

pub fn save_file_dialog(dialog: &FileDialog) -> Option<String> {
    let title = dialog.dialog.title();
    let path = dialog.path();
    let filter_patterns = dialog.filter_patterns();
    
    if !init_jni() {
        return None;
    }
    
    let env = match get_env() {
        Some(env) => env,
        None => return None,
    };
    
    let activity = match get_activity() {
        Some(activity) => activity,
        None => return None,
    };
    
    // Convert filter patterns to Java string array
    let filter_array = if !filter_patterns.is_empty() {
        let jstring_array = env.new_object_array(
            filter_patterns.len() as jsize,
            "java/lang/String",
            JObject::null(),
        ).ok()?;
        
        for (i, pattern) in filter_patterns.iter().enumerate() {
            let jstring = to_jstring(&env, pattern);
            env.set_object_array_element(jstring_array, i as jsize, JObject::from(jstring)).ok()?;
        }
        
        JObject::from(jstring_array)
    } else {
        JObject::null()
    };
    
    // Call Android to show file picker in SAVE mode
    let result = env.call_static_method(
        "com/example/tinyfiledialogs/DialogHelper",
        "showSaveFileDialog",
        "(Landroid/app/Activity;Ljava/lang/String;Ljava/lang/String;[Ljava/lang/String;)Ljava/lang/String;",
        &[
            JValue::Object(activity),
            JValue::Object(JObject::from(to_jstring(&env, title))),
            JValue::Object(JObject::from(to_jstring(&env, path))),
            JValue::Object(filter_array),
        ],
    );
    
    match result {
        Ok(jvalue) => {
            let java_string = jvalue.l().ok()?;
            if java_string.is_null() {
                None
            } else {
                Some(env.get_string(unsafe { JString::from_raw(java_string.into_raw()) }).ok()?.into())
            }
        }
        Err(_) => None,
    }
}

pub fn open_file_dialog(dialog: &FileDialog) -> Option<Vec<String>> {
    let title = dialog.dialog.title();
    let path = dialog.path();
    let filter_patterns = dialog.filter_patterns();
    let allow_multi = dialog.multiple_selection();
    
    if !init_jni() {
        return None;
    }
    
    let env = match get_env() {
        Some(env) => env,
        None => return None,
    };
    
    let activity = match get_activity() {
        Some(activity) => activity,
        None => return None,
    };
    
    // Convert filter patterns to Java string array
    let filter_array = if !filter_patterns.is_empty() {
        let jstring_array = env.new_object_array(
            filter_patterns.len() as jsize,
            "java/lang/String",
            JObject::null(),
        ).ok()?;
        
        for (i, pattern) in filter_patterns.iter().enumerate() {
            let jstring = to_jstring(&env, pattern);
            env.set_object_array_element(jstring_array, i as jsize, JObject::from(jstring)).ok()?;
        }
        
        JObject::from(jstring_array)
    } else {
        JObject::null()
    };
    
    // Call Android to show file picker in OPEN mode
    let result = env.call_static_method(
        "com/example/tinyfiledialogs/DialogHelper",
        "showOpenFileDialog",
        "(Landroid/app/Activity;Ljava/lang/String;Ljava/lang/String;[Ljava/lang/String;Z)[Ljava/lang/String;",
        &[
            JValue::Object(activity),
            JValue::Object(JObject::from(to_jstring(&env, title))),
            JValue::Object(JObject::from(to_jstring(&env, path))),
            JValue::Object(filter_array),
            JValue::Bool(allow_multi as jni::sys::jboolean),
        ],
    );
    
    match result {
        Ok(jvalue) => {
            let java_array = jvalue.l().ok()?;
            if java_array.is_null() {
                return None;
            }
            
            let array = unsafe { jobjectArray::from(java_array.into_raw()) };
            let length = env.get_array_length(array).ok()?;
            
            let mut files = Vec::with_capacity(length as usize);
            for i in 0..length {
                let jstr = env.get_object_array_element(array, i).ok()?;
                if !jstr.is_null() {
                    let string = env.get_string(unsafe { JString::from_raw(jstr.into_raw()) }).ok()?.into();
                    files.push(string);
                }
            }
            
            if files.is_empty() {
                None
            } else {
                Some(files)
            }
        }
        Err(_) => None,
    }
}

pub fn select_folder_dialog(dialog: &FileDialog) -> Option<String> {
    let title = dialog.dialog.title();
    let path = dialog.path();
    
    if !init_jni() {
        return None;
    }
    
    let env = match get_env() {
        Some(env) => env,
        None => return None,
    };
    
    let activity = match get_activity() {
        Some(activity) => activity,
        None => return None,
    };
    
    // Call Android to show folder picker
    let result = env.call_static_method(
        "com/example/tinyfiledialogs/DialogHelper",
        "showFolderDialog",
        "(Landroid/app/Activity;Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
        &[
            JValue::Object(activity),
            JValue::Object(JObject::from(to_jstring(&env, title))),
            JValue::Object(JObject::from(to_jstring(&env, path))),
        ],
    );
    
    match result {
        Ok(jvalue) => {
            let java_string = jvalue.l().ok()?;
            if java_string.is_null() {
                None
            } else {
                Some(env.get_string(unsafe { JString::from_raw(java_string.into_raw()) }).ok()?.into())
            }
        }
        Err(_) => None,
    }
}

pub fn color_chooser_dialog(chooser: &ColorChooser) -> Option<(String, [u8; 3])> {
    let title = chooser.dialog.title();
    
    let default_rgb = match chooser.default_color() {
        DefaultColorValue::Hex(hex) => super::hex_to_rgb(hex),
        DefaultColorValue::RGB(rgb) => *rgb,
    };
    
    if !init_jni() {
        return None;
    }
    
    let env = match get_env() {
        Some(env) => env,
        None => return None,
    };
    
    let activity = match get_activity() {
        Some(activity) => activity,
        None => return None,
    };
    
    // Call Android color picker
    let result = env.call_static_method(
        "com/example/tinyfiledialogs/DialogHelper",
        "showColorChooser",
        "(Landroid/app/Activity;Ljava/lang/String;III)[I",
        &[
            JValue::Object(activity),
            JValue::Object(JObject::from(to_jstring(&env, title))),
            JValue::Int(default_rgb[0] as jint),
            JValue::Int(default_rgb[1] as jint),
            JValue::Int(default_rgb[2] as jint),
        ],
    );
    
    match result {
        Ok(jvalue) => {
            let java_array = jvalue.l().ok()?;
            if java_array.is_null() {
                return None;
            }
            
            let array = env.get_int_array_elements(unsafe { 
                jni::sys::jintArray::from(java_array.into_raw())
            }, 0).ok()?;
            
            if array.len() >= 3 {
                let r = array[0] as u8;
                let g = array[1] as u8;
                let b = array[2] as u8;
                
                let rgb = [r, g, b];
                let hex = super::rgb_to_hex(&rgb);
                
                Some((hex, rgb))
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

pub fn notification(notification: &Notification) -> bool {
    let title = notification.title();
    let message = notification.message();
    let subtitle = notification.subtitle().unwrap_or("");
    
    if !init_jni() {
        return false;
    }
    
    let env = match get_env() {
        Some(env) => env,
        None => return false,
    };
    
    let activity = match get_activity() {
        Some(activity) => activity,
        None => return false,
    };
    
    // Call Android notification service
    let result = env.call_static_method(
        "com/example/tinyfiledialogs/DialogHelper",
        "showNotification",
        "(Landroid/app/Activity;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Z",
        &[
            JValue::Object(activity),
            JValue::Object(JObject::from(to_jstring(&env, title))),
            JValue::Object(JObject::from(to_jstring(&env, message))),
            JValue::Object(JObject::from(to_jstring(&env, subtitle))),
        ],
    );
    
    match result {
        Ok(jvalue) => jvalue.z().unwrap_or(false),
        Err(_) => false,
    }
}
