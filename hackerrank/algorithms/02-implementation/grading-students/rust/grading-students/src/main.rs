use std::io::{self, BufRead};

fn main() {
    let mut num_grades = String::new();
    read_line(&mut num_grades);
    let num_grades = to_u8(&num_grades);

    for _ in 0..num_grades {
        let mut grade = String::new();
        read_line(&mut grade);
        let grade = to_u8(&grade);
        println!("{}", round(grade));
    }
}

fn read_line(mut string: &mut String) {
    let stdin = io::stdin();
    stdin.lock().read_line(&mut string).expect("Could not read line.");
}

fn to_u8(string: &str) -> u8 {
    string.trim().parse().expect("Unable to parse to integer")
}

fn round(grade: u8) -> u8 {
    let difference = grade % 5;
    if difference == 0 || (5 - difference) >=  3 || grade < 38 {
        grade
    } else {
        grade - difference + 5
    }
}
