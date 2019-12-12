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
    Quit,
}

enum Mode {
    Position,
    Immediate,
}

struct Machine {
    pointer: usize,
    intcode: Intcode,
    receiver: Receiver<i64>,
    sender: Sender<i64>,
}

impl Machine {
    const MAX_INSTRUCTION_SIZE: usize = 4;

    fn new(pointer: usize, intcode: Intcode, receiver: Receiver<i64>, sender: Sender<i64>) -> Self {
        Self {
            pointer,
            intcode,
            receiver,
            sender,
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
                Opcode::Input => self.input_operation(),
                Opcode::Output => self.output_operation(&modes),
                Opcode::JumpIfTrue => self.jump_if_true_operation(&modes),
                Opcode::JumpIfFalse => self.jump_if_false_operation(&modes),
                Opcode::LessThan => self.less_than_operation(&modes),
                Opcode::Equals => self.equals_operation(&modes),
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
    fn add_operation(&mut self, modes: &[Mode]) {
        let read_1 = self.nth_parameter_with_modes(1, modes);
        let read_2 = self.nth_parameter_with_modes(2, modes);
        let write_index = self.nth_parameter_as_index(3);
        self.intcode[write_index] = read_1 + read_2;
        self.pointer += 4;
    }

    fn multiply_operation(&mut self, modes: &[Mode]) {
        let read_1 = self.nth_parameter_with_modes(1, modes);
        let read_2 = self.nth_parameter_with_modes(2, modes);
        let write_index = self.nth_parameter_as_index(3);
        self.intcode[write_index] = read_1 * read_2;
        self.pointer += 4;
    }

    fn input_operation(&mut self) {
        let write_index = self.nth_parameter_as_index(1);
        if let Ok(input) = self.receiver.recv() {
            self.intcode[write_index] = input;
            self.pointer += 2;
        }
    }

    fn output_operation(&mut self, modes: &[Mode]) {
        let output = self.nth_parameter_with_modes(1, modes);
        if self.sender.send(output).is_ok() {
            self.pointer += 2;
        } // otherwise, channel has hung up, program has ended
    }

    fn jump_if_true_operation(&mut self, modes: &[Mode]) {
        match self.nth_parameter_with_modes(1, modes) {
            0 => self.pointer += 3,
            _ => self.pointer = self.nth_parameter_with_modes(2, modes) as usize,
        }
    }

    fn jump_if_false_operation(&mut self, modes: &[Mode]) {
        match self.nth_parameter_with_modes(1, modes) {
            0 => self.pointer = self.nth_parameter_with_modes(2, modes) as usize,
            _ => self.pointer += 3,
        }
    }

    fn less_than_operation(&mut self, modes: &[Mode]) {
        let read_1 = self.nth_parameter_with_modes(1, modes);
        let read_2 = self.nth_parameter_with_modes(2, modes);
        let write_index = self.nth_parameter_as_index(3);
        self.intcode[write_index] = if read_1 < read_2 { 1 } else { 0 };
        self.pointer += 4;
    }

    fn equals_operation(&mut self, modes: &[Mode]) {
        let read_1 = self.nth_parameter_with_modes(1, modes);
        let read_2 = self.nth_parameter_with_modes(2, modes);
        let write_index = self.nth_parameter_as_index(3);
        self.intcode[write_index] = if read_1 == read_2 { 1 } else { 0 };
        self.pointer += 4;
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

struct AmplifierSystem {
    amplifiers: Vec<Machine>,
    global_receiver: Receiver<i64>,
    global_sender: Sender<i64>,
}

impl AmplifierSystem {
    const NUMBER_OF_AMPLIFIERS: usize = 5;

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

    fn compute(&mut self, phases: Vec<u8>, input: i64) -> Option<i64> {
        for (i, phase) in phases.iter().enumerate() {
            if i == 0 {
                self.global_sender.send(*phase as i64).unwrap();
                self.global_sender.send(input).unwrap();
            } else {
                self.amplifiers[i - 1].sender.send(*phase as i64).unwrap();
            }
        }
        while let Some(mut amplifier) = self.amplifiers.pop() {
            thread::spawn(move || {
                amplifier.compute();
            });
        }
        self.listen_for_output()
    }

    fn listen_for_output(&mut self) -> Option<i64> {
        let mut system_output = None;
        while let Ok(output) = self.global_receiver.recv() {
            system_output = Some(output);
            if self.global_sender.send(output).is_err() {
                break;
            }
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

    let mut phases = (5..=9).collect::<Vec<u8>>();
    let mut phase_permutations = Vec::new();
    generate_permutations_without_repetition(&mut phases, &mut phase_permutations, 0);

    let output = phase_permutations
        .into_iter()
        .map(|permutation| {
            let mut amplifier_system = AmplifierSystem::new(&intcode);
            amplifier_system.compute(permutation, 0).unwrap()
        })
        .max()
        .unwrap();

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

fn generate_permutations_without_repetition(
    elements: &mut Vec<u8>,
    permutations: &mut Vec<Vec<u8>>,
    index: usize,
) {
    let length = elements.len();
    if index >= length {
        permutations.push(elements.clone());
        return;
    }

    for i in index..length {
        if !(index..i).any(|j| elements[j] == elements[i]) {
            let value_at_index = elements[index];
            let value_at_i = elements[i];
            elements[index] = value_at_i;
            elements[i] = value_at_index;
            generate_permutations_without_repetition(elements, permutations, index + 1);
            elements[index] = value_at_index;
            elements[i] = value_at_i;
        }
    }
}
