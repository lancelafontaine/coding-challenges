use std::io::{self, BufRead};
use std::cmp;
extern crate time;

fn main() {
    let mut number_of_rows = String::new();
    read_line(&mut number_of_rows);
    let number_of_rows = to_u32(&number_of_rows.as_str());

    let mut pyramid: Vec<Vec<u32>> = Vec::new();

    for _ in 0..number_of_rows {
        let mut row = String::new();
        read_line(&mut row);
        let row_elements: Vec<u32> = row.trim().split(" ").map(|string| to_u32(string)).collect();
        pyramid.push(row_elements);
    }

    let start_time = time::precise_time_s();

    println!("{}", find_shortest_path(&mut pyramid));

    let end_time = time::precise_time_s();

    println!("{}", end_time - start_time);
}

fn read_line(mut string: &mut String) {
    let stdin = io::stdin();
    stdin.lock().read_line(&mut string).expect("Couldn't read line from stdin into string");
}

fn to_u32(string: &str) -> u32 {
    string.trim().parse().expect("Could not parse string into integer")
}

fn find_shortest_path(pyramid: &mut Vec<Vec<u32>>) -> u32 {
    for i in (0..pyramid.len()).rev() {
        for j in 0..(pyramid[i].len()-1) {
            pyramid[i-1][j] = pyramid[i-1][j] + cmp::min(pyramid[i][j], pyramid[i][j+1]);
        }
    }
    pyramid[0][0]
}
