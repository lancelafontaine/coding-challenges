use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let input = stdin_handle.lines().next().unwrap()?;
    let mut n = input.parse::<u64>()?;

    while n % to_digits(n).iter().sum::<u64>() != 0 {
        n += 1;
    }
    writeln!(stdout_handle, "{}", n)?;
    Ok(())
}

fn to_digits(n: u64) -> Vec<u64> {
    fn to_digits_inner(n: u64, digits: &mut Vec<u64>) {
        if n >= 10 {
            to_digits_inner(n / 10, digits);
        }
        digits.push(n % 10);
    }
    let mut digits = Vec::new();
    to_digits_inner(n, &mut digits);
    digits
}
