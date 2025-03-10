use super::*;
use std::process::Command;

pub fn message_box_ok(title: &str, message: &str, icon: MessageBoxIcon) {
    let icon_name = match icon {
        MessageBoxIcon::Info => "note",
        MessageBoxIcon::Warning => "caution",
        MessageBoxIcon::Error => "stop",
        MessageBoxIcon::Question => "note",
    };
    
    let script = format!(
        "osascript -e 'tell app \"System Events\" to display dialog \"{}\" with title \"{}\" buttons {{\"OK\"}} with icon {}'",
        message.replace("\"", "\\\""),
        title.replace("\"", "\\\""),
        icon_name
    );
    
    let _ = Command::new("sh")
        .arg("-c")
        .arg(&script)
        .status();
}

pub fn message_box_ok_cancel(title: &str, message: &str, icon: MessageBoxIcon, default: OkCancel) -> OkCancel {
    let icon_name = match icon {
        MessageBoxIcon::Info => "note",
        MessageBoxIcon::Warning => "caution",
        MessageBoxIcon::Error => "stop",
        MessageBoxIcon::Question => "note",
    };
    
    let default_button = match default {
        OkCancel::Ok => "OK",
        OkCancel::Cancel => "Cancel",
    };
    
    let script = format!(
        "osascript -e 'tell app \"System Events\" to display dialog \"{}\" with title \"{}\" buttons {{\"Cancel\", \"OK\"}} default button \"{}\" with icon {}'",
        message.replace("\"", "\\\""),
        title.replace("\"", "\\\""),
        default_button,
        icon_name
    );
    
    let output = Command::new("sh")
        .arg("-c")
        .arg(&script)
        .output();
        
    match output {
        Ok(out) => {
            if out.status.success() {
                OkCancel::Ok
            } else {
                OkCancel::Cancel
            }
        },
        Err(_) => OkCancel::Cancel,
    }
}

pub fn message_box_yes_no(title: &str, message: &str, icon: MessageBoxIcon, default: YesNo) -> YesNo {
    let icon_name = match icon {
        MessageBoxIcon::Info => "note",
        MessageBoxIcon::Warning => "caution",
        MessageBoxIcon::Error => "stop",
        MessageBoxIcon::Question => "note",
    };
    
    let default_button = match default {
        YesNo::Yes => "Yes",
        YesNo::No => "No",
    };
    
    let script = format!(
        "osascript -e 'tell app \"System Events\" to display dialog \"{}\" with title \"{}\" buttons {{\"No\", \"Yes\"}} default button \"{}\" with icon {}'",
        message.replace("\"", "\\\""),
        title.replace("\"", "\\\""),
        default_button,
        icon_name
    );
    
    let output = Command::new("sh")
        .arg("-c")
        .arg(&script)
        .output();
        
    match output {
        Ok(out) => {
            if out.status.success() {
                YesNo::Yes
            } else {
                YesNo::No
            }
        },
        Err(_) => YesNo::No,
    }
}

pub fn message_box_yes_no_cancel(title: &str, message: &str, icon: MessageBoxIcon, default: YesNoCancel) -> YesNoCancel {
    let icon_name = match icon {
        MessageBoxIcon::Info => "note",
        MessageBoxIcon::Warning => "caution",
        MessageBoxIcon::Error => "stop",
        MessageBoxIcon::Question => "note",
    };
    
    let default_button = match default {
        YesNoCancel::Yes => "Yes",
        YesNoCancel::No => "No",
        YesNoCancel::Cancel => "Cancel",
    };
    
    let script = format!(
        "osascript -e 'tell app \"System Events\" to display dialog \"{}\" with title \"{}\" buttons {{\"Cancel\", \"No\", \"Yes\"}} default button \"{}\" with icon {}'",
        message.replace("\"", "\\\""),
        title.replace("\"", "\\\""),
        default_button,
        icon_name
    );
    
    let output = Command::new("sh")
        .arg("-c")
        .arg(&script)
        .output();
        
    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            if stdout.contains("button returned:Yes") {
                YesNoCancel::Yes
            } else if stdout.contains("button returned:No") {
                YesNoCancel::No
            } else {
                YesNoCancel::Cancel
            }
        },
        Err(_) => YesNoCancel::Cancel,
    }
}

pub fn input_box(title: &str, message: &str, default: Option<&str>) -> Option<String> {
    let default_text = default.unwrap_or("");
    let hidden = default.is_none();
    
    let script = if hidden {
        format!(
            "osascript -e 'tell app \"System Events\" to display dialog \"{}\" with title \"{}\" default answer \"{}\" with hidden answer'",
            message.replace("\"", "\\\""),
            title.replace("\"", "\\\""),
            default_text.replace("\"", "\\\"")
        )
    } else {
        format!(
            "osascript -e 'tell app \"System Events\" to display dialog \"{}\" with title \"{}\" default answer \"{}\"'",
            message.replace("\"", "\\\""),
            title.replace("\"", "\\\""),
            default_text.replace("\"", "\\\"")
        )
    };
    
    let output = Command::new("sh")
        .arg("-c")
        .arg(&script)
        .output();
        
    match output {
        Ok(out) => {
            if out.status.success() {
                let stdout = String::from_utf8_lossy(&out.stdout);
                if let Some(idx) = stdout.find("text returned:") {
                    let start = idx + 13; // Length of "text returned:"
                    Some(stdout[start..].trim().to_string())
                } else {
                    None
                }
            } else {
                None
            }
        },
        Err(_) => None,
    }
}

pub fn save_file_dialog(title: &str, path: &str, filter_patterns: &[&str], description: &str) -> Option<String> {
    let filters = if !filter_patterns.is_empty() {
        format!("of type {{\"{}\" as list}}", filter_patterns.join("\", \""))
    } else {
        String::new()
    };
    
    let default_path = if !path.is_empty() {
        format!("default location \"{}\"", path.replace("\"", "\\\""))
    } else {
        String::new()
    };
    
    let script = format!(
        "osascript -e 'tell app \"System Events\" to POSIX path of (choose file name with prompt \"{}\" {} {})'",
        title.replace("\"", "\\\""),
        default_path,
        filters
    );
    
    let output = Command::new("sh")
        .arg("-c")
        .arg(&script)
        .output();
        
    match output {
        Ok(out) => {
            if out.status.success() {
                let stdout = String::from_utf8_lossy(&out.stdout);
                Some(stdout.trim().to_string())
            } else {
                None
            }
        },
        Err(_) => None,
    }
}

pub fn open_file_dialog(title: &str, path: &str, filter_patterns: &[&str], description: &str, 
                    allow_multi: bool) -> Option<Vec<String>> {
    let filters = if !filter_patterns.is_empty() {
        format!("of type {{\"{}\" as list}}", filter_patterns.join("\", \""))
    } else {
        String::new()
    };
    
    let default_path = if !path.is_empty() {
        format!("default location \"{}\"", path.replace("\"", "\\\""))
    } else {
        String::new()
    };
    
    let multiple = if allow_multi { "with multiple selections allowed" } else { "" };
    
    let script = format!(
        "osascript -e 'tell app \"System Events\" to set theFiles to choose file with prompt \"{}\" {} {} {}' -e 'set result to \"\"' -e 'repeat with aFile in theFiles' -e 'set result to result & POSIX path of aFile & \"|\"' -e 'end repeat' -e 'return result'",
        title.replace("\"", "\\\""),
        default_path,
        filters,
        multiple
    );
    
    let output = Command::new("sh")
        .arg("-c")
        .arg(&script)
        .output();
        
    match output {
        Ok(out) => {
            if out.status.success() {
                let stdout = String::from_utf8_lossy(&out.stdout);
                let files: Vec<String> = stdout.trim_end_matches('|').split('|')
                    .map(|s| s.to_string())
                    .collect();
                if files.is_empty() { None } else { Some(files) }
            } else {
                None
            }
        },
        Err(_) => None,
    }
}

pub fn select_folder_dialog(title: &str, path: &str) -> Option<String> {
    let default_path = if !path.is_empty() {
        format!("default location \"{}\"", path.replace("\"", "\\\""))
    } else {
        String::new()
    };
    
    let script = format!(
        "osascript -e 'tell app \"System Events\" to POSIX path of (choose folder with prompt \"{}\" {})'",
        title.replace("\"", "\\\""),
        default_path
    );
    
    let output = Command::new("sh")
        .arg("-c")
        .arg(&script)
        .output();
        
    match output {
        Ok(out) => {
            if out.status.success() {
                let stdout = String::from_utf8_lossy(&out.stdout);
                Some(stdout.trim().to_string())
            } else {
                None
            }
        },
        Err(_) => None,
    }
}

pub fn color_chooser_dialog(title: &str, default: DefaultColorValue) -> Option<(String, [u8; 3])> {
    let default_rgb = match default {
        DefaultColorValue::Hex(hex) => super::hex_to_rgb(hex),
        DefaultColorValue::RGB(rgb) => *rgb,
    };
    
    let script = format!(
        "osascript -e 'tell app \"System Events\" to choose color default color {{{}, {}, {}}}' -e 'set r to (item 1 of result) / 65535 * 255' -e 'set g to (item 2 of result) / 65535 * 255' -e 'set b to (item 3 of result) / 65535 * 255' -e 'return (r as integer) & \" \" & (g as integer) & \" \" & (b as integer)'",
        (default_rgb[0] as f32 / 255.0 * 65535.0) as u32,
        (default_rgb[1] as f32 / 255.0 * 65535.0) as u32,
        (default_rgb[2] as f32 / 255.0 * 65535.0) as u32
    );
    
    let output = Command::new("sh")
        .arg("-c")
        .arg(&script)
        .output();
        
    match output {
        Ok(out) => {
            if out.status.success() {
                let stdout = String::from_utf8_lossy(&out.stdout);
                let parts: Vec<&str> = stdout.trim().split(' ').collect();
                
                if parts.len() >= 3 {
                    let r = parts[0].parse::<u8>().unwrap_or(0);
                    let g = parts[1].parse::<u8>().unwrap_or(0);
                    let b = parts[2].parse::<u8>().unwrap_or(0);
                    
                    let rgb = [r, g, b];
                    let hex = super::rgb_to_hex(&rgb);
                    
                    Some((hex, rgb))
                } else {
                    None
                }
            } else {
                None
            }
        },
        Err(_) => None,
    }
}
