use std::io::{self, BufRead};

fn main() {
    let mut _discard = String::new();
    read_line(&mut _discard);

    let mut input = String::new();
    read_line(&mut input);

    let output = input.trim().split_whitespace().rev().collect::<Vec<&str>>().join(" ");
    println!("{}", output);
}

fn read_line(mut string: &mut String) {
    let stdin = io::stdin();
    stdin.lock().read_line(&mut string).expect("Couldn't read line from stdin into string");
}
