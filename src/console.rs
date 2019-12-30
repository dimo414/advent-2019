pub struct Console;

impl Console {
    #[inline]
    pub fn init() -> Console {
        if interactive!() {
            print!("\u{001B}[?25l"); // hide cursor
        }
        Console
    }
}

// Take advantage of Drop to (attempt to) unconditionally restore the cursor. See
// https://stackoverflow.com/a/57860708/113632 for more, or
// https://doc.rust-lang.org/std/panic/fn.catch_unwind.html for another potential approach.
#[cfg(any(feature="interactive", all(debug_assertions, not(test))))]
impl Drop for Console {
    fn drop(&mut self) {
        print!("\u{001B}[?25h"); // restore cursor
    }
}