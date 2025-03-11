fn main() {
    
    let input = tfd::InputBox::new("Input", "Test").run_modal();
    println!("input: {input:?}");

    // New style API usage with builder pattern
    let result = tfd::FileDialog::new("Select a file (new API)")
        .with_path(".")
        .open_file();
    
    println!("New API result: {result:?}");

    // Show a notification
    let notified = tfd::Notification::new("Test Notification", "This is a test notification")
        .with_subtitle("With a subtitle")
        .show();
        
    println!("Notification shown: {notified}");
    
    // Show a message box
    tfd::MessageBox::new("Message Box", "This is a test message box")
        .with_icon(tfd::MessageBoxIcon::Info)
        .run_modal();
    
    // Show a color picker
    let color_result = tfd::ColorChooser::new("Choose a color")
        .with_default_color(tfd::DefaultColorValue::RGB([255, 0, 0]))
        .run_modal();
    
    println!("Color chosen: {color_result:?}");
}