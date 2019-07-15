use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let mut rolls = stdin_handle
        .lines()
        .next()
        .unwrap()?
        .split_whitespace()
        .map(|s| s.parse::<u32>())
        .filter_map(std::result::Result::ok)
        .collect::<Vec<u32>>();

    if rolls[0] > rolls[1] {
        let temp = rolls[0];
        rolls[0] = rolls[1];
        rolls[1] = temp;
    }

    for i in (rolls[0] + 1)..=(rolls[1] + 1) {
        writeln!(stdout_handle, "{}", i)?;
    }

    Ok(())
}
