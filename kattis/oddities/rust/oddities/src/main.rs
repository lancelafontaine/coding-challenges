use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let mut lines = stdin_handle.lines();
    let _ = lines.next().unwrap()?;
    for line in lines {
        let num = line?.parse::<i32>()?;
        if num.abs() % 2 == 0 {
            writeln!(stdout_handle, "{} is even", num)?;
        } else {
            writeln!(stdout_handle, "{} is odd", num)?;
        }
    }
    Ok(())
}
