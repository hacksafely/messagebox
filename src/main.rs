use windows::Win32::Foundation::HWND;

use windows::core::PCSTR;
use windows::Win32::UI::WindowsAndMessaging::{
    MessageBoxA, MB_ICONINFORMATION, MB_OK, MESSAGEBOX_RESULT,
};



fn main() {

    let hwnd = HWND(0);
    let lptext = PCSTR(c"Hello".as_ptr() as *const u8);
    let lpcaption = PCSTR(c"World".as_ptr() as *const u8);
    let utype = MB_ICONINFORMATION | MB_OK;

    let result  = unsafe {
        MessageBoxA(hwnd, lptext, lpcaption, utype)
    };
    println!("result {:?}", result);

}

