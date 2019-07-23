use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let days_of_week_starting_from_jan_1_2009 = [
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
    ];

    let num_days_in_months_throughout_year_2009: [usize; 12] =
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    let input = stdin_handle.lines().next().unwrap()?;
    let parsed_input = input
        .split_whitespace()
        .map(str::parse::<usize>)
        .filter_map(std::result::Result::ok)
        .collect::<Vec<usize>>();
    let expected_day = parsed_input[0];
    let expected_month = parsed_input[1];

    let sum_days_in_prior_months = &num_days_in_months_throughout_year_2009[..expected_month - 1]
        .iter()
        .sum::<usize>();
    let day_of_year_2009 = sum_days_in_prior_months + expected_day;
    let index = ((day_of_year_2009 % 7) as i8) - 1;
    let index: usize = if index == -1 { 6usize } else { index as usize };
    let expected_day_of_week = days_of_week_starting_from_jan_1_2009[index];

    writeln!(stdout_handle, "{}", expected_day_of_week)?;
    Ok(())
}
