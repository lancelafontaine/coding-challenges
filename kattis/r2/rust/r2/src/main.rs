use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let input = stdin_handle.lines().next().unwrap()?;
    let parsed_input = input
        .split_whitespace()
        .map(str::parse::<i32>)
        .filter_map(std::result::Result::ok)
        .collect::<Vec<i32>>();
    let r1 = parsed_input[0];
    let s = parsed_input[1];

    let output = (s * 2) - r1;
    writeln!(stdout_handle, "{}", output)?;
    Ok(())
}
