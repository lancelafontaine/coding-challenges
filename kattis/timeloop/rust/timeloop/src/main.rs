use std::io::{self, BufRead, Write};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let num = stdin_handle
        .lines()
        .take(1)
        .collect::<std::result::Result<String, _>>()?
        .parse::<u32>()?;

    let output = (1..=num)
        .map(|i| format!("{} Abracadabra", i))
        .collect::<Vec<String>>()
        .join("\n");

    writeln!(stdout_handle, "{}", output)?;
    Ok(())
}
