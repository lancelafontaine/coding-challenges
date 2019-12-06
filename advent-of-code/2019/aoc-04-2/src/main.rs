use std::convert::TryInto;
use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
type Digit = u8;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let input = stdin_handle
        .lines()
        .next()
        .unwrap()?
        .split('-')
        .map(|s| s.parse::<u64>())
        .filter_map(std::result::Result::ok)
        .take(2)
        .collect::<Vec<u64>>();
    let range = input[0]..=input[1];

    let output = range
        .filter(|i| satisfies_password_requirements(&digits(*i)))
        .count();
    writeln!(stdout_handle, "{}", output)?;
    Ok(())
}

fn satisfies_password_requirements(digits: &[Digit]) -> bool {
    digits.len() == 6
        && is_monotonically_increasing(digits)
        && has_exactly_two_same_adjacent_digits(digits)
}

fn is_monotonically_increasing(digits: &[Digit]) -> bool {
    let mut digits_iter = digits.iter();
    let mut previous_digit = digits_iter.next().unwrap();

    for digit in digits_iter {
        if digit < previous_digit {
            return false;
        }
        previous_digit = digit
    }
    true
}

fn has_exactly_two_same_adjacent_digits(digits: &[Digit]) -> bool {
    let mut digits_iter = digits.iter();
    let mut previous_digit = digits_iter.next().unwrap();
    let mut num_grouped_digits = 1;

    for digit in digits_iter {
        if digit == previous_digit {
            num_grouped_digits += 1;
        } else {
            if num_grouped_digits == 2 {
                return true;
            }
            num_grouped_digits = 1;
        }
        previous_digit = digit;
    }
    num_grouped_digits == 2
}

fn digits(number: u64) -> Vec<Digit> {
    fn digit_inner(number: u64, digits: &mut Vec<Digit>) {
        if number >= 10 {
            digit_inner(number / 10, digits);
        }
        digits.push((number % 10).try_into().unwrap());
    }
    let mut digits = Vec::new();
    digit_inner(number, &mut digits);
    digits
}
