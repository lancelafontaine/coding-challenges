use std::convert::TryInto;
use std::io::{self, BufRead, Write};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

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
    RelativeBaseOffset,
    Quit,
}

enum Mode {
    Position,
    Immediate,
    Relative,
}

struct Machine {
    pointer: usize,
    intcode: Intcode,
    receiver: Receiver<i64>,
    sender: Sender<i64>,
    relative_base: i64,
}

impl Machine {
    const MAX_INSTRUCTION_SIZE: usize = 4;

    fn new(pointer: usize, intcode: Intcode, receiver: Receiver<i64>, sender: Sender<i64>) -> Self {
        Self {
            pointer,
            intcode,
            receiver,
            sender,
            relative_base: 0,
        }
    }

    fn compute(&mut self) {
        let mut modes = Vec::new();

        loop {
            modes.clear();
            let opcode = self.get_opcode_with_modes(&mut modes);
            match opcode {
                Opcode::Add => self.add_operation(&modes),
                Opcode::Multiply => self.multiply_operation(&modes),
                Opcode::Input => self.input_operation(&modes),
                Opcode::Output => self.output_operation(&modes),
                Opcode::JumpIfTrue => self.jump_if_true_operation(&modes),
                Opcode::JumpIfFalse => self.jump_if_false_operation(&modes),
                Opcode::LessThan => self.less_than_operation(&modes),
                Opcode::Equals => self.equals_operation(&modes),
                Opcode::RelativeBaseOffset => self.relative_base_offset_operation(&modes),
                Opcode::Quit => break,
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
            9 => Opcode::RelativeBaseOffset,
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
                2 => modes.push(Mode::Relative),
                _ => unimplemented!(),
            }
        }
        opcode
    }
    fn add_operation(&mut self, modes: &[Mode]) {
        let read_1 = self.read_nth_parameter(1, modes);
        let read_2 = self.read_nth_parameter(2, modes);
        self.write_nth_parameter(3, modes, read_1 + read_2);
        self.pointer += 4;
    }

    fn multiply_operation(&mut self, modes: &[Mode]) {
        let read_1 = self.read_nth_parameter(1, modes);
        let read_2 = self.read_nth_parameter(2, modes);
        self.write_nth_parameter(3, modes, read_1 * read_2);
        self.pointer += 4;
    }

    fn input_operation(&mut self, modes: &[Mode]) {
        if let Ok(input) = self.receiver.recv() {
            self.write_nth_parameter(1, modes, input);
            self.pointer += 2;
        }
    }

    fn output_operation(&mut self, modes: &[Mode]) {
        let output = self.read_nth_parameter(1, modes);
        if self.sender.send(output).is_ok() {
            self.pointer += 2;
        } // otherwise, channel has hung up, program has ended
    }

    fn jump_if_true_operation(&mut self, modes: &[Mode]) {
        match self.read_nth_parameter(1, modes) {
            0 => self.pointer += 3,
            _ => self.pointer = self.read_nth_parameter(2, modes) as usize,
        }
    }

    fn jump_if_false_operation(&mut self, modes: &[Mode]) {
        match self.read_nth_parameter(1, modes) {
            0 => self.pointer = self.read_nth_parameter(2, modes) as usize,
            _ => self.pointer += 3,
        }
    }

    fn less_than_operation(&mut self, modes: &[Mode]) {
        let read_1 = self.read_nth_parameter(1, modes);
        let read_2 = self.read_nth_parameter(2, modes);
        self.write_nth_parameter(3, modes, if read_1 < read_2 { 1 } else { 0 });
        self.pointer += 4;
    }

    fn equals_operation(&mut self, modes: &[Mode]) {
        let read_1 = self.read_nth_parameter(1, modes);
        let read_2 = self.read_nth_parameter(2, modes);
        self.write_nth_parameter(3, modes, if read_1 == read_2 { 1 } else { 0 });
        self.pointer += 4;
    }

    fn relative_base_offset_operation(&mut self, modes: &[Mode]) {
        let offset = self.read_nth_parameter(1, modes);
        self.relative_base += offset;
        self.pointer += 2;
    }

    fn read_nth_parameter(&self, n: usize, modes: &[Mode]) -> i64 {
        let value = self.intcode[self.pointer + n];
        match modes[n - 1] {
            Mode::Position => *self.intcode.get(value as usize).or(Some(&0)).unwrap(),
            Mode::Immediate => value,
            Mode::Relative => *self
                .intcode
                .get((self.relative_base + value) as usize)
                .or(Some(&0))
                .unwrap(),
        }
    }

    fn write_nth_parameter(&mut self, n: usize, modes: &[Mode], value: i64) {
        let index = self.intcode[self.pointer + n];
        let index_value = match modes[n - 1] {
            Mode::Position => index as usize,
            Mode::Immediate => unimplemented!(),
            Mode::Relative => (self.relative_base + index) as usize,
        };
        if index_value > self.intcode.len() - 1 {
            self.intcode.resize(index_value + 1, 0);
        }
        self.intcode[index_value] = value;
    }
}

struct AmplifierSystem {
    amplifiers: Vec<Machine>,
    global_receiver: Receiver<i64>,
    global_sender: Sender<i64>,
}

impl AmplifierSystem {
    const NUMBER_OF_AMPLIFIERS: usize = 1;

    fn new(intcode: &[i64]) -> Self {
        let mut senders = Vec::with_capacity(Self::NUMBER_OF_AMPLIFIERS + 1);
        let mut receivers = Vec::with_capacity(Self::NUMBER_OF_AMPLIFIERS + 1);
        for _ in 0..=Self::NUMBER_OF_AMPLIFIERS {
            let (sender, receiver) = channel();
            senders.push(sender);
            receivers.push(receiver);
        }

        let global_receiver = receivers.pop().unwrap();
        let global_sender = senders.remove(0);
        senders.reverse();
        receivers.reverse();

        let amplifiers = (0..Self::NUMBER_OF_AMPLIFIERS).fold(Vec::new(), |mut vec, _| {
            vec.push(Machine::new(
                0,
                intcode.to_vec(),
                receivers.pop().unwrap(),
                senders.pop().unwrap(),
            ));
            vec
        });
        Self {
            amplifiers,
            global_sender,
            global_receiver,
        }
    }

    fn compute(&mut self, input: i64) -> Vec<i64> {
        self.global_sender.send(input).unwrap();
        while let Some(mut amplifier) = self.amplifiers.pop() {
            thread::spawn(move || {
                amplifier.compute();
            });
        }
        self.listen_for_output()
    }

    fn listen_for_output(&mut self) -> Vec<i64> {
        let mut system_output = Vec::new();
        while let Ok(output) = self.global_receiver.recv() {
            system_output.push(output);
        }
        system_output
    }
}

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let mut input_string = String::new();
    stdin_handle.read_line(&mut input_string)?;
    let intcode = input_string
        .split(',')
        .map(|s| s.trim().parse::<i64>())
        .filter_map(std::result::Result::ok)
        .collect::<Vec<i64>>();

    let mut amplifier_system = AmplifierSystem::new(&intcode);
    let output = amplifier_system
        .compute(1)
        .iter()
        .map(i64::to_string)
        .collect::<Vec<String>>()
        .join(",");

    writeln!(stdout_handle, "{}", output)?;
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
