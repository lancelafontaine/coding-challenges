use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let mut lines = stdin_handle.lines();
    let _ = lines.next();

    for line in lines {
        let line = line.unwrap();
        let input = line
            .split_whitespace()
            .map(str::parse::<i32>)
            .filter_map(std::result::Result::ok)
            .collect::<Vec<i32>>();

        if input[0] + input[1] == input[2]
            || input[0] * input[1] == input[2]
            || input[0] - input[1] == input[2]
            || input[1] - input[0] == input[2]
            || input[0] as f32 / input[1] as f32 == input[2] as f32
            || input[1] as f32 / input[0] as f32 == input[2] as f32
        {
            writeln!(stdout_handle, "Possible")?;
        } else {
            writeln!(stdout_handle, "Impossible")?;
        };
    }
    Ok(())
}
