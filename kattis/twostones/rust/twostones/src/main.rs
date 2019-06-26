use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let input = stdin_handle.lines().next().unwrap()?.parse::<u32>()?;
    let output = match input % 2 {
        1 => "Alice",
        0 => "Bob",
        _ => panic!(),
    };

    writeln!(stdout_handle, "{}", output)?;
    Ok(())
}
