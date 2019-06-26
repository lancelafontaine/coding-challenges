use std::collections::HashMap;
use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let input = stdin_handle.lines().next().unwrap()?;

    let card_count = input.chars().fold(HashMap::new(), |mut acc, c| {
        acc.entry(c).and_modify(|count| *count += 1).or_insert(1);
        acc
    });
    let num_points_from_exponentiation = card_count
        .values()
        .fold(0, |acc, &count: &i32| acc + count.pow(2));

    let num_points_from_set = if card_count.len() >= 3 {
        card_count
            .values()
            .fold(std::i32::MAX, |acc, &count| acc.min(count))
            * 7
    } else {
        0
    };

    let num_points = num_points_from_set + num_points_from_exponentiation;
    writeln!(stdout_handle, "{}", num_points)?;
    Ok(())
}
