use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();
    const INITIAL_NUM_POINT_ON_SIDE_OF_SQUARE: u32 = 2;

    let num_iterations = stdin_handle
        .lines()
        .next()
        .unwrap()?
        .parse::<u32>()
        .unwrap();

    let mut num_points_on_side_of_square = INITIAL_NUM_POINT_ON_SIDE_OF_SQUARE;
    for _ in 1..=num_iterations {
        num_points_on_side_of_square = (num_points_on_side_of_square * 2) - 1;
    }
    let num_intersecting_points_within_square = num_points_on_side_of_square.pow(2);

    writeln!(stdout_handle, "{}", num_intersecting_points_within_square)?;
    Ok(())
}
