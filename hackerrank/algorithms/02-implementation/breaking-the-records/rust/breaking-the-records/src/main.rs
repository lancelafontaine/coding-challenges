use std::io::{self, BufRead};

fn main() {
    let mut highest_score: Option<usize> = None;
    let mut lowest_score: Option<usize> = None;
    let mut num_highest_scores = 0;
    let mut num_lowest_scores = 0;

    let mut _line = String::new();
    read_line(&mut _line);

    let mut point_line = String::new();
    read_line(&mut point_line);

    for score in point_line.split(" ") {
        let int_score: usize = score.parse().unwrap();

        if let None = highest_score {
            highest_score = Some(int_score);
            lowest_score = Some(int_score);
            continue;
        }

        if int_score > highest_score.unwrap() {
            highest_score = Some(int_score);
            num_highest_scores += 1;
        }

        if int_score < lowest_score.unwrap() {
            lowest_score = Some(int_score);
            num_lowest_scores += 1;
        }
    }
    println!("{} {}", num_highest_scores, num_lowest_scores);
}

fn read_line(mut buffer: &mut String) {
    let stdin = io::stdin();
    stdin.lock().read_line(&mut buffer).expect("Could not read line");
}

