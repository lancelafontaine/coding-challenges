use std::collections::HashMap;
use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut twice_count = 0;
    let mut thrice_count = 0;
    let mut char_frequency = HashMap::new();

    for word in input.lines() {
        for character in word.chars() {
            char_frequency
                .entry(character)
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
        if char_frequency.values().any(|&count| count == 2) {
            twice_count += 1;
        }
        if char_frequency.values().any(|&count| count == 3) {
            thrice_count += 1;
        }
        char_frequency.clear();
    }
    let checksum = twice_count * thrice_count;
    writeln!(io::stdout(), "{}", checksum)?;
    Ok(())
}
