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
        let line = line?;
        let turtle_sequence = line
            .split_whitespace()
            .map(str::parse::<u32>)
            .filter_map(std::result::Result::ok)
            .collect::<Vec<u32>>();

        let mut num_turtle_imports = 0;
        for i in 1..turtle_sequence.len() {
            let maximum_turtle_growth = turtle_sequence[i - 1] * 2;
            if maximum_turtle_growth < turtle_sequence[i] {
                num_turtle_imports += turtle_sequence[i] - maximum_turtle_growth
            }
        }
        writeln!(stdout_handle, "{}", num_turtle_imports)?;
    }
    Ok(())
}
