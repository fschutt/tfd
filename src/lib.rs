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

use std::path::{Path, PathBuf};

// Platform-specific modules
#[cfg(target_os = "macos")]
mod macos;
#[cfg(all(unix, not(target_os = "macos")))]
mod unix;
#[cfg(target_os = "windows")]
mod windows;

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

// Base dialog struct
pub struct Dialog {
    title: String,
    message: String,
}

impl Dialog {
    pub fn new<S: Into<String>, Q: Into<String>>(title: S, message: Q) -> Self {
        Self {
            title: title.into(),
            message: message.into(),
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn with_title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = title.into();
        self
    }

    pub fn with_message<S: Into<String>>(mut self, message: S) -> Self {
        self.message = message.into();
        self
    }

    /// Sanitize input for shell execution
    fn sanitize_input(input: &str) -> String {
        input
            .replace("\"", "\\\"")
            .replace("'", "\\'")
            .replace("`", "\\`")
    }

    /// Verify path exists
    fn verify_path(path: &str) -> Option<PathBuf> {
        let path = Path::new(path);
        if path.exists() {
            Some(path.to_path_buf())
        } else {
            None
        }
    }
}

// Message Box
pub struct MessageBox {
    dialog: Dialog,
    icon: MessageBoxIcon,
}

impl MessageBox {
    pub fn new<S: Into<String>>(title: S, message: S) -> Self {
        Self {
            dialog: Dialog::new(title, message),
            icon: MessageBoxIcon::Info,
        }
    }

    pub fn with_icon(mut self, icon: MessageBoxIcon) -> Self {
        self.icon = icon;
        self
    }

    pub fn icon(&self) -> MessageBoxIcon {
        self.icon
    }

    pub fn run_modal(&self) {
        #[cfg(target_os = "macos")]
        macos::message_box_ok(self);

        #[cfg(all(unix, not(target_os = "macos")))]
        unix::message_box_ok(self);

        #[cfg(target_os = "windows")]
        windows::message_box_ok(self);
    }

    pub fn run_modal_ok_cancel(&self, default: OkCancel) -> OkCancel {
        #[cfg(target_os = "macos")]
        return macos::message_box_ok_cancel(self, default);

        #[cfg(all(unix, not(target_os = "macos")))]
        return unix::message_box_ok_cancel(self, default);

        #[cfg(target_os = "windows")]
        return windows::message_box_ok_cancel(self, default);

        #[allow(unreachable_code)]
        OkCancel::Cancel
    }

    pub fn run_modal_yes_no(&self, default: YesNo) -> YesNo {
        #[cfg(target_os = "macos")]
        return macos::message_box_yes_no(self, default);

        #[cfg(all(unix, not(target_os = "macos")))]
        return unix::message_box_yes_no(self, default);

        #[cfg(target_os = "windows")]
        return windows::message_box_yes_no(self, default);

        #[allow(unreachable_code)]
        YesNo::No
    }

    pub fn run_modal_yes_no_cancel(&self, default: YesNoCancel) -> YesNoCancel {
        #[cfg(target_os = "macos")]
        return macos::message_box_yes_no_cancel(self, default);

        #[cfg(all(unix, not(target_os = "macos")))]
        return unix::message_box_yes_no_cancel(self, default);

        #[cfg(target_os = "windows")]
        return windows::message_box_yes_no_cancel(self, default);

        #[allow(unreachable_code)]
        YesNoCancel::Cancel
    }
}

// Input Box
pub struct InputBox {
    dialog: Dialog,
    default_value: Option<String>,
    is_password: bool,
}

impl InputBox {
    pub fn new<S: Into<String>>(title: S, message: S) -> Self {
        Self {
            dialog: Dialog::new(title, message),
            default_value: None,
            is_password: false,
        }
    }

    pub fn with_default<S: Into<String>>(mut self, default: S) -> Self {
        self.default_value = Some(default.into());
        self
    }

    pub fn password(mut self, is_password: bool) -> Self {
        self.is_password = is_password;
        self
    }

    pub fn default_value(&self) -> Option<&str> {
        self.default_value.as_deref()
    }

    pub fn is_password(&self) -> bool {
        self.is_password
    }

    pub fn run_modal(&self) -> Option<String> {
        #[cfg(target_os = "macos")]
        return macos::input_box(self);

        #[cfg(all(unix, not(target_os = "macos")))]
        return unix::input_box(self);

        #[cfg(target_os = "windows")]
        return windows::input_box(self);

        #[allow(unreachable_code)]
        None
    }
}

// File Dialog
pub struct FileDialog {
    dialog: Dialog,
    path: String,
    filter_patterns: Vec<String>,
    filter_description: String,
    multiple_selection: bool,
}

impl FileDialog {
    pub fn new<S: Into<String>>(title: S) -> Self {
        Self {
            dialog: Dialog::new(title, ""),
            path: String::new(),
            filter_patterns: Vec::new(),
            filter_description: String::new(),
            multiple_selection: false,
        }
    }

    pub fn with_path<S: Into<String>>(mut self, path: S) -> Self {
        self.path = path.into();
        self
    }

    pub fn with_filter<S: Into<String>>(mut self, patterns: &[&str], description: S) -> Self {
        self.filter_patterns = patterns.iter().map(|&s| s.to_string()).collect();
        self.filter_description = description.into();
        self
    }

    pub fn with_multiple_selection(mut self, allow_multi: bool) -> Self {
        self.multiple_selection = allow_multi;
        self
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn filter_patterns(&self) -> &[String] {
        &self.filter_patterns
    }

    pub fn filter_description(&self) -> &str {
        &self.filter_description
    }

    pub fn multiple_selection(&self) -> bool {
        self.multiple_selection
    }

    pub fn save_file(&self) -> Option<String> {
        #[cfg(target_os = "macos")]
        return macos::save_file_dialog(self);

        #[cfg(all(unix, not(target_os = "macos")))]
        return unix::save_file_dialog(self);

        #[cfg(target_os = "windows")]
        return windows::save_file_dialog(self);

        #[allow(unreachable_code)]
        None
    }

    pub fn open_file(&self) -> Option<String> {
        self.open_files().and_then(|v| v.into_iter().next())
    }

    pub fn open_files(&self) -> Option<Vec<String>> {
        #[cfg(target_os = "macos")]
        return macos::open_file_dialog(self);

        #[cfg(all(unix, not(target_os = "macos")))]
        return unix::open_file_dialog(self);

        #[cfg(target_os = "windows")]
        return windows::open_file_dialog(self);

        #[allow(unreachable_code)]
        None
    }

    pub fn select_folder(&self) -> Option<String> {
        #[cfg(target_os = "macos")]
        return macos::select_folder_dialog(self);

        #[cfg(all(unix, not(target_os = "macos")))]
        return unix::select_folder_dialog(self);

        #[cfg(target_os = "windows")]
        return windows::select_folder_dialog(self);

        #[allow(unreachable_code)]
        None
    }
}

pub enum DefaultColorValue {
    Hex(String),
    RGB([u8; 3]),
}

pub struct ColorChooser {
    dialog: Dialog,
    default_color: DefaultColorValue,
}

impl ColorChooser {
    pub fn new<S: Into<String>>(title: S) -> Self {
        Self {
            dialog: Dialog::new(title, String::new()),
            default_color: DefaultColorValue::RGB([0, 0, 0]),
        }
    }

    pub fn with_default_color(mut self, default: DefaultColorValue) -> Self {
        self.default_color = default;
        self
    }

    pub fn default_color(&self) -> &DefaultColorValue {
        &self.default_color
    }

    pub fn run_modal(&self) -> Option<(String, [u8; 3])> {
        #[cfg(target_os = "macos")]
        return macos::color_chooser_dialog(self);

        #[cfg(all(unix, not(target_os = "macos")))]
        return unix::color_chooser_dialog(self);

        #[cfg(target_os = "windows")]
        return windows::color_chooser_dialog(self);

        #[allow(unreachable_code)]
        None
    }
}

pub struct Notification {
    title: String,
    message: String,
    subtitle: Option<String>,
    sound: Option<String>,
}

impl Notification {
    pub fn new<S: Into<String>>(title: S, message: S) -> Self {
        Self {
            title: title.into(),
            message: message.into(),
            subtitle: None,
            sound: None,
        }
    }

    pub fn with_subtitle<S: Into<String>>(mut self, subtitle: S) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    pub fn with_sound<S: Into<String>>(mut self, sound: S) -> Self {
        self.sound = Some(sound.into());
        self
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn subtitle(&self) -> Option<&str> {
        self.subtitle.as_deref()
    }

    pub fn sound(&self) -> Option<&str> {
        self.sound.as_deref()
    }

    pub fn show(&self) -> bool {
        #[cfg(target_os = "macos")]
        return macos::notification(self);

        #[cfg(all(unix, not(target_os = "macos")))]
        return unix::notification(self);

        #[cfg(target_os = "windows")]
        return windows::notification(self);

        #[allow(unreachable_code)]
        false
    }
}

// Utility functions
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
