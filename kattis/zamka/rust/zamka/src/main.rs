use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let mut lines = stdin_handle.lines();
    let l = lines.next().unwrap()?.parse::<i32>()?;
    let d = lines.next().unwrap()?.parse::<i32>()?;
    let x = lines.next().unwrap()?.parse::<i32>()?;

    let mut lowest = 0;
    let mut highest = 0;

    for i in l..=d {
        if sum_of_digits(i) == x {
            if lowest == 0 {
                lowest = i;
            }
            highest = i;
        }
    }

    writeln!(stdout_handle, "{}\n{}", lowest, highest)?;
    Ok(())
}

fn sum_of_digits(i: i32) -> i32 {
    let mut i = i;
    let mut sum = 0;
    while i != 0 {
        sum += i % 10;
        i = i / 10;
    }
    sum
}
