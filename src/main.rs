#![no_std]
#![no_main]

extern crate uefi;
extern crate uefi_services;

mod shell;
mod common;

use uefi::prelude::*;
use uefi::proto::console::text::{Input, Output};

#[no_mangle]
pub extern "C" fn efi_main(_image_handle: uefi::Handle, system_table: &'static SystemTable) -> ! {
    let mut stdout = system_table.stdout();
    let mut stdin = system_table.stdin();
    shell::shell(system_table, stdin, stdout)
}