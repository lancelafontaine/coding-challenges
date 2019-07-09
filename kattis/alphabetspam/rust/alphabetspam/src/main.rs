use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let mut whitespace_count = 0;
    let mut lowercase_count = 0;
    let mut uppercase_count = 0;
    let mut symbol_count = 0;

    for c in stdin_handle.lines().next().unwrap()?.chars() {
        if c == '_' {
            whitespace_count += 1
        } else if c.is_lowercase() {
            lowercase_count += 1
        } else if c.is_uppercase() {
            uppercase_count += 1
        } else {
            symbol_count += 1
        }
    }

    let total_count = whitespace_count + lowercase_count + uppercase_count + symbol_count;

    writeln!(
        stdout_handle,
        "{}",
        whitespace_count as f32 / total_count as f32
    )?;
    writeln!(
        stdout_handle,
        "{}",
        lowercase_count as f32 / total_count as f32
    )?;
    writeln!(
        stdout_handle,
        "{}",
        uppercase_count as f32 / total_count as f32
    )?;
    writeln!(
        stdout_handle,
        "{}",
        symbol_count as f32 / total_count as f32
    )?;

    Ok(())
}
