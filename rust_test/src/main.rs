use current_platform::{COMPILED_ON, CURRENT_PLATFORM};
use windows_sys::{
    core::*, Win32::Foundation::*, Win32::System::Threading::*, Win32::UI::WindowsAndMessaging::*,
};

fn main() {
    unsafe {
        let event = CreateEventW(std::ptr::null(), 1, 0, std::ptr::null());
        SetEvent(event);
        WaitForSingleObject(event, 0);
        CloseHandle(event);

        MessageBoxA(0, s!("Ansi"), s!("Caption"), MB_OK);
        MessageBoxW(0, w!("Wide"), w!("Caption"), MB_OK);
    }
    println!("Hello, world from {}! I was compiled on {}.", CURRENT_PLATFORM, COMPILED_ON);
}

//cross run --target x86_64-pc-windows-gnu