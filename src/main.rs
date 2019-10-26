#![no_std]
#![no_main]

extern crate uefi;
extern crate uefi_services;

use uefi::prelude::*;

#[no_mangle]
pub extern "C" fn efi_main(_image_handle: uefi::Handle, system_table: &'static SystemTable) -> ! {
    let stdout = system_table.stdout();
    let string = "hello uefi".as_bytes();
    let mut buf = [0u16; 32];

    for i in 0..string.len() {
        buf[i] = string[i] as u16;
    }

    stdout.output_string(buf.as_ptr());

    loop {}
}