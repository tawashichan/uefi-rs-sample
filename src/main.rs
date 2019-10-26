#![no_std]
#![no_main]

extern crate uefi;
extern crate uefi_services;

use uefi::prelude::*;
use uefi::proto::console::text::{Input,Output};

#[no_mangle]
pub extern "C" fn efi_main(_image_handle: uefi::Handle, system_table: &'static SystemTable) -> ! {
    let mut stdout = system_table.stdout();
    let mut stdin = system_table.stdin();

    loop {
        match stdin.read_key() {
            Ok(key) => match key {
                Some(key) => {
                    let mut buf = [0u16; 32];
                    buf[0] = key.unicode_char;
                    output_string_ptr(buf, &mut stdout);
                },
                None => {},
            },
            Err(err) => panic!()
        }
    }
}

fn output_string_ptr(buf: [u16; 32],stdout: &mut Output) {
    stdout.reset(false);
    stdout.output_string(buf.as_ptr());
}

fn output_string(string: &str,stdout: &mut Output) {
    let string = string.as_bytes();
    let mut buf = [0u16; 32];
    for i in 0..string.len() {
        buf[i] = string[i] as u16;
    }
    stdout.reset(false);
    stdout.output_string(buf.as_ptr());
}

fn echo_string(stdin: &mut Input,stdout: &mut Output) {
    stdout.reset(false);
    loop {}
}

fn input_string() {

}

