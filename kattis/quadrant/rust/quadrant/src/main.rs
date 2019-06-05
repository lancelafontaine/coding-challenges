use std::io::{self, BufRead, Write};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let coordinates = stdin_handle
        .lines()
        .take(2)
        .filter_map(::std::result::Result::ok)
        .map(|s| s.parse::<i32>())
        .filter_map(std::result::Result::ok)
        .collect::<Vec<i32>>();

    let quadrant = if coordinates[0] > 0 && coordinates[1] > 0 {
        1
    } else if coordinates[0] > 0 && coordinates[1] < 0 {
        4
    } else if coordinates[0] < 0 && coordinates[1] > 0 {
        2
    } else if coordinates[0] < 0 && coordinates[1] < 0 {
        3
    } else {
        panic!("input should never be 0")
    };

    writeln!(stdout_handle, "{}", quadrant)?;
    Ok(())
}
