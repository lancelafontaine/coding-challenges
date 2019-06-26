use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let mut lines = stdin_handle.lines();

    let num_mb_in_plan = lines.next().unwrap()?.parse::<i32>()?;
    let _ = lines.next();

    let output = lines
        .filter_map(std::result::Result::ok)
        .map(|s| s.parse::<i32>())
        .filter_map(std::result::Result::ok)
        .fold(0, |acc, i| acc + (num_mb_in_plan - i))
        + num_mb_in_plan;

    writeln!(stdout_handle, "{}", output)?;
    Ok(())
}
