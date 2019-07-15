use std::collections::HashMap;
use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let mut lines = stdin_handle.lines();

    let mut trump_card_values = HashMap::new();
    trump_card_values.insert('A', 11);
    trump_card_values.insert('K', 4);
    trump_card_values.insert('Q', 3);
    trump_card_values.insert('J', 20);
    trump_card_values.insert('T', 10);
    trump_card_values.insert('9', 14);

    let mut non_trump_card_values = HashMap::new();
    non_trump_card_values.insert('A', 11);
    non_trump_card_values.insert('K', 4);
    non_trump_card_values.insert('Q', 3);
    non_trump_card_values.insert('J', 2);
    non_trump_card_values.insert('T', 10);

    let temp_input = lines.next().unwrap()?;
    let trump_suit = temp_input.split_whitespace().collect::<Vec<&str>>()[1]
        .chars()
        .next()
        .unwrap();

    let output = lines
        .filter_map(std::result::Result::ok)
        .fold(0, |acc, line| {
            let mut chars = line.chars();
            let card_value = chars.next().unwrap();
            let card_suit = chars.next().unwrap();

            let map = if card_suit == trump_suit {
                &mut trump_card_values
            } else {
                &mut non_trump_card_values
            };
            acc + *map.entry(card_value).or_default()
        });

    writeln!(stdout_handle, "{}", output)?;
    Ok(())
}
