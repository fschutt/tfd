fn main() {
    let s = tfd::color_chooser_dialog("Message box", tfd::DefaultColorValue::RGB([0, 0, 0]));
    println!("{s:?}");
}
