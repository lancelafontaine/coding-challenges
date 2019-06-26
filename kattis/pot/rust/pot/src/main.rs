use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let mut lines = stdin_handle.lines();
    let _ = lines.next();

    let output = lines
        .filter_map(std::result::Result::ok)
        .fold(0, |acc, mut s| {
            let exponent = s.pop().unwrap().to_digit(10).unwrap();
            let base = s.parse::<u32>().unwrap();
            acc + base.pow(exponent)
        });
    writeln!(stdout_handle, "{}", output)?;
    Ok(())
}
