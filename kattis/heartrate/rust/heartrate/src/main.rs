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
        let parsed_input = line
            .unwrap()
            .split_whitespace()
            .map(str::parse::<f32>)
            .filter_map(std::result::Result::ok)
            .collect::<Vec<f32>>();
        let beats = parsed_input[0];
        let time = parsed_input[1];

        writeln!(
            stdout_handle,
            "{:.4} {:.4} {:.4}",
            (beats - 1.0) / time * 60.0,
            beats / time * 60.0,
            (beats + 1.0) / time * 60.0
        )?;
    }

    Ok(())
}
