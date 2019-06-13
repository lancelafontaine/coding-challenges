use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let input = stdin_handle.lines().next().unwrap()?;

    let mut previous_char_was_s = false;
    for c in input.chars() {
        if c == 's' {
            if previous_char_was_s {
                return Ok(writeln!(stdout_handle, "hiss")?);
            } else {
                previous_char_was_s = true
            }
        } else {
            previous_char_was_s = false
        }
    }
    writeln!(stdout_handle, "no hiss")?;
    Ok(())
}
