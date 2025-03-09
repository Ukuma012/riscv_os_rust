use crate::{getchar, putchar};

#[unsafe(no_mangle)]
fn main() {
    print("> ");
    let mut cmdline: [u8; 128] = [0; 128];
    let mut count = 0;
    loop {
        let ch = getchar() as u8;
        putchar(ch);
        if ch == b'\r' {
            cmdline[count] = b'\0';
            print("\n");
            break;
        } else {
            cmdline[count] = ch;
        }

        count += 1;
        if count == 128 {
            break;
        }
    }
    match core::str::from_utf8(&cmdline[..count]) {
        Ok(s) => {
            if s == "hello" {
                print("Hello world\n");
            }
        }
        Err(_) => print("command not found\n"),
    }
    print("\n");
}

fn print(s: &str) {
    for c in s.bytes() {
        putchar(c);
    }
}
