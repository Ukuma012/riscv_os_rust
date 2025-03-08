use crate::putchar;

#[unsafe(no_mangle)]
fn main() {
    loop {}
}

fn print(s: &str) {
    for c in s.bytes() {
        putchar(c);
    }
}
