use std::io::{self, BufRead};

fn main() {
    let mut wanted_value = String::new();
    read_line(&mut wanted_value);
    let wanted_value = to_u32(&wanted_value);

    let mut _discard = String::new();
    read_line(&mut _discard);

    let mut array_string = String::new();
    read_line(&mut array_string);
    let vec: Vec<u32> = array_string
        .trim()
        .split(' ')
        .map(|elem| to_u32(&elem.to_string()))
        .collect();

    if let Ok(index) = vec.binary_search(&wanted_value) {
        println!("{}", index);
    } else {
        println!("Value not found in array.");
    }
}

fn read_line(mut string: &mut String) {
    let stdin = io::stdin();
    stdin.lock().read_line(&mut string).expect("Could not read input. Exiting.");
}

fn to_u32(string: &String) -> u32 {
    string.trim().parse().expect("Could not parse string into integer. Exiting.")
}
