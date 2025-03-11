use super::*;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::path::Path;

// Check which dialog program is available
fn detect_dialog_program() -> &'static str {
    if command_exists("zenity") {
        "zenity"
    } else if command_exists("kdialog") {
        "kdialog"
    } else if command_exists("Xdialog") {
        "Xdialog"
    } else if command_exists("dialog") {
        "dialog"
    } else {
        "zenity" // Default to zenity, even if not present
    }
}

fn command_exists(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .stdout(Stdio::null())
        .status()
        .map_or(false, |s| s.success())
}

pub fn message_box_ok(msg_box: &MessageBox) {
    let title = msg_box.dialog.title();
    let message = msg_box.dialog.message();
    let icon = msg_box.icon();
    
    let dialog_program = detect_dialog_program();

    match dialog_program {
        "zenity" => {
            let icon_type = match icon {
                MessageBoxIcon::Info => "info",
                MessageBoxIcon::Warning => "warning",
                MessageBoxIcon::Error => "error",
                MessageBoxIcon::Question => "question",
            };

            let _ = Command::new("zenity")
                .arg("--info")
                .arg("--title")
                .arg(title)
                .arg("--text")
                .arg(message)
                .arg("--icon-name")
                .arg(icon_type)
                .status();
        }
        "kdialog" => {
            let icon_type = match icon {
                MessageBoxIcon::Info => "dialog-information",
                MessageBoxIcon::Warning => "dialog-warning",
                MessageBoxIcon::Error => "dialog-error",
                MessageBoxIcon::Question => "dialog-question",
            };

            let _ = Command::new("kdialog")
                .arg("--msgbox")
                .arg(message)
                .arg("--title")
                .arg(title)
                .arg("--icon")
                .arg(icon_type)
                .status();
        }
        "Xdialog" => {
            let _ = Command::new("Xdialog")
                .arg("--msgbox")
                .arg(message)
                .arg("0")
                .arg("0")
                .arg("--title")
                .arg(title)
                .status();
        }
        "dialog" => {
            let _ = Command::new("dialog")
                .arg("--msgbox")
                .arg(message)
                .arg("0")
                .arg("0")
                .arg("--title")
                .arg(title)
                .status();
        }
        _ => {
            // Fallback to console
            println!("{}: {}", title, message);
        }
    }
}

pub fn message_box_ok_cancel(msg_box: &MessageBox, default: OkCancel) -> OkCancel {
    let title = msg_box.dialog.title();
    let message = msg_box.dialog.message();
    let icon = msg_box.icon();
    
    let dialog_program = detect_dialog_program();

    match dialog_program {
        "zenity" => {
            let icon_type = match icon {
                MessageBoxIcon::Info => "info",
                MessageBoxIcon::Warning => "warning",
                MessageBoxIcon::Error => "error",
                MessageBoxIcon::Question => "question",
            };

            let status = Command::new("zenity")
                .arg("--question")
                .arg("--title")
                .arg(title)
                .arg("--text")
                .arg(message)
                .arg("--icon-name")
                .arg(icon_type)
                .arg(if default == OkCancel::Cancel {
                    "--default-cancel"
                } else {
                    ""
                })
                .arg("--ok-label=Ok")
                .arg("--cancel-label=Cancel")
                .status();

            match status {
                Ok(exit) => {
                    if exit.success() {
                        OkCancel::Ok
                    } else {
                        OkCancel::Cancel
                    }
                }
                Err(_) => OkCancel::Cancel,
            }
        }
        "kdialog" => {
            let status = Command::new("kdialog")
                .arg("--yesno")
                .arg(message)
                .arg("--title")
                .arg(title)
                .arg("--yes-label")
                .arg("Ok")
                .arg("--no-label")
                .arg("Cancel")
                .status();

            match status {
                Ok(exit) => {
                    if exit.success() {
                        OkCancel::Ok
                    } else {
                        OkCancel::Cancel
                    }
                }
                Err(_) => OkCancel::Cancel,
            }
        }
        "Xdialog" => {
            let status = Command::new("Xdialog")
                .arg("--yesno")
                .arg(message)
                .arg("0")
                .arg("0")
                .arg("--title")
                .arg(title)
                .arg("--yes-label")
                .arg("Ok")
                .arg("--no-label")
                .arg("Cancel")
                .status();

            match status {
                Ok(exit) => {
                    if exit.success() {
                        OkCancel::Ok
                    } else {
                        OkCancel::Cancel
                    }
                }
                Err(_) => OkCancel::Cancel,
            }
        }
        "dialog" => {
            let status = Command::new("dialog")
                .arg("--yesno")
                .arg(message)
                .arg("0")
                .arg("0")
                .arg("--title")
                .arg(title)
                .status();

            match status {
                Ok(exit) => {
                    if exit.success() {
                        OkCancel::Ok
                    } else {
                        OkCancel::Cancel
                    }
                }
                Err(_) => OkCancel::Cancel,
            }
        }
        _ => {
            // Fallback to console
            println!("{}: {} (y/n)", title, message);
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap_or(0);
            if input.trim().to_lowercase() == "y" {
                OkCancel::Ok
            } else {
                OkCancel::Cancel
            }
        }
    }
}

pub fn message_box_yes_no(msg_box: &MessageBox, default: YesNo) -> YesNo {
    let title = msg_box.dialog.title();
    let message = msg_box.dialog.message();
    let icon = msg_box.icon();
    
    let dialog_program = detect_dialog_program();

    match dialog_program {
        "zenity" => {
            let icon_type = match icon {
                MessageBoxIcon::Info => "info",
                MessageBoxIcon::Warning => "warning",
                MessageBoxIcon::Error => "error",
                MessageBoxIcon::Question => "question",
            };

            let status = Command::new("zenity")
                .arg("--question")
                .arg("--title")
                .arg(title)
                .arg("--text")
                .arg(message)
                .arg("--icon-name")
                .arg(icon_type)
                .arg(if default == YesNo::No {
                    "--default-cancel"
                } else {
                    ""
                })
                .status();

            match status {
                Ok(exit) => {
                    if exit.success() {
                        YesNo::Yes
                    } else {
                        YesNo::No
                    }
                }
                Err(_) => YesNo::No,
            }
        }
        "kdialog" => {
            let status = Command::new("kdialog")
                .arg("--yesno")
                .arg(message)
                .arg("--title")
                .arg(title)
                .status();

            match status {
                Ok(exit) => {
                    if exit.success() {
                        YesNo::Yes
                    } else {
                        YesNo::No
                    }
                }
                Err(_) => YesNo::No,
            }
        }
        "Xdialog" => {
            let status = Command::new("Xdialog")
                .arg("--yesno")
                .arg(message)
                .arg("0")
                .arg("0")
                .arg("--title")
                .arg(title)
                .status();

            match status {
                Ok(exit) => {
                    if exit.success() {
                        YesNo::Yes
                    } else {
                        YesNo::No
                    }
                }
                Err(_) => YesNo::No,
            }
        }
        "dialog" => {
            let status = Command::new("dialog")
                .arg("--yesno")
                .arg(message)
                .arg("0")
                .arg("0")
                .arg("--title")
                .arg(title)
                .status();

            match status {
                Ok(exit) => {
                    if exit.success() {
                        YesNo::Yes
                    } else {
                        YesNo::No
                    }
                }
                Err(_) => YesNo::No,
            }
        }
        _ => {
            // Fallback to console
            println!("{}: {} (y/n)", title, message);
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap_or(0);
            if input.trim().to_lowercase() == "y" {
                YesNo::Yes
            } else {
                YesNo::No
            }
        }
    }
}

pub fn message_box_yes_no_cancel(msg_box: &MessageBox, default: YesNoCancel) -> YesNoCancel {
    let title = msg_box.dialog.title();
    let message = msg_box.dialog.message();
    let icon = msg_box.icon();
    
    let dialog_program = detect_dialog_program();

    match dialog_program {
        "zenity" => {
            let icon_type = match icon {
                MessageBoxIcon::Info => "info",
                MessageBoxIcon::Warning => "warning",
                MessageBoxIcon::Error => "error",
                MessageBoxIcon::Question => "question",
            };

            let output = Command::new("zenity")
                .arg("--list")
                .arg("--radiolist")
                .arg("--title")
                .arg(title)
                .arg("--text")
                .arg(message)
                .arg("--column")
                .arg("")
                .arg("--column")
                .arg("Response")
                .arg(match default {
                    YesNoCancel::Yes => "TRUE",
                    _ => "FALSE",
                })
                .arg("Yes")
                .arg(match default {
                    YesNoCancel::No => "TRUE",
                    _ => "FALSE",
                })
                .arg("No")
                .arg(match default {
                    YesNoCancel::Cancel => "TRUE",
                    _ => "FALSE",
                })
                .arg("Cancel")
                .output();

            match output {
                Ok(out) => {
                    if out.status.success() {
                        let stdout = String::from_utf8_lossy(&out.stdout);
                        match stdout.trim() {
                            "Yes" => YesNoCancel::Yes,
                            "No" => YesNoCancel::No,
                            _ => YesNoCancel::Cancel,
                        }
                    } else {
                        YesNoCancel::Cancel
                    }
                }
                Err(_) => YesNoCancel::Cancel,
            }
        }
        "kdialog" => {
            let output = Command::new("kdialog")
                .arg("--yesnocancel")
                .arg(message)
                .arg("--title")
                .arg(title)
                .output();

            match output {
                Ok(out) => match out.status.code() {
                    Some(0) => YesNoCancel::Yes,
                    Some(1) => YesNoCancel::No,
                    _ => YesNoCancel::Cancel,
                },
                Err(_) => YesNoCancel::Cancel,
            }
        }
        _ => {
            // Fallback to console or other dialog programs
            println!("{}: {} (y/n/c)", title, message);
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap_or(0);
            match input.trim().to_lowercase().as_str() {
                "y" => YesNoCancel::Yes,
                "n" => YesNoCancel::No,
                _ => YesNoCancel::Cancel,
            }
        }
    }
}

pub fn input_box(input: &InputBox) -> Option<String> {
    let title = input.dialog.title();
    let message = input.dialog.message();
    let default_value = input.default_value().unwrap_or("");
    let is_password = input.is_password();
    
    let dialog_program = detect_dialog_program();

    match dialog_program {
        "zenity" => {
            let mut cmd = Command::new("zenity");
            cmd.arg("--entry")
                .arg("--title")
                .arg(title)
                .arg("--text")
                .arg(message);

            if !default_value.is_empty() {
                cmd.arg("--entry-text").arg(default_value);
            }

            if is_password {
                cmd.arg("--hide-text");
            }

            let output = cmd.output();

            match output {
                Ok(out) => {
                    if out.status.success() {
                        let stdout = String::from_utf8_lossy(&out.stdout);
                        Some(stdout.trim().to_string())
                    } else {
                        None
                    }
                }
                Err(_) => None,
            }
        }
        "kdialog" => {
            let mut cmd = Command::new("kdialog");

            if is_password {
                cmd.arg("--password");
            } else {
                cmd.arg("--inputbox");
            }

            cmd.arg(message)
                .arg(default_value)
                .arg("--title")
                .arg(title);

            let output = cmd.output();

            match output {
                Ok(out) => {
                    if out.status.success() {
                        let stdout = String::from_utf8_lossy(&out.stdout);
                        Some(stdout.trim().to_string())
                    } else {
                        None
                    }
                }
                Err(_) => None,
            }
        }
        _ => {
            // Fallback to console
            println!("{}: {}", title, message);
            print!("> ");
            let mut input = String::new();
            if std::io::stdin().read_line(&mut input).is_ok() {
                Some(input.trim().to_string())
            } else {
                None
            }
        }
    }
}

pub fn save_file_dialog(dialog: &FileDialog) -> Option<String> {
    let title = dialog.dialog.title();
    let path = dialog.path();
    let filter_patterns = dialog.filter_patterns();
    let description = dialog.filter_description();
    
    let dialog_program = detect_dialog_program();

    match dialog_program {
        "zenity" => {
            let mut cmd = Command::new("zenity");
            cmd.arg("--file-selection")
                .arg("--save")
                .arg("--confirm-overwrite")
                .arg("--title")
                .arg(title);

            if !path.is_empty() {
                cmd.arg("--filename").arg(path);
            }

            if !filter_patterns.is_empty() {
                let filter = format!(
                    "--file-filter={} | {}",
                    description,
                    filter_patterns.join(" ")
                );
                cmd.arg(filter);
            }

            let output = cmd.output();

            match output {
                Ok(out) => {
                    if out.status.success() {
                        let stdout = String::from_utf8_lossy(&out.stdout);
                        Some(stdout.trim().to_string())
                    } else {
                        None
                    }
                }
                Err(_) => None,
            }
        }
        "kdialog" => {
            let mut cmd = Command::new("kdialog");
            cmd.arg("--getsavefilename")
                .arg(path)
                .arg("--title")
                .arg(title);

            if !filter_patterns.is_empty() {
                let filter = filter_patterns.join(" ");
                cmd.arg(filter);
            }

            let output = cmd.output();

            match output {
                Ok(out) => {
                    if out.status.success() {
                        let stdout = String::from_utf8_lossy(&out.stdout);
                        Some(stdout.trim().to_string())
                    } else {
                        None
                    }
                }
                Err(_) => None,
            }
        }
        _ => {
            // Fallback to console
            println!("{}: Save file (default: {})", title, path);
            print!("> ");
            let mut input = String::new();
            if std::io::stdin().read_line(&mut input).is_ok() {
                let input = input.trim();
                if input.is_empty() {
                    Some(path.to_string())
                } else {
                    Some(input.to_string())
                }
            } else {
                None
            }
        }
    }
}

pub fn open_file_dialog(dialog: &FileDialog) -> Option<Vec<String>> {
    let title = dialog.dialog.title();
    let path = dialog.path();
    let filter_patterns = dialog.filter_patterns();
    let description = dialog.filter_description();
    let allow_multi = dialog.multiple_selection();
    
    let dialog_program = detect_dialog_program();

    match dialog_program {
        "zenity" => {
            let mut cmd = Command::new("zenity");
            cmd.arg("--file-selection").arg("--title").arg(title);

            if allow_multi {
                cmd.arg("--multiple");
            }

            if !path.is_empty() {
                cmd.arg("--filename").arg(path);
            }

            if !filter_patterns.is_empty() {
                let filter = format!(
                    "--file-filter={} | {}",
                    description,
                    filter_patterns.join(" ")
                );
                cmd.arg(filter);
            }

            let output = cmd.output();

            match output {
                Ok(out) => {
                    if out.status.success() {
                        let stdout = String::from_utf8_lossy(&out.stdout);
                        let files: Vec<String> =
                            stdout.trim().split('|').map(|s| s.to_string()).collect();
                        if files.is_empty() {
                            None
                        } else {
                            Some(files)
                        }
                    } else {
                        None
                    }
                }
                Err(_) => None,
            }
        }
        "kdialog" => {
            let mut cmd = Command::new("kdialog");
            cmd.arg(if allow_multi {
                "--getopenfilename"
            } else {
                "--getopenfilename"
            })
            .arg(path)
            .arg("--title")
            .arg(title);

            if !filter_patterns.is_empty() {
                let filter = filter_patterns.join(" ");
                cmd.arg(filter);
            }

            let output = cmd.output();

            match output {
                Ok(out) => {
                    if out.status.success() {
                        let stdout = String::from_utf8_lossy(&out.stdout);
                        let files: Vec<String> =
                            stdout.trim().split(' ').map(|s| s.to_string()).collect();
                        if files.is_empty() {
                            None
                        } else {
                            Some(files)
                        }
                    } else {
                        None
                    }
                }
                Err(_) => None,
            }
        }
        _ => {
            // Fallback to console
            println!("{}: Open file", title);
            print!("> ");
            let mut input = String::new();
            if std::io::stdin().read_line(&mut input).is_ok() {
                let input = input.trim();
                if input.is_empty() {
                    None
                } else {
                    Some(vec![input.to_string()])
                }
            } else {
                None
            }
        }
    }
}

pub fn select_folder_dialog(dialog: &FileDialog) -> Option<String> {
    let title = dialog.dialog.title();
    let path = dialog.path();
    
    let dialog_program = detect_dialog_program();

    match dialog_program {
        "zenity" => {
            let mut cmd = Command::new("zenity");
            cmd.arg("--file-selection")
                .arg("--directory")
                .arg("--title")
                .arg(title);

            if !path.is_empty() {
                cmd.arg("--filename").arg(path);
            }

            let output = cmd.output();

            match output {
                Ok(out) => {
                    if out.status.success() {
                        let stdout = String::from_utf8_lossy(&out.stdout);
                        Some(stdout.trim().to_string())
                    } else {
                        None
                    }
                }
                Err(_) => None,
            }
        }
        "kdialog" => {
            let mut cmd = Command::new("kdialog");
            cmd.arg("--getexistingdirectory")
                .arg(path)
                .arg("--title")
                .arg(title);

            let output = cmd.output();

            match output {
                Ok(out) => {
                    if out.status.success() {
                        let stdout = String::from_utf8_lossy(&out.stdout);
                        Some(stdout.trim().to_string())
                    } else {
                        None
                    }
                }
                Err(_) => None,
            }
        }
        _ => {
            // Fallback to console
            println!("{}: Select folder (default: {})", title, path);
            print!("> ");
            let mut input = String::new();
            if std::io::stdin().read_line(&mut input).is_ok() {
                let input = input.trim();
                if input.is_empty() {
                    Some(path.to_string())
                } else {
                    Some(input.to_string())
                }
            } else {
                None
            }
        }
    }
}

pub fn color_chooser_dialog(chooser: &ColorChooser) -> Option<(String, [u8; 3])> {
    let title = chooser.dialog.title();
    
    let default_rgb = match chooser.default_color() {
        DefaultColorValue::Hex(hex) => super::hex_to_rgb(hex),
        DefaultColorValue::RGB(rgb) => *rgb,
    };

    let default_hex = super::rgb_to_hex(&default_rgb);
    
    let dialog_program = detect_dialog_program();

    match dialog_program {
        "zenity" => {
            let output = Command::new("zenity")
                .arg("--color-selection")
                .arg("--title")
                .arg(title)
                .arg("--color")
                .arg(&default_hex)
                .output();

            match output {
                Ok(out) => {
                    if out.status.success() {
                        let stdout = String::from_utf8_lossy(&out.stdout);
                        let color = stdout.trim();

                        // Parse RGB values
                        let rgb = if color.starts_with('#') {
                            super::hex_to_rgb(color)
                        } else if color.starts_with("rgb") {
                            // Parse "rgb(R,G,B)"
                            let parts: Vec<&str> = color
                                .trim_start_matches("rgb(")
                                .trim_end_matches(')')
                                .split(',')
                                .collect();

                            if parts.len() >= 3 {
                                let r = parts[0].trim().parse::<u8>().unwrap_or(0);
                                let g = parts[1].trim().parse::<u8>().unwrap_or(0);
                                let b = parts[2].trim().parse::<u8>().unwrap_or(0);
                                [r, g, b]
                            } else {
                                [0, 0, 0]
                            }
                        } else {
                            [0, 0, 0]
                        };

                        let hex = super::rgb_to_hex(&rgb);
                        Some((hex, rgb))
                    } else {
                        None
                    }
                }
                Err(_) => None,
            }
        }
        "kdialog" => {
            let output = Command::new("kdialog")
                .arg("--getcolor")
                .arg("--default")
                .arg(&default_hex)
                .arg("--title")
                .arg(title)
                .output();

            match output {
                Ok(out) => {
                    if out.status.success() {
                        let stdout = String::from_utf8_lossy(&out.stdout);
                        let color = stdout.trim();

                        if color.starts_with('#') {
                            let rgb = super::hex_to_rgb(color);
                            Some((color.to_string(), rgb))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                Err(_) => None,
            }
        }
        _ => {
            // Fallback to console
            println!("{}: Choose color (default: {})", title, default_hex);
            print!("> ");
            let mut input = String::new();
            if std::io::stdin().read_line(&mut input).is_ok() {
                let input = input.trim();
                if input.is_empty() {
                    Some((default_hex, default_rgb))
                } else if input.starts_with('#') && input.len() == 7 {
                    let rgb = super::hex_to_rgb(input);
                    Some((input.to_string(), rgb))
                } else {
                    None
                }
            } else {
                None
            }
        }
    }
}

pub fn notification(notification: &Notification) -> bool {
    let title = notification.title();
    let message = notification.message();
    let subtitle = notification.subtitle().unwrap_or("");
    
    // Try notify-send first (standard for Linux desktop notifications)
    if command_exists("notify-send") {
        let status = Command::new("notify-send")
            .arg(title)
            .arg(message)
            .status();
            
        return status.is_ok() && status.unwrap().success();
    }
    
    // Fallback to zenity if available
    if command_exists("zenity") {
        let status = Command::new("zenity")
            .arg("--notification")
            .arg("--text")
            .arg(format!("{}: {}", title, message))
            .status();
            
        return status.is_ok() && status.unwrap().success();
    }
    
    // Fallback to kdialog if available
    if command_exists("kdialog") {
        let status = Command::new("kdialog")
            .arg("--passivepopup")
            .arg(message)
            .arg("5")  // Show for 5 seconds
            .arg("--title")
            .arg(title)
            .status();
            
        return status.is_ok() && status.unwrap().success();
    }
    
    // Last resort - print to console
    println!("Notification: {} - {}", title, message);
    if !subtitle.is_empty() {
        println!("  {}", subtitle);
    }
    
    true
}