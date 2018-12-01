use std::collections::HashSet;
use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn main() -> Result<()> {
    let mut historical_frequencies = HashSet::new();
    let mut frequency = 0;
    historical_frequencies.insert(frequency);

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    'outer: loop {
        for change in input.lines() {
            frequency += change.parse::<i32>()?;
            if historical_frequencies.contains(&frequency) {
                break 'outer;
            }
            historical_frequencies.insert(frequency);
        }
    }
    writeln!(io::stdout(), "{}", frequency)?;
    Ok(())
}
