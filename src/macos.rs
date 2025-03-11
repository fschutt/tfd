use super::*;
use std::process::Command;
use std::path::Path;

// Helper function to run AppleScript and get the result
fn run_osascript(script: &str) -> Option<String> {
    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .ok()?;
    
    println!("output: {output:#?}");
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let result = stdout.trim_end().to_string();
        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    } else {
        None
    }
}

// Helper function to run multiple AppleScript commands
fn run_osascript_multi(scripts: &[&str]) -> Option<String> {
    let mut command = Command::new("osascript");
    
    for script in scripts {
        command.arg("-e").arg(script);
    }
    
    let output = command.output().ok()?;
    println!("output: {output:#?}");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let result = stdout.trim_end().to_string();
        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    } else {
        None
    }
}

// Convert path to POSIX format for AppleScript
fn to_posix_path(path: &str) -> String {
    if path.is_empty() {
        return String::new();
    }
    
    // If already has quotes, strip them
    let path = path.trim_matches('"');
    
    // Ensure the path is properly formatted for AppleScript
    if path.starts_with("alias ") {
        // Run osascript to convert alias to POSIX path
        let script = format!("get POSIX path of {}", path);
        if let Some(posix_path) = run_osascript(&script) {
            return posix_path;
        }
    } else if !path.starts_with('/') {
        // Assume it's a relative path, get absolute path
        if let Ok(canon_path) = std::fs::canonicalize(path) {
            if let Some(path_str) = canon_path.to_str() {
                return path_str.to_string();
            }
        }
    }
    
    path.to_string()
}

// Helper function to sanitize AppleScript strings
fn sanitize_for_applescript(s: &str) -> String {
    s.replace("\"", "\\\"").replace("\\", "\\\\")
}

// Message box implementation
pub fn message_box_ok(msg_box: &MessageBox) {
    let title = sanitize_for_applescript(msg_box.dialog.title());
    let message = sanitize_for_applescript(msg_box.dialog.message());
    
    let icon_param = match msg_box.icon() {
        MessageBoxIcon::Info => "",
        MessageBoxIcon::Warning => "with icon caution",
        MessageBoxIcon::Error => "with icon stop",
        MessageBoxIcon::Question => "with icon note",
    };
    
    let script = format!(
        "display dialog \"{}\" with title \"{}\" buttons {{\"OK\"}} default button \"OK\" {}",
        message, title, icon_param
    );
    
    let _ = run_osascript(&script);
}

pub fn message_box_ok_cancel(msg_box: &MessageBox, default: OkCancel) -> OkCancel {
    let title = sanitize_for_applescript(msg_box.dialog.title());
    let message = sanitize_for_applescript(msg_box.dialog.message());
    
    let icon_param = match msg_box.icon() {
        MessageBoxIcon::Info => "",
        MessageBoxIcon::Warning => "with icon caution",
        MessageBoxIcon::Error => "with icon stop",
        MessageBoxIcon::Question => "with icon note",
    };
    
    let default_button = match default {
        OkCancel::Ok => "\"OK\"",
        OkCancel::Cancel => "\"Cancel\"",
    };
    
    let script = format!(
        "display dialog \"{}\" with title \"{}\" buttons {{\"Cancel\", \"OK\"}} default button {} {}",
        message, title, default_button, icon_param
    );
    
    match run_osascript(&script) {
        Some(result) => {
            if result.contains("button returned:OK") {
                OkCancel::Ok
            } else {
                OkCancel::Cancel
            }
        }
        None => OkCancel::Cancel,
    }
}

pub fn message_box_yes_no(msg_box: &MessageBox, default: YesNo) -> YesNo {
    let title = sanitize_for_applescript(msg_box.dialog.title());
    let message = sanitize_for_applescript(msg_box.dialog.message());
    
    let icon_param = match msg_box.icon() {
        MessageBoxIcon::Info => "",
        MessageBoxIcon::Warning => "with icon caution",
        MessageBoxIcon::Error => "with icon stop",
        MessageBoxIcon::Question => "with icon note",
    };
    
    let default_button = match default {
        YesNo::Yes => "\"Yes\"",
        YesNo::No => "\"No\"",
    };
    
    let script = format!(
        "display dialog \"{}\" with title \"{}\" buttons {{\"No\", \"Yes\"}} default button {} {}",
        message, title, default_button, icon_param
    );
    
    match run_osascript(&script) {
        Some(result) => {
            if result.contains("button returned:Yes") {
                YesNo::Yes
            } else {
                YesNo::No
            }
        }
        None => YesNo::No,
    }
}

pub fn message_box_yes_no_cancel(msg_box: &MessageBox, default: YesNoCancel) -> YesNoCancel {
    let title = sanitize_for_applescript(msg_box.dialog.title());
    let message = sanitize_for_applescript(msg_box.dialog.message());
    
    let icon_param = match msg_box.icon() {
        MessageBoxIcon::Info => "",
        MessageBoxIcon::Warning => "with icon caution",
        MessageBoxIcon::Error => "with icon stop",
        MessageBoxIcon::Question => "with icon note",
    };
    
    let default_button = match default {
        YesNoCancel::Yes => "\"Yes\"",
        YesNoCancel::No => "\"No\"",
        YesNoCancel::Cancel => "\"Cancel\"",
    };
    
    let script = format!(
        "display dialog \"{}\" with title \"{}\" buttons {{\"Cancel\", \"No\", \"Yes\"}} default button {} {}",
        message, title, default_button, icon_param
    );
    
    match run_osascript(&script) {
        Some(result) => {
            if result.contains("button returned:Yes") {
                YesNoCancel::Yes
            } else if result.contains("button returned:No") {
                YesNoCancel::No
            } else {
                YesNoCancel::Cancel
            }
        }
        None => YesNoCancel::Cancel,
    }
}

pub fn input_box(input: &InputBox) -> Option<String> {
    let title = sanitize_for_applescript(input.dialog.title());
    let message = sanitize_for_applescript(input.dialog.message());
    let default = input.default_value().unwrap_or("");
    let default = sanitize_for_applescript(default);
    
    let hidden_param = if input.is_password() {
        "with hidden answer"
    } else {
        ""
    };
    
    let script = format!(
        "display dialog \"{}\" with title \"{}\" default answer \"{}\" buttons {{\"Cancel\", \"OK\"}} default button \"OK\" {}",
        message, title, default, hidden_param
    );
    
    match run_osascript(&script) {
        Some(result) => {
            // Parse the result to extract text returned
            // Example: {button returned:OK, text returned:hello}
            if result.contains("button returned:OK") {
                if let Some(start) = result.find("text returned:") {
                    let start = start + "text returned:".len();
                    let text = &result[start..];
                    
                    // Handle whether the text is in braces or not
                    if text.ends_with('}') {
                        Some(text[0..text.len()-1].to_string())
                    } else {
                        Some(text.to_string())
                    }
                } else {
                    None
                }
            } else {
                None
            }
        }
        None => None,
    }
}

pub fn save_file_dialog(dialog: &FileDialog) -> Option<String> {
    let title = sanitize_for_applescript(dialog.dialog.title());
    let path = to_posix_path(dialog.path());
    
    // Prepare default location parameter if path exists
    let default_location = if !path.is_empty() {
        if let Some(parent) = Path::new(&path).parent() {
            if let Some(dir_str) = parent.to_str() {
                format!("default location \"{}\"", sanitize_for_applescript(dir_str))
            } else {
                String::new()
            }
        } else {
            String::new()
        }
    } else {
        String::new()
    };
    
    // Prepare default name if provided
    let default_name = if !path.is_empty() {
        if let Some(filename) = Path::new(&path).file_name() {
            if let Some(name_str) = filename.to_str() {
                format!("default name \"{}\"", sanitize_for_applescript(name_str))
            } else {
                String::new()
            }
        } else {
            String::new()
        }
    } else {
        String::new()
    };
    
    // Prepare filter if provided
    let filter = if !dialog.filter_patterns().is_empty() {
        let patterns: Vec<String> = dialog.filter_patterns()
            .iter()
            .map(|p| {
                // Extract extension from pattern (*.ext -> ext)
                let ext = p.trim_start_matches("*.");
                format!("\"{}\"", sanitize_for_applescript(ext))
            })
            .collect();
        
        format!("of type {{{}}}", patterns.join(", "))
    } else {
        String::new()
    };
    
    let script = format!(
        "choose file name with prompt \"{}\" {} {} {}",
        title, default_location, default_name, filter
    );
    
    match run_osascript(&script) {
        Some(alias_path) => {
            // Convert the returned alias to a POSIX path
            let conversion_script = format!("POSIX path of {}", alias_path);
            run_osascript(&conversion_script)
        }
        None => None,
    }
}

pub fn open_file_dialog(dialog: &FileDialog) -> Option<Vec<String>> {
    let title = sanitize_for_applescript(dialog.dialog.title());
    let path = to_posix_path(dialog.path());
    
    // Prepare default location parameter if path exists
    let default_location = if !path.is_empty() {
        format!("default location \"{}\"", sanitize_for_applescript(&path))
    } else {
        String::new()
    };
    
    // Prepare multiple selection parameter
    let multiple = if dialog.multiple_selection() {
        "with multiple selections allowed"
    } else {
        ""
    };
    
    // Prepare filter if provided
    let filter = if !dialog.filter_patterns().is_empty() {
        let patterns: Vec<String> = dialog.filter_patterns()
            .iter()
            .map(|p| {
                // Extract extension from pattern (*.ext -> ext)
                let ext = p.trim_start_matches("*.");
                format!("\"{}\"", sanitize_for_applescript(ext))
            })
            .collect();
        
        format!("of type {{{}}}", patterns.join(", "))
    } else {
        String::new()
    };
    
    // First script gets the alias paths
    let choose_script = format!(
        "set theResult to choose file with prompt \"{}\" {} {} {}",
        title, default_location, multiple, filter
    );
    
    // Second script ensures we handle both single and multiple selection correctly
    let prepare_result_script = r#"
    if class of theResult is list then
        set resultList to theResult
    else
        set resultList to {theResult}
    end if
    
    set posixPaths to {}
    repeat with onePath in resultList
        set end of posixPaths to POSIX path of onePath
    end repeat
    
    set AppleScript's text item delimiters to "||"
    posixPaths as text
    "#;
    
    match run_osascript_multi(&[&choose_script, prepare_result_script]) {
        Some(result) => {
            // Split the paths that are joined by the delimiter
            let paths: Vec<String> = result.split("||").map(|s| s.to_string()).collect();
            Some(paths)
        }
        None => None,
    }
}

pub fn select_folder_dialog(dialog: &FileDialog) -> Option<String> {
    let title = sanitize_for_applescript(dialog.dialog.title());
    let path = to_posix_path(dialog.path());
    
    // Prepare default location parameter if path exists
    let default_location = if !path.is_empty() {
        format!("default location \"{}\"", sanitize_for_applescript(&path))
    } else {
        String::new()
    };
    
    let script = format!(
        "choose folder with prompt \"{}\" {}",
        title, default_location
    );
    
    match run_osascript(&script) {
        Some(alias_path) => {
            // Convert the returned alias to a POSIX path
            let conversion_script = format!("POSIX path of {}", alias_path);
            run_osascript(&conversion_script)
        }
        None => None,
    }
}

pub fn color_chooser_dialog(chooser: &ColorChooser) -> Option<(String, [u8; 3])> {
    let title = sanitize_for_applescript(chooser.dialog.title());
    
    // Prepare default color parameter if provided
    let default_rgb = match chooser.default_color() {
        DefaultColorValue::Hex(hex) => super::hex_to_rgb(hex),
        DefaultColorValue::RGB(rgb) => *rgb,
    };
    
    // Convert RGB values (0-255) to AppleScript color values (0-65535)
    let r = (default_rgb[0] as u32) * 257;
    let g = (default_rgb[1] as u32) * 257;
    let b = (default_rgb[2] as u32) * 257;
    
    // First script to choose color
    let choose_script = format!(
        "set theColor to choose color default color {{{}, {}, {}}} with prompt \"{}\"",
        r, g, b, title
    );
    
    // Second script to convert the color values back to RGB
    let convert_script = r#"
    -- Convert color values to RGB (0-255)
    set r to (item 1 of theColor) div 257
    set g to (item 2 of theColor) div 257
    set b to (item 3 of theColor) div 257
    return r & "," & g & "," & b
    "#;
    
    match run_osascript_multi(&[&choose_script, convert_script]) {
        Some(result) => {
            // Parse the RGB values
            let parts: Vec<&str> = result.split(',').collect();
            if parts.len() >= 3 {
                if let (Ok(r), Ok(g), Ok(b)) = (
                    parts[0].parse::<u8>(),
                    parts[1].parse::<u8>(),
                    parts[2].parse::<u8>(),
                ) {
                    let rgb = [r, g, b];
                    let hex = super::rgb_to_hex(&rgb);
                    return Some((hex, rgb));
                }
            }
            None
        }
        None => None,
    }
}

pub fn notification(notification: &Notification) -> bool {
    let title = sanitize_for_applescript(notification.title());
    let message = sanitize_for_applescript(notification.message());
    
    // Prepare subtitle parameter if provided
    let subtitle = match notification.subtitle() {
        Some(subtitle) => format!("subtitle \"{}\"", sanitize_for_applescript(subtitle)),
        None => String::new(),
    };
    
    // Prepare sound parameter if provided
    let sound = match notification.sound() {
        Some(sound) => format!("sound name \"{}\"", sanitize_for_applescript(sound)),
        None => String::new(),
    };
    
    let script = format!(
        "display notification \"{}\" with title \"{}\" {} {}",
        message, title, subtitle, sound
    );
    
    println!("script: {}", script);

    run_osascript(&script).is_some()
}