use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let input = stdin_handle
        .lines()
        .next()
        .unwrap()?
        .chars()
        .collect::<Vec<char>>();
    let output = input.chunks(3).fold(0, |acc, chars| {
        let mut local_count = 0;
        if chars.len() >= 1 && chars[0] != 'P' {
            local_count += 1
        }
        if chars.len() >= 2 && chars[1] != 'E' {
            local_count += 1
        }
        if chars.len() == 3 && chars[2] != 'R' {
            local_count += 1
        }
        acc + local_count
    });

    writeln!(stdout_handle, "{}", output)?;
    Ok(())
}
