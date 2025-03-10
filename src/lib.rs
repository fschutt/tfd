//! # tinyfiledialogs-rs
//!
//! A pure Rust implementation of the tinyfiledialogs library.
//! Based on the original C library by Guillaume Vareille.
//!
//! ## Security Warning
//!
//! tinyfiledialogs should only be used with trusted input. Using it with
//! untrusted input, for example as dialog title or message, can in the worst
//! case lead to execution of arbitrary commands.

use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::env;

// Platform-specific modules
#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(all(unix, not(target_os = "macos")))]
mod unix;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MessageBoxIcon {
    Info,
    Warning,
    Error,
    Question,
}

impl MessageBoxIcon {
    fn to_str(&self) -> &'static str {
        match *self {
            MessageBoxIcon::Info => "info",
            MessageBoxIcon::Warning => "warning",
            MessageBoxIcon::Error => "error",
            MessageBoxIcon::Question => "question",
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum OkCancel {
    Cancel = 0,
    Ok = 1,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum YesNo {
    No = 0,
    Yes = 1,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum YesNoCancel {
    Cancel = 0,
    Yes = 1,
    No = 2,
}

pub fn message_box_ok(title: &str, message: &str, icon: MessageBoxIcon) {
    #[cfg(target_os = "windows")]
    {
        windows::message_box_ok(title, message, icon);
        return;
    }
    #[cfg(target_os = "macos")]
    {
        macos::message_box_ok(title, message, icon);
        return;
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        unix::message_box_ok(title, message, icon);
        return;
    }
}

pub fn message_box_ok_cancel(title: &str, message: &str, icon: MessageBoxIcon, default: OkCancel) -> OkCancel {
    #[cfg(target_os = "windows")]
    {
        return windows::message_box_ok_cancel(title, message, icon, default);
    }
    #[cfg(target_os = "macos")]
    {
        return macos::message_box_ok_cancel(title, message, icon, default);
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        return unix::message_box_ok_cancel(title, message, icon, default);
    }
    #[allow(unreachable_code)]
    OkCancel::Cancel
}

pub fn message_box_yes_no(title: &str, message: &str, icon: MessageBoxIcon, default: YesNo) -> YesNo {
    #[cfg(target_os = "windows")]
    {
        return windows::message_box_yes_no(title, message, icon, default);
    }
    #[cfg(target_os = "macos")]
    {
        return macos::message_box_yes_no(title, message, icon, default);
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        return unix::message_box_yes_no(title, message, icon, default);
    }
    #[allow(unreachable_code)]
    YesNo::No
}

pub fn message_box_yes_no_cancel(title: &str, message: &str, icon: MessageBoxIcon, default: YesNoCancel) -> YesNoCancel {
    #[cfg(target_os = "windows")]
    {
        return windows::message_box_yes_no_cancel(title, message, icon, default);
    }
    #[cfg(target_os = "macos")]
    {
        return macos::message_box_yes_no_cancel(title, message, icon, default);
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        return unix::message_box_yes_no_cancel(title, message, icon, default);
    }
    #[allow(unreachable_code)]
    YesNoCancel::Cancel
}

pub fn input_box(title: &str, message: &str, default: &str) -> Option<String> {
    #[cfg(target_os = "windows")]
    {
        return windows::input_box(title, message, Some(default));
    }
    #[cfg(target_os = "macos")]
    {
        return macos::input_box(title, message, Some(default));
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        return unix::input_box(title, message, Some(default));
    }
    #[allow(unreachable_code)]
    None
}

pub fn password_box(title: &str, message: &str) -> Option<String> {
    #[cfg(target_os = "windows")]
    {
        return windows::input_box(title, message, None);
    }
    #[cfg(target_os = "macos")]
    {
        return macos::input_box(title, message, None);
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        return unix::input_box(title, message, None);
    }
    #[allow(unreachable_code)]
    None
}

pub fn save_file_dialog(title: &str, path: &str) -> Option<String> {
    save_file_dialog_with_filter(title, path, &[], "")
}

pub fn save_file_dialog_with_filter(title: &str,
                                    path: &str,
                                    filter_patterns: &[&str],
                                    description: &str) -> Option<String> {
    #[cfg(target_os = "windows")]
    {
        return windows::save_file_dialog(title, path, filter_patterns, description);
    }
    #[cfg(target_os = "macos")]
    {
        return macos::save_file_dialog(title, path, filter_patterns, description);
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        return unix::save_file_dialog(title, path, filter_patterns, description);
    }
    #[allow(unreachable_code)]
    None
}

pub fn open_file_dialog(title: &str,
                        path: &str,
                        filter: Option<(&[&str], &str)>) -> Option<String> {
    open_file_dialog_multi(title, path, filter).and_then(|v| v.into_iter().next())
}

pub fn open_file_dialog_multi(title: &str,
                              path: &str,
                              filter: Option<(&[&str], &str)>) -> Option<Vec<String>> {
    let (patterns, description) = filter.unwrap_or((&[], ""));
    
    #[cfg(target_os = "windows")]
    {
        return windows::open_file_dialog(title, path, patterns, description, true);
    }
    #[cfg(target_os = "macos")]
    {
        return macos::open_file_dialog(title, path, patterns, description, true);
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        return unix::open_file_dialog(title, path, patterns, description, true);
    }
    #[allow(unreachable_code)]
    None
}

pub fn select_folder_dialog(title: &str, path: &str) -> Option<String> {
    #[cfg(target_os = "windows")]
    {
        return windows::select_folder_dialog(title, path);
    }
    #[cfg(target_os = "macos")]
    {
        return macos::select_folder_dialog(title, path);
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        return unix::select_folder_dialog(title, path);
    }
    #[allow(unreachable_code)]
    None
}

pub enum DefaultColorValue {
    Hex(String),
    RGB([u8; 3]),
}

pub fn color_chooser_dialog(title: &str, default: DefaultColorValue)
                            -> Option<(String, [u8; 3])> {
    #[cfg(target_os = "windows")]
    {
        return windows::color_chooser_dialog(title, default);
    }
    #[cfg(target_os = "macos")]
    {
        return macos::color_chooser_dialog(title, default);
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        return unix::color_chooser_dialog(title, default);
    }
    #[allow(unreachable_code)]
    None
}

// Helper functions
#[cfg(unix)]
fn get_command_output(cmd: &mut Command) -> Option<String> {
    if let Ok(output) = cmd.output() {
        if output.status.success() {
            if let Ok(stdout) = String::from_utf8(output.stdout) {
                return Some(stdout.trim().to_string());
            }
        }
    }
    None
}

#[cfg(unix)]
fn command_exists(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .stdout(Stdio::null())
        .status()
        .map_or(false, |s| s.success())
}

fn hex_to_rgb(hex: &str) -> [u8; 3] {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
    [r, g, b]
}

fn rgb_to_hex(rgb: &[u8; 3]) -> String {
    format!("#{:02x}{:02x}{:02x}", rgb[0], rgb[1], rgb[2])
}