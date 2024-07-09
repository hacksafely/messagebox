use windows::Win32::Foundation::HWND;

use windows::core::PCSTR;
use windows::Win32::UI::WindowsAndMessaging::{
    MessageBoxA, MB_ICONINFORMATION, MB_OK, MESSAGEBOX_RESULT, MESSAGEBOX_STYLE,
};

#[no_mangle]
pub unsafe fn show_message_box() -> MESSAGEBOX_RESULT {
    let message = "Hello\0";
    let title = " Hello 2\0";
    let message_ptr = message.as_ptr();
    let title_ptr = title.as_ptr();
    println!("test");
    MessageBoxA(
        HWND(0),
        PCSTR(message_ptr),
        PCSTR(title_ptr),
        MB_OK | MB_ICONINFORMATION,
    )
    // Alternate way using MESSAGEBOX_STYLE
    // MessageBoxA(HWND(0), PCSTR(message.as_ptr()), PCSTR(title.as_ptr()), MESSAGEBOX_STYLE(0 | 64))
}

    // This function is needed to prevent "unresolved import" errors

/* fn main() {
    let result = unsafe { show_message_box("Hello from Rust!\0", "My First MessageBox\0") };
    loop {
        println!("{:?}", result)
    }
} */
