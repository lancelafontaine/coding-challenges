use std::io::{self, BufRead, Write};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn main() -> Result<()> {
    let mut frequency = 0;

    let stdin = io::stdin();
    for change in stdin.lock().lines() {
        frequency += change?.parse::<i32>()?;
    }
    writeln!(io::stdout(), "{}", frequency)?;
    Ok(())
}
