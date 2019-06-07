use std::io::{self, BufRead, Write};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn main() -> Result<()> {
    const NUM_DISTINCT_PIECES: usize = 6;
    const EXPECTED_NUM_KING: i32 = 1;
    const EXPECTED_NUM_QUEEN: i32 = 1;
    const EXPECTED_NUM_ROOKS: i32 = 2;
    const EXPECTED_NUM_BISHOPS: i32 = 2;
    const EXPECTED_NUM_KNIGHTS: i32 = 2;
    const EXPECTED_NUM_PAWNS: i32 = 8;
    const EXPECTED_NUM_PIECES: [i32; NUM_DISTINCT_PIECES] = [
        EXPECTED_NUM_KING,
        EXPECTED_NUM_QUEEN,
        EXPECTED_NUM_ROOKS,
        EXPECTED_NUM_BISHOPS,
        EXPECTED_NUM_KNIGHTS,
        EXPECTED_NUM_PAWNS,
    ];

    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let input = stdin_handle.lines().next().unwrap()?;
    let num_pieces = input
        .split_whitespace()
        .map(str::trim)
        .map(str::parse)
        .collect::<std::result::Result<Vec<i32>, std::num::ParseIntError>>()?;

    let output = EXPECTED_NUM_PIECES
        .iter()
        .zip(num_pieces)
        .take(NUM_DISTINCT_PIECES)
        .map(|(expected_num, actual_num)| *expected_num - actual_num)
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    writeln!(stdout_handle, "{}", output)?;
    Ok(())
}
