use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let mut intcode = stdin_handle
        .lines()
        .next()
        .unwrap()?
        .split(',')
        .map(|s| s.parse::<u64>())
        .filter_map(std::result::Result::ok)
        .collect::<Vec<u64>>();

    for opcode_index in (0..intcode.len()).step_by(4) {
        if perform_operation(&mut intcode, opcode_index).is_none() {
            break;
        }
    }

    let output = intcode
        .into_iter()
        .map(|u| format!("{}", u))
        .collect::<Vec<String>>()
        .join(",");
    writeln!(stdout_handle, "{}", output)?;
    Ok(())
}

fn perform_operation(intcode: &mut Vec<u64>, i: usize) -> Option<()> {
    match intcode[i] {
        opcode @ 1 | opcode @ 2 => {
            let read_index_1 = intcode[(i + 1) as usize] as usize;
            let read_index_2 = intcode[(i + 2) as usize] as usize;
            let write_index = intcode[(i + 3) as usize] as usize;
            match opcode {
                1 => intcode[write_index] = intcode[read_index_1] + intcode[read_index_2],
                2 => intcode[write_index] = intcode[read_index_1] * intcode[read_index_2],
                _ => unimplemented!(),
            }
            Some(())
        }
        99 => None,
        _ => unimplemented!(),
    }
}
