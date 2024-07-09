use windows::Win32::Foundation::HWND;
use windows::core::{PCSTR, PCWSTR};
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxA, MessageBoxW, MB_ICONINFORMATION, MB_OK};

fn main() {
    let hwnd = HWND(0);
    let utype = MB_ICONINFORMATION | MB_OK;

    // UTF-8 MessageBoxA
    let lptext = PCSTR(c"Hello".as_ptr() as *const u8);
    let lpcaption = PCSTR(c"World".as_ptr() as *const u8);
    let result_utf8 = unsafe { MessageBoxA(hwnd, lptext, lpcaption, utype) };
    println!("result (UTF-8): {:?}", result_utf8);

    // UTF-16 MessageBoxW
    let lptext_utf16: Vec<u16> = "Hello".encode_utf16().chain(std::iter::once(0)).collect();
    let lpcaption_utf16: Vec<u16> = "World".encode_utf16().chain(std::iter::once(0)).collect();
    let lptext = PCWSTR(lptext_utf16.as_ptr());
    let lpcaption = PCWSTR(lpcaption_utf16.as_ptr());

    let result_utf16 = unsafe {
        MessageBoxW(hwnd, lptext, lpcaption, utype)
    };
    println!("result (UTF-16): {:?}", result_utf16);
}