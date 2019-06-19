use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let mut lines = stdin_handle.lines();

    let _ = lines.next();

    let qaly = lines.fold(0.0f32, |acc, line| {
        let input = line.unwrap();
        let parsed_input = input
            .split_whitespace()
            .take(2)
            .map(str::parse::<f32>)
            .filter_map(std::result::Result::ok)
            .collect::<Vec<f32>>();
        let quality_of_life = parsed_input[0];
        let num_years = parsed_input[1];
        acc + (quality_of_life * num_years)
    });

    writeln!(stdout_handle, "{:.3}", qaly)?;
    Ok(())
}
