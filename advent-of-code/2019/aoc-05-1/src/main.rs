use std::convert::TryInto;
use std::io::{self, BufRead, StdinLock, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
type Intcode = Vec<i64>;
type Digit = u8;

const MAX_INSTRUCTION_SIZE: usize = 4;

enum Opcode {
    Add,
    Multiply,
    Input,
    Output,
    Quit,
}

enum Mode {
    Position,
    Immediate,
}

struct Machine {
    pointer: usize,
    intcode: Intcode,
}

impl Machine {
    fn new(pointer: usize, intcode: Intcode) -> Self {
        Machine { pointer, intcode }
    }
}

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let mut input = String::new();
    stdin_handle.read_line(&mut input)?;
    let intcode = input
        .split(',')
        .map(|s| s.parse::<i64>())
        .filter_map(std::result::Result::ok)
        .collect::<Vec<i64>>();

    let mut machine = Machine::new(0, intcode);

    loop {
        let (opcode, modes) = get_opcode_and_modes(&mut machine);
        if let Some(output) = match opcode {
            Opcode::Add => add_operation(&mut machine, modes),
            Opcode::Multiply => multiply_operation(&mut machine, modes),
            Opcode::Input => input_operation(&mut machine, &mut stdin_handle),
            Opcode::Output => output_operation(&mut machine),
            Opcode::Quit => break,
        } {
            writeln!(stdout_handle, "{}", output)?;
        }
    }

    Ok(())
}

fn get_opcode_and_modes(machine: &mut Machine) -> (Opcode, Vec<Mode>) {
    let raw_opcode = machine.intcode[machine.pointer];
    let opcode = match raw_opcode % 100 {
        1 => Opcode::Add,
        2 => Opcode::Multiply,
        3 => Opcode::Input,
        4 => Opcode::Output,
        99 => Opcode::Quit,
        _ => unimplemented!(),
    };

    let mut raw_modes = digits(raw_opcode / 100);
    raw_modes.reverse();
    while raw_modes.len() < MAX_INSTRUCTION_SIZE - 1 {
        raw_modes.push(0);
    }
    let modes = raw_modes
        .iter()
        .map(|raw_mode| match raw_mode {
            0 => Mode::Position,
            1 => Mode::Immediate,
            _ => unimplemented!(),
        })
        .collect::<Vec<Mode>>();

    (opcode, modes)
}

fn add_operation(machine: &mut Machine, modes: Vec<Mode>) -> Option<i64> {
    let read_1 = {
        let value = machine.intcode[machine.pointer + 1];
        match modes[0] {
            Mode::Position => machine.intcode[value as usize],
            Mode::Immediate => value,
        }
    };
    let read_2 = {
        let value = machine.intcode[machine.pointer + 2];
        match modes[1] {
            Mode::Position => machine.intcode[value as usize],
            Mode::Immediate => value,
        }
    };
    let write_index = machine.intcode[machine.pointer + 3] as usize;

    machine.intcode[write_index] = read_1 + read_2;
    machine.pointer += 4;
    None
}

fn multiply_operation(machine: &mut Machine, modes: Vec<Mode>) -> Option<i64> {
    let read_1 = {
        let value = machine.intcode[machine.pointer + 1];
        match modes[0] {
            Mode::Position => machine.intcode[value as usize],
            Mode::Immediate => value,
        }
    };
    let read_2 = {
        let value = machine.intcode[machine.pointer + 2];
        match modes[1] {
            Mode::Position => machine.intcode[value as usize],
            Mode::Immediate => value,
        }
    };
    let write_index = machine.intcode[machine.pointer + 3] as usize;

    machine.intcode[write_index] = read_1 * read_2;
    machine.pointer += 4;
    None
}

fn input_operation(machine: &mut Machine, stdin_handle: &mut StdinLock) -> Option<i64> {
    let mut input = String::new();
    stdin_handle.read_line(&mut input).unwrap();
    let write_index = machine.intcode[machine.pointer + 1] as usize;
    machine.intcode[write_index] = input.trim().parse::<i64>().unwrap();
    machine.pointer += 2;
    None
}

fn output_operation(machine: &mut Machine) -> Option<i64> {
    let read_index = machine.intcode[machine.pointer + 1] as usize;
    let output = machine.intcode[read_index];
    machine.pointer += 2;
    Some(output)
}

fn digits(number: i64) -> Vec<Digit> {
    fn digit_inner(number: i64, digits: &mut Vec<Digit>) {
        if number >= 10 {
            digit_inner(number / 10, digits);
        }
        digits.push((number % 10).try_into().unwrap());
    }
    let mut digits = Vec::new();
    digit_inner(number, &mut digits);
    digits
}
