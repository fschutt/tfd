# tfd

A pure Rust implementation of the tinyfiledialogs library, based on the original C library by Guillaume Vareille.

## Features

- Cross-platform native dialog boxes (macOS, Linux/Unix, Windows)
- Message boxes (info, warning, error, question)
- Input boxes (with optional password mode)
- File open/save dialogs
- Folder selection
- Color picker
- System notifications

## Security Warning

tinyfiledialogs should only be used with trusted input. Using it with untrusted input, for example as dialog title or message, can in the worst case lead to execution of arbitrary commands.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tfd = "1.0.0"
```

## Examples

### Message Box

```rust
use tinyfiledialogs as tfd;

// Simple message box
tfd::MessageBox::new("Information", "This is an informational message")
    .with_icon(tfd::MessageBoxIcon::Info)
    .run_modal();

// Yes/No dialog
let result = tfd::MessageBox::new("Question", "Do you want to continue?")
    .with_icon(tfd::MessageBoxIcon::Question)
    .run_modal_yes_no(tfd::YesNo::Yes);

if result == tfd::YesNo::Yes {
    println!("User selected Yes");
} else {
    println!("User selected No");
}

// Yes/No/Cancel dialog
let result = tfd::MessageBox::new("Question", "Save changes?")
    .with_icon(tfd::MessageBoxIcon::Question)
    .run_modal_yes_no_cancel(tfd::YesNoCancel::Yes);

match result {
    tfd::YesNoCancel::Yes => println!("User selected Yes"),
    tfd::YesNoCancel::No => println!("User selected No"),
    tfd::YesNoCancel::Cancel => println!("User selected Cancel"),
}
```

### Input Box

```rust
use tinyfiledialogs as tfd;

// Simple input
let name = tfd::InputBox::new("Input", "Enter your name:")
    .with_default("User")
    .run_modal();

if let Some(name) = name {
    println!("Hello, {name}!");
}

// Password input
let password = tfd::InputBox::new("Password", "Enter your password:")
    .password(true)
    .run_modal();

if let Some(password) = password {
    println!("Password entered: {}", "*".repeat(password.len()));
}
```

### File Dialogs

```rust
use tinyfiledialogs as tfd;

// Open file dialog
let file = tfd::FileDialog::new("Open File")
    .with_filter(&["*.txt", "*.rs"], "Text files")
    .open_file();

if let Some(path) = file {
    println!("Selected file: {path}");
}

// Open multiple files
let files = tfd::FileDialog::new("Open Files")
    .with_filter(&["*.png", "*.jpg"], "Image files")
    .with_multiple_selection(true)
    .open_files();

if let Some(paths) = files {
    println!("Selected files:");
    for path in paths {
        println!("  {path}");
    }
}

// Save file dialog
let file = tfd::FileDialog::new("Save File")
    .with_filter(&["*.txt"], "Text files")
    .save_file();

if let Some(path) = file {
    println!("File will be saved to: {path}");
}

// Select folder
let folder = tfd::FileDialog::new("Select Folder")
    .select_folder();

if let Some(path) = folder {
    println!("Selected folder: {path}");
}
```

### Color Chooser

```rust
use tinyfiledialogs as tfd;

// Color picker with default black
let color_result = tfd::ColorChooser::new("Choose a color")
    .run_modal();

if let Some((hex, rgb)) = color_result {
    println!("Color chosen: {hex} (RGB: {}, {}, {})", rgb[0], rgb[1], rgb[2]);
}

// Color picker with default color
let color_result = tfd::ColorChooser::new("Choose a color")
    .with_default_color(tfd::DefaultColorValue::RGB([255, 0, 0]))  // Default red
    .run_modal();

if let Some((hex, rgb)) = color_result {
    println!("Color chosen: {hex} (RGB: {}, {}, {})", rgb[0], rgb[1], rgb[2]);
}
```

### Notifications

```rust
use tinyfiledialogs as tfd;

// Simple notification
tfd::Notification::new("Alert", "Process completed successfully")
    .show();

// Notification with sound and subtitle
tfd::Notification::new("Download Complete", "Your file has been downloaded")
    .with_subtitle("File: example.zip")
    .with_sound("Default") // "Default", "IM", "Mail", "Reminder", etc.
    .show();

// Platform-specific behavior:
// - macOS: Uses native notifications via AppleScript
// - Linux: Uses notify-send, zenity, or kdialog
// - Windows: Uses Toast notifications on Win10+ or message boxes on older versions
```

## Platform-specific Notes

### macOS

Dialogs on macOS are implemented using AppleScript.

### Linux/Unix

Dialogs on Linux/Unix use the following programs in order of preference:
- zenity
- kdialog
- Xdialog
- dialog

If none are available, it falls back to console/terminal.

### Windows

Dialogs on Windows use the native Windows API:
- Dialog boxes use standard Win32 API functions
- File dialogs use the Common Dialog API
- Color picker uses the Common Dialog API
- Notifications use Toast Notifications on Windows 10+ and fallback to message boxes on older versions

## License

This project is licensed under the MIT License - see the LICENSE file for details.