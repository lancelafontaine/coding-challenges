use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let mut lines = stdin_handle.lines();
    let _ = lines.next();

    for line in lines {
        let input = line
            .unwrap()
            .split_whitespace()
            .take(3)
            .map(str::parse::<i32>)
            .filter_map(std::result::Result::ok)
            .collect::<Vec<i32>>();
        let r = input[0];
        let e = input[1];
        let c = input[2];
        let output = if r > e - c {
            "do not advertise"
        } else if r < e - c {
            "advertise"
        } else {
            "does not matter"
        };
        writeln!(stdout_handle, "{}", output)?;
    }

    Ok(())
}
