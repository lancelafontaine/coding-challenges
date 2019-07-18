use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let mut lines = stdin_handle.lines();

    let cost_per_square_meter = lines.next().unwrap().unwrap().parse::<f64>().unwrap();
    let _ = lines.next();

    let mut total_cost = 0f64;
    for line in lines {
        let line = line.unwrap();
        let parsed_input = line
            .split_whitespace()
            .map(str::parse::<f64>)
            .filter_map(std::result::Result::ok)
            .collect::<Vec<f64>>();
        let area = parsed_input[0] * parsed_input[1];
        total_cost += area * cost_per_square_meter;
    }

    writeln!(stdout_handle, "{}", total_cost)?;
    Ok(())
}
