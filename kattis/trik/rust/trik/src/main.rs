use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    // 110
    const A_OPERATION: u8 = 6;
    // 011
    const B_OPERATION: u8 = 3;
    // 101
    const C_OPERATION: u8 = 5;
    // 100
    let mut ball_position: u8 = 4;

    let input = stdin_handle.lines().next().unwrap()?;

    for c in input.chars() {
        match c {
            'A' => ball_position ^= A_OPERATION,
            'B' => ball_position ^= B_OPERATION,
            'C' => ball_position ^= C_OPERATION,
            _ => panic!(),
        }
        // 111
        if ball_position != 7 {
            continue;
        }
        match c {
            'A' => ball_position ^= A_OPERATION,
            'B' => ball_position ^= B_OPERATION,
            'C' => ball_position ^= C_OPERATION,
            _ => panic!(),
        }
    }

    let output = match ball_position {
        // 100
        4 => "1",
        // 010
        2 => "2",
        // 001
        1 => "3",
        _ => panic!(),
    };

    writeln!(stdout_handle, "{}", output)?;
    Ok(())
}
