use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let mut lines = stdin_handle.lines();
    let _ = lines.next();

    for line in lines {
        let factorial = line?.parse::<u64>()?;
        let output = (2..=factorial).fold(1u64, |acc, factor| (acc * factor) % 10);
        writeln!(stdout_handle, "{}", output)?;
    }
    Ok(())
}
