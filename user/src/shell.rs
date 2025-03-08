use crate::putchar;

#[unsafe(no_mangle)]
fn main() {
    loop {
        print("> ");
    }
}

fn print(s: &str) {
    for c in s.bytes() {
        putchar(c);
    }
}
