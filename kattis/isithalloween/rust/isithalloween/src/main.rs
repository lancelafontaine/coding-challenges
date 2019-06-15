use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let input = stdin_handle.lines().next().unwrap()?;
    let output = if input == "OCT 31" || input == "DEC 25" {
        "yup"
    } else {
        "nope"
    };

    writeln!(stdout_handle, "{}", output)?;
    Ok(())
}
