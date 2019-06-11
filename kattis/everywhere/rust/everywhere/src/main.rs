use std::collections::HashSet;
use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();
    let mut lines = stdin_handle.lines();

    let num_test_cases = lines.next().unwrap()?.parse::<u32>().unwrap();
    for _ in 0..num_test_cases {
        let mut unique_cities = HashSet::new();
        let num_cities = lines.next().unwrap()?.parse::<u32>().unwrap();
        for _ in 0..num_cities {
            let city = lines.next().unwrap()?;
            unique_cities.insert(city);
        }
        writeln!(stdout_handle, "{}", unique_cities.len())?;
    }
    Ok(())
}
