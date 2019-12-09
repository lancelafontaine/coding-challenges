use std::convert::TryInto;
use std::io::{self, BufRead, StdinLock, StdoutLock, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
type Intcode = Vec<i64>;
type Digit = u8;

enum Opcode {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
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
    const MAX_INSTRUCTION_SIZE: usize = 4;

    fn new(pointer: usize, intcode: Intcode) -> Self {
        Machine { pointer, intcode }
    }

    fn compute(
        &mut self,
        mut stdin_handle: StdinLock,
        mut stdout_handle: StdoutLock,
    ) -> Result<()> {
        let mut modes = Vec::new();

        loop {
            modes.clear();
            let opcode = self.get_opcode_with_modes(&mut modes);
            if let Some(output) = match opcode {
                Opcode::Add => self.add_operation(&modes),
                Opcode::Multiply => self.multiply_operation(&modes),
                Opcode::Input => self.input_operation(&mut stdin_handle),
                Opcode::Output => self.output_operation(&modes),
                Opcode::JumpIfTrue => self.jump_if_true_operation(&modes),
                Opcode::JumpIfFalse => self.jump_if_false_operation(&modes),
                Opcode::LessThan => self.less_than_operation(&modes),
                Opcode::Equals => self.equals_operation(&modes),
                Opcode::Quit => return Ok(()),
            } {
                writeln!(stdout_handle, "{}", output)?;
            }
        }
    }

    fn get_opcode_with_modes(&self, modes: &mut Vec<Mode>) -> Opcode {
        let raw_opcode = self.intcode[self.pointer];
        let opcode = match raw_opcode % 100 {
            1 => Opcode::Add,
            2 => Opcode::Multiply,
            3 => Opcode::Input,
            4 => Opcode::Output,
            5 => Opcode::JumpIfTrue,
            6 => Opcode::JumpIfFalse,
            7 => Opcode::LessThan,
            8 => Opcode::Equals,
            99 => Opcode::Quit,
            _ => unimplemented!(),
        };

        let mut raw_modes = digits(raw_opcode / 100);
        raw_modes.reverse();
        while raw_modes.len() < Self::MAX_INSTRUCTION_SIZE - 1 {
            raw_modes.push(0);
        }
        for raw_mode in raw_modes {
            match raw_mode {
                0 => modes.push(Mode::Position),
                1 => modes.push(Mode::Immediate),
                _ => unimplemented!(),
            }
        }
        opcode
    }
    fn add_operation(&mut self, modes: &[Mode]) -> Option<i64> {
        let read_1 = self.nth_parameter_with_modes(1, modes);
        let read_2 = self.nth_parameter_with_modes(2, modes);
        let write_index = self.nth_parameter_as_index(3);
        self.intcode[write_index] = read_1 + read_2;
        self.pointer += 4;
        None
    }

    fn multiply_operation(&mut self, modes: &[Mode]) -> Option<i64> {
        let read_1 = self.nth_parameter_with_modes(1, modes);
        let read_2 = self.nth_parameter_with_modes(2, modes);
        let write_index = self.nth_parameter_as_index(3);
        self.intcode[write_index] = read_1 * read_2;
        self.pointer += 4;
        None
    }

    fn input_operation(&mut self, stdin_handle: &mut StdinLock) -> Option<i64> {
        let mut input = String::new();
        stdin_handle.read_line(&mut input).unwrap();
        let write_index = self.nth_parameter_as_index(1);

        self.intcode[write_index] = input.trim().parse::<i64>().unwrap();
        self.pointer += 2;
        None
    }

    fn output_operation(&mut self, modes: &[Mode]) -> Option<i64> {
        let output = self.nth_parameter_with_modes(1, modes);
        self.pointer += 2;
        Some(output)
    }

    fn jump_if_true_operation(&mut self, modes: &[Mode]) -> Option<i64> {
        match self.nth_parameter_with_modes(1, modes) {
            0 => self.pointer += 3,
            _ => self.pointer = self.nth_parameter_with_modes(2, modes) as usize,
        }
        None
    }

    fn jump_if_false_operation(&mut self, modes: &[Mode]) -> Option<i64> {
        match self.nth_parameter_with_modes(1, modes) {
            0 => self.pointer = self.nth_parameter_with_modes(2, modes) as usize,
            _ => self.pointer += 3,
        }
        None
    }

    fn less_than_operation(&mut self, modes: &[Mode]) -> Option<i64> {
        let read_1 = self.nth_parameter_with_modes(1, modes);
        let read_2 = self.nth_parameter_with_modes(2, modes);
        let write_index = self.nth_parameter_as_index(3);
        self.intcode[write_index] = if read_1 < read_2 { 1 } else { 0 };
        self.pointer += 4;
        None
    }

    fn equals_operation(&mut self, modes: &[Mode]) -> Option<i64> {
        let read_1 = self.nth_parameter_with_modes(1, modes);
        let read_2 = self.nth_parameter_with_modes(2, modes);
        let write_index = self.nth_parameter_as_index(3);
        self.intcode[write_index] = if read_1 == read_2 { 1 } else { 0 };
        self.pointer += 4;
        None
    }

    fn nth_parameter_with_modes(&self, n: usize, modes: &[Mode]) -> i64 {
        let value = self.intcode[self.pointer + n];
        match modes[n - 1] {
            Mode::Position => self.intcode[value as usize],
            Mode::Immediate => value,
        }
    }

    fn nth_parameter_as_index(&self, n: usize) -> usize {
        self.intcode[self.pointer + n] as usize
    }
}

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdin_handle = stdin.lock();
    let stdout_handle = stdout.lock();

    let mut input = String::new();
    stdin_handle.read_line(&mut input)?;
    let intcode = input
        .split(',')
        .map(|s| s.trim().parse::<i64>())
        .filter_map(std::result::Result::ok)
        .collect::<Vec<i64>>();

    let mut machine = Machine::new(0, intcode);
    machine.compute(stdin_handle, stdout_handle)?;
    Ok(())
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
