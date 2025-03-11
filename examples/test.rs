#[cfg(target_os = "android")]
extern "C" {
    fn JNI_OnLoad(vm: *mut ::std::ffi::c_void, reserved: *mut ::std::ffi::c_void) -> i32;
}

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "C" fn Java_com_example_tinyfiledialogs_MainActivity_runTest(
    env: *mut ::std::ffi::c_void,
    _: *mut ::std::ffi::c_void,
) {
    unsafe {
        // Store JNI environment in thread local storage
        // Implementation details would depend on your Android setup
    }
    main()
}

fn main() {
    // Show a color picker
    let color_result = tfd::ColorChooser::new("Test").run_modal();

    println!("Color chosen: {color_result:?}");
}
