use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let fuel_total: u64 = stdin_handle
        .lines()
        .filter_map(std::result::Result::ok)
        .map(|s| s.parse::<u64>())
        .filter_map(std::result::Result::ok)
        .map(|u| (u / 3) - 2)
        .sum();

    writeln!(stdout_handle, "{}", fuel_total)?;
    Ok(())
}
