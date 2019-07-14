use std::collections::HashMap;
use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let mut card_count: HashMap<char, u32> = HashMap::new();

    let input = stdin_handle.lines().next().unwrap()?;
    input.split_whitespace().for_each(|card| {
        let rank = card.chars().next().unwrap();
        let temp = *card_count.entry(rank).or_insert(0) + 1;
        card_count.insert(rank, temp);
    });
    let output = card_count
        .values()
        .fold(0, |acc, &count| std::cmp::max(acc, count));

    writeln!(stdout_handle, "{}", output)?;
    Ok(())
}
