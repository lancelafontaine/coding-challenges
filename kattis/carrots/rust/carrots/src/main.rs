use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdin_handle = stdin.lock();
    let stdout = io::stdout();
    let mut stdout_handle = stdout.lock();

    let line = stdin_handle.lines().next().unwrap()?;
    let output = line.split_whitespace().rev().next().unwrap();

    writeln!(stdout_handle, "{}", output)?;
    Ok(())
}
