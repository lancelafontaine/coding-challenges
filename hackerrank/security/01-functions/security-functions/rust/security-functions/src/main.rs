use std::io::{self, BufRead};

fn main() {
    let mut input = String::new();
    read_line(&mut input);
    let input = to_u16(input);
    let result = input % 11;
    println!("{}", result);
}

fn read_line(mut string: &mut String) {
    let stdin = io::stdin();
    stdin.lock().read_line(&mut string).expect("Could not read line.");
}

fn to_u16(string: String) -> u16 {
    string.trim().parse().expect("Couldn't convert to an integer.")

}

