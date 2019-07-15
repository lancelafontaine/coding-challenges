use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let num_y_switches = stdin_handle
        .lines()
        .next()
        .unwrap()?
        .split_whitespace()
        .map(str::parse::<u32>)
        .filter_map(std::result::Result::ok)
        .collect::<Vec<u32>>()[1];

    if num_y_switches % 2 == 0 {
        writeln!(stdout_handle, "possible")?;
    } else {
        writeln!(stdout_handle, "impossible")?;
    }
    Ok(())
}
