use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let input = stdin_handle.lines().next().unwrap()?;
    let split_input = input
        .split_whitespace()
        .map(String::from)
        .map(|s| s.parse())
        .filter_map(std::result::Result::ok)
        .collect::<Vec<u32>>();

    let num_articles = split_input[0];
    let required_impact_factor = split_input[1];
    let num_bribes = ((required_impact_factor - 1) * num_articles) + 1;
    writeln!(stdout_handle, "{}", num_bribes)?;
    Ok(())
}
