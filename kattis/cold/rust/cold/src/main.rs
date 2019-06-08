use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let mut stdin_lines = stdin_handle.lines();
    stdin_lines.next();
    let temperatures = stdin_lines.next().unwrap()?;
    let output = temperatures
        .split_whitespace()
        .map(str::parse::<i32>)
        .map(std::result::Result::unwrap)
        .filter(|i| *i < 0)
        .count();

    writeln!(stdout_handle, "{}", output)?;
    Ok(())
}
