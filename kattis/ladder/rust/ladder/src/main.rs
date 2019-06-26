use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let input = stdin_handle.lines().next().unwrap()?;
    let parsed_input = input
        .split_whitespace()
        .map(str::parse::<f32>)
        .filter_map(std::result::Result::ok)
        .collect::<Vec<f32>>();
    let height = parsed_input[0];
    let angle = parsed_input[1];

    let output = (angle.to_radians().sin().powf(-1.0) * height).ceil() as u32;

    writeln!(stdout_handle, "{}", output)?;
    Ok(())
}
