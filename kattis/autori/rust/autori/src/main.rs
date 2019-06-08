use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let input = stdin_handle.lines().next().unwrap()?;

    let mut should_keep_next_character = true;
    let chars = input.chars();
    let output = chars
        .filter(|c| {
            if should_keep_next_character {
                should_keep_next_character = false;
                return true;
            }
            if c == &'-' {
                should_keep_next_character = true;
            }
            false
        })
        .collect::<String>();

    writeln!(stdout_handle, "{}", output)?;
    Ok(())
}
