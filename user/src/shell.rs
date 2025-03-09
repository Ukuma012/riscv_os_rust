use crate::{getchar, putchar};

#[unsafe(no_mangle)]
fn main() {
    print("> ");
    let mut cmdline: [u8; 128] = [0; 128];
    let mut count = 0;
    loop {
        let ch = getchar() as u8;
        putchar(ch);
    }
}

fn print(s: &str) {
    for c in s.bytes() {
        putchar(c);
    }
}
