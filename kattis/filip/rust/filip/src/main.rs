use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let input = stdin_handle.lines().next().unwrap()?;
    let output = input
        .split_whitespace()
        .map(|s| s.chars().rev().collect::<String>())
        .map(|s| s.parse::<i32>())
        .filter_map(std::result::Result::ok)
        .fold(0i32, |acc, i| if i > acc { i } else { acc });

    writeln!(stdout_handle, "{}", output)?;
    Ok(())
}
