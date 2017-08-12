use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let mut _discard = String::new();
    read_line(&mut _discard);

    let mut birds = String::new();
    read_line(&mut birds);

    let mut bird_frequencies = HashMap::new();
    for bird in birds.trim().split(' ') {
        let frequency = bird_frequencies.entry(to_u8(bird.trim())).or_insert(0);
        *frequency += 1;
    }

    let mut highest_frequency_value: u32 = 0;
    let mut highest_frequency_type: u8 = u8::max_value();

    for (bird_type, frequency) in bird_frequencies.iter() {
        if (frequency > &highest_frequency_value) ||
           (frequency == &highest_frequency_value && bird_type < &highest_frequency_type) {
            highest_frequency_value = frequency.clone();
            highest_frequency_type = bird_type.clone();
        }
    }
    println!("{}", highest_frequency_type);
}

fn read_line(mut string: &mut String) {
    let stdin = io::stdin();
    stdin.lock().read_line(&mut string).expect("Could not read line.");
}

fn to_u8(string: &str) -> u8 {
    string.trim().parse().expect(&format!("Could not parse {} into integer", string))
}
