use crate::putchar;

#[unsafe(no_mangle)]
fn main() {
    print("Hello World from shell!\n");
}

fn print(s: &str) {
    for c in s.bytes() {
        putchar(c);
    }
}
