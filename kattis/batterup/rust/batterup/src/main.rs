use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let mut lines = stdin_handle.lines();
    let _ = lines.next();
    let input = lines.next().unwrap()?;
    let at_bats = input
        .split_whitespace()
        .map(str::parse::<i32>)
        .filter_map(std::result::Result::ok)
        .filter(|&i| i >= 0)
        .collect::<Vec<i32>>();
    let sum: f64 = at_bats.iter().sum::<i32>() as f64;
    let slugging_percentage = sum / at_bats.len() as f64;

    writeln!(stdout_handle, "{}", slugging_percentage)?;
    Ok(())
}
