use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let mut lines = stdin_handle.lines();
    let input = lines.next().unwrap()?;
    let parsed_input = input
        .split_whitespace()
        .map(str::parse::<f32>)
        .take(3)
        .filter_map(std::result::Result::ok)
        .collect::<Vec<f32>>();
    let width = parsed_input[1];
    let height = parsed_input[2];
    let hypotenuse = (width.powi(2) + height.powi(2)).sqrt();

    for line in lines {
        let match_length = line?.parse::<f32>()?;
        if match_length <= hypotenuse {
            writeln!(stdout_handle, "DA")?;
        } else {
            writeln!(stdout_handle, "NE")?;
        }
    }
    Ok(())
}
