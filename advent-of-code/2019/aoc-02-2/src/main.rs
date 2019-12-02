use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let intcode = stdin_handle
        .lines()
        .next()
        .unwrap()?
        .split(',')
        .map(|s| s.parse::<u64>())
        .filter_map(std::result::Result::ok)
        .collect::<Vec<u64>>();

    for noun in 0..100 {
        for verb in 0..100 {
            let mut new_intcode = intcode.clone();
            new_intcode[1] = noun;
            new_intcode[2] = verb;
            part1(&mut new_intcode);
            if new_intcode[0] == 19_690_720 {
                writeln!(stdout_handle, "{}", (100 * new_intcode[1]) + new_intcode[2])?;
                return Ok(());
            }
        }
    }
    Ok(())
}

fn part1(mut intcode: &mut Vec<u64>) {
    for opcode_index in (0..intcode.len()).step_by(4) {
        if perform_operation(&mut intcode, opcode_index).is_none() {
            break;
        }
    }
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
