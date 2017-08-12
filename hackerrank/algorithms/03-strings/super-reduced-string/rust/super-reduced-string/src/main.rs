use std::io::{self, BufRead};

fn main() {
    let mut input = String::new();
    read_line(&mut input);

    let mut stack: Vec<char> = Vec::new();

    for char in input.trim().chars() {
        if stack.last() == Some(&char) {
            stack.pop();
        } else {
            stack.push(char);
        }
    }
    let output: String = stack.into_iter().collect();
    if output == "" {
        println!("Empty String");
    } else {
        println!("{}", output);
    }
}

fn read_line(mut string: &mut String) {
    let stdin = io::stdin();
    stdin.lock().read_line(&mut string).expect("Couldn't read input.");
}
