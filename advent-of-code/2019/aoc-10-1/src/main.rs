use std::collections::HashSet;
use std::io::{self, BufRead, StdinLock, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Eq, PartialEq, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

fn asteroid_locations(stdin_handle: StdinLock) -> Result<Vec<Point>> {
    let mut asteroids = Vec::new();
    for (y, row) in stdin_handle.lines().enumerate() {
        for (x, c) in row?.chars().enumerate() {
            if c == '#' {
                asteroids.push(Point::new(x as i64, -(y as i64)));
            }
        }
    }
    Ok(asteroids)
}

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let asteroids = asteroid_locations(stdin_handle)?;

    let mut best_candidate: Option<usize> = None;
    let mut distinct_angles = HashSet::new();

    for candidate_asteroid in &asteroids {
        for comparator_asteroid in &asteroids {
            let x = comparator_asteroid.x - candidate_asteroid.x;
            let y = comparator_asteroid.y - candidate_asteroid.y;
            if x == 0 && y == 0 {
                continue;
            }
            let angle = (y as f64).atan2(x as f64);
            distinct_angles.insert(format!("{:.20}", angle));
        }
        let asteroid_count = distinct_angles.len();
        if best_candidate.is_none() || asteroid_count > best_candidate.unwrap() {
            best_candidate = Some(asteroid_count)
        }
        distinct_angles.clear();
    }
    writeln!(stdout_handle, "{:?}", best_candidate.unwrap())?;
    Ok(())
}
