use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    const A_OPERATION: u8 = 0b110;
    const B_OPERATION: u8 = 0b011;
    const C_OPERATION: u8 = 0b101;
    let mut ball_position: u8 = 0b100;

    let input = stdin_handle.lines().next().unwrap()?;

    for c in input.chars() {
        match c {
            'A' => ball_position ^= A_OPERATION,
            'B' => ball_position ^= B_OPERATION,
            'C' => ball_position ^= C_OPERATION,
            _ => panic!(),
        }
        if ball_position != 0b111 {
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
        0b100 => "1",
        0b010 => "2",
        0b001 => "3",
        _ => panic!(),
    };

    writeln!(stdout_handle, "{}", output)?;
    Ok(())
}
