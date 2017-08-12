use std::io::{self, BufRead};

fn main() {
    let mut input = String::new();
    read_line(&mut input);

    let mut chars : Vec<char> = input.trim().chars().collect();
    let mut should_restart = true ;

    while should_restart {
        should_restart = false;
        for index in 0..chars.len() {
            if index + 1 < chars.len() && chars[index] == chars[index + 1] {
                chars.remove(index + 1);
                chars.remove(index);
                should_restart = true;
                break;
            }
        }
    }
    if chars.is_empty() {
        println!("Empty String");
    } else {
        let output: String = chars.into_iter().collect();
        println!("{}", output);
    }
}

fn read_line(mut string: &mut String) {
    let stdin = io::stdin();
    stdin.lock().read_line(&mut string).expect("Couldn't read input.");
}
