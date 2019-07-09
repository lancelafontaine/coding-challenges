use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let lines = stdin_handle
        .lines()
        .filter_map(std::result::Result::ok)
        .collect::<Vec<String>>();
    for pairs in lines[1..].chunks_exact(2) {
        writeln!(stdout_handle, "{}", pairs[0])?;
        writeln!(stdout_handle, "{}", pairs[1])?;
        let difference =
            pairs[0]
                .chars()
                .zip(pairs[1].chars())
                .fold(String::new(), |mut acc, (c1, c2)| {
                    if c1 == c2 {
                        acc.push('.')
                    } else {
                        acc.push('*')
                    }
                    acc
                });
        writeln!(stdout_handle, "{}", difference)?;
    }
    Ok(())
}
