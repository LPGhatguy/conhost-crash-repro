extern crate all_term;

fn main() {
    // All-Term enables ANSI mode right away, if available
    let terminal = all_term::terminal();

    {
        let mut handle = terminal.lock().unwrap();

        // If these two lines are flipped, the crash goes away.
        // enable_alternate_screen does so via ANSI escape codes
        // enable_raw_mode sets a couple flags on the stdout handle using
        // GetConsoleMode and SetConsoleMode.
        handle.enable_alternate_screen();
        handle.enable_raw_mode();
    }
}
