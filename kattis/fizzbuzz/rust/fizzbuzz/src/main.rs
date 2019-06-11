use std::borrow::Cow;
use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let input = stdin_handle.lines().next().unwrap()?;
    let parsed_input = input
        .split_whitespace()
        .map(str::parse::<u32>)
        .filter_map(std::result::Result::ok)
        .take(3)
        .collect::<Vec<u32>>();
    let x = parsed_input[0];
    let y = parsed_input[1];
    let n = parsed_input[2];

    for i in 1..=n {
        let output: Cow<'static, str> = match (i % x, i % y) {
            (0, 0) => "FizzBuzz".into(),
            (0, _) => "Fizz".into(),
            (_, 0) => "Buzz".into(),
            (_, _) => i.to_string().into(),
        };
        writeln!(stdout_handle, "{}", output)?;
    }

    Ok(())
}
