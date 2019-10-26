use uefi::prelude::*;
use uefi::proto::console::text::{Input, Output};

pub fn get_char(system_table: &'static SystemTable,stdin: &mut Input) -> u16 {
    let event = stdin.wait_for_key_event();
    system_table.boot.wait_for_event(&mut [event]);

    match stdin.read_key() {
        Ok(key) => match key {
            Some(key) => {
                return key.unicode_char
            }
            None => panic!(),
        },
        Err(err) => panic!(),
    };
}

pub fn get_string(system_table: &'static SystemTable,stdin: &mut Input,stdout: &mut Output) -> [u16;32] {
    let mut buf = [0u16;32];
    let mut index = 0;
    loop {
        let ch = get_char(system_table,stdin);
        buf[index] = ch;
        output_char(ch,stdout);
        if buf[index] == '\r' as u16 {
            buf[index] = '\0' as u16;
            return buf
        }
        index += 1;
    };
}

pub fn output_buf(buf: [u16;32], stdout: &mut Output) {
    stdout.output_string(buf.as_ptr());
}

pub fn output_char(ch: u16,stdout: &mut Output) {
    let mut buf = [0u16;2];
    buf[0] = ch;
    stdout.output_string(buf.as_ptr());
}

pub fn output_string(string: &str, stdout: &mut Output) {
    let string = string.as_bytes();
    let mut buf = [0u16; 32];
    for i in 0..string.len() {
        buf[i] = string[i] as u16;
    }
    stdout.output_string(buf.as_ptr());
}

pub fn is_str_equal(buf1: [u16;32],buf2: [u16;32],stdout: &mut Output) -> bool {
    //output_buf(buf1,stdout);
    //output_buf(buf2,stdout);
    for b1 in buf1.iter() {
        for b2 in buf2.iter() {
            if b1 != b2 {
                return false
            }
        }
    }
    true
}