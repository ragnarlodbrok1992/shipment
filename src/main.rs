use winapi::shared::minwindef::UINT;
use winapi::shared::minwindef::HINSTANCE;

use winapi::shared::ntdef::LPSTR;

use std::ffi::CString;

unsafe extern "system" {
    pub fn DLLWinMain(
        h_instance: HINSTANCE,
        h_prev_instance: HINSTANCE,
        lp_cmd_line: LPSTR,
        n_cmd_show: i32)-> i32;
}

fn main() {
    println!("Hello from Shipment!");

    unsafe {
        DLLWinMain(
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            1,
        );
    }
    
    println!("All went well, goodbye...");
}
