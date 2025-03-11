fn main() {
    // Show a color picker
    let color_result = tfd::ColorChooser::new("Test").run_modal();

    println!("Color chosen: {color_result:?}");
}
