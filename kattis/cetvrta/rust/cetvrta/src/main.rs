use std::collections::HashMap;
use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let points = stdin_handle
        .lines()
        .filter_map(std::result::Result::ok)
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<u32>())
                .filter_map(std::result::Result::ok)
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut coordinate_counter = HashMap::new();
    let mut coordinate_finder = |acc, x| {
        coordinate_counter
            .entry(x)
            .and_modify(|e| *e += 1)
            .or_insert(1);
        if coordinate_counter.values().max() == Some(&2) && coordinate_counter.keys().len() == 2 {
            let mut output = None;
            for (&key, &mut value) in coordinate_counter.iter_mut() {
                if value == 1 {
                    output = Some(key);
                }
            }
            coordinate_counter.clear();
            return output;

        }
        acc
    };

    let x = points
        .iter()
        .map(|point| point[0])
        .fold(None, &mut coordinate_finder)
        .unwrap();
    let y = points
        .iter()
        .map(|point| point[1])
        .fold(None, &mut coordinate_finder)
        .unwrap();

    writeln!(stdout_handle, "{} {}", x, y)?;
    Ok(())
}
