use uefi::prelude::*;
use uefi::proto::console::text::{Input, Output};

use crate::common;

pub fn shell(system_table: &'static SystemTable,stdin: &mut Input, stdout: &mut Output) -> ! {
    let hello = "hello".as_bytes();
    let mut hello_buf = [0u16;32];
    for i in 0..hello.len() {
        hello_buf[i] = hello[i] as u16;
    }
    
    loop {
        common::output_string("tawashell> ",stdout);
        let buf = common::get_string(system_table,stdin,stdout);
        common::output_string("\n",stdout);
        if common::is_str_equal(buf,hello_buf,stdout) {
            common::output_string("hello\n",stdout);
        } else {
            common::output_string("unknown_command\n",stdout);
        }   
    }
}

