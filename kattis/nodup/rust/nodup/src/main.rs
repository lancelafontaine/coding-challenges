use std::collections::HashSet;
use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let input = stdin_handle.lines().next().unwrap()?;
    let parsed_input = input.split_whitespace();
    let mut seen_words = HashSet::new();
    for word in parsed_input {
        if seen_words.contains(word) {
            return Ok(writeln!(stdout_handle, "no")?);
        }
        seen_words.insert(word);
    }

    writeln!(stdout_handle, "yes")?;
    Ok(())
}
