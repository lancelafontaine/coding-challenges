use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

const TIME_DECREMENT: i32 = 45;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdin_handle = stdin.lock();
    let stdout = io::stdout();
    let mut stdout_handle = stdout.lock();

    let input = stdin_handle.lines().next().unwrap()?;
    let parsed_input = input
        .split_whitespace()
        .map(str::parse::<i32>)
        .take(2)
        .filter_map(std::result::Result::ok)
        .collect::<Vec<i32>>();
    let hour = parsed_input[0];
    let minute = parsed_input[1];

    let new_hour = if minute - TIME_DECREMENT < 0 {
        modulo(hour - 1, 24)
    } else {
        hour
    };
    let new_minute = modulo(minute - TIME_DECREMENT, 60);

    writeln!(stdout_handle, "{} {}", new_hour, new_minute)?;
    Ok(())
}

fn modulo(x: i32, m: i32) -> i32 {
    (x % m + m) % m
}
