use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let input = stdin_handle.lines().next().unwrap()?;

    let mut previous_character = None;
    let output = input
        .chars()
        .filter(|c| {
            if previous_character == Some(*c) {
                return false;
            }
            previous_character = Some(*c);
            true
        })
        .collect::<String>();

    writeln!(stdout_handle, "{}", output)?;
    Ok(())
}
