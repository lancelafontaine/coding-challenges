use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    let mut _discard = String::new();
    stdin.lock().read_line(&mut _discard).expect("Could not read line");

    let mut candle_heights = String::new();
    stdin.lock().read_line(&mut candle_heights).expect("Could not read line");

    let mut max_candle_height = 0;
    let mut max_candle_number = 0;

    let candle_heights = candle_heights.split(' ');
    for height in candle_heights {
        let height: u32 = height
            .trim()
            .parse()
            .expect(&format!("{} {}", height.trim(), "is not a parseable integer"));

        if height > max_candle_height {
            max_candle_height = height;
            max_candle_number = 1;
        } else if height == max_candle_height {
            max_candle_number += 1;
        }
    }
    println!("{}", max_candle_number)
}
