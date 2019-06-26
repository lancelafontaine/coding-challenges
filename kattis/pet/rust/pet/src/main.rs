use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let output = stdin_handle
        .lines()
        .filter_map(std::result::Result::ok)
        .enumerate()
        .map(|(i, s)| {
            let u = s
                .split_whitespace()
                .map(str::parse::<u32>)
                .filter_map(std::result::Result::ok)
                .sum();
            (i, u)
        })
        .fold((0, u32::min_value()), |(a, b), (i, u)| {
            if b > u {
                (a, b)
            } else {
                (i + 1, u)
            }
        });

    writeln!(stdout_handle, "{} {}", output.0, output.1)?;
    Ok(())
}
