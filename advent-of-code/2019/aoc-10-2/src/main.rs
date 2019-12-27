use std::collections::{HashMap, HashSet};
use std::f64::consts::{FRAC_PI_2, PI};
use std::io::{self, BufRead, StdinLock, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const TWO_PI: f64 = PI + PI;
const STARTING_DIRECTION: f64 = -FRAC_PI_2;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct AsteroidCount {
    asteroid: Point,
    count: usize,
}

impl AsteroidCount {
    fn new(asteroid: Point, count: usize) -> Self {
        Self { asteroid, count }
    }
}

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let asteroids = parse_asteroid_locations(stdin_handle)?;
    let (_, mut asteroids_by_relative_angle) = optimal_asteroid(&asteroids).unwrap();
    let two_hundredth_asteroid_vaporized =
        get_vaporized_asteroid(200, &mut asteroids_by_relative_angle);
    let output = two_hundredth_asteroid_vaporized.x * 100 + two_hundredth_asteroid_vaporized.y;
    writeln!(stdout_handle, "{}", output)?;
    Ok(())
}

fn get_vaporized_asteroid(
    n: u64,
    asteroids_by_relative_angle: &mut HashMap<String, HashSet<Point>>,
) -> Point {
    let mut angles = asteroids_by_relative_angle
        .keys()
        .filter_map(|s| s.parse::<f64>().ok())
        .collect::<Vec<f64>>();
    angles.sort_unstable_by(|&angle1, &angle2| {
        let mut angle1 = angle1;
        while angle1 < STARTING_DIRECTION {
            angle1 += TWO_PI;
        }
        let mut angle2 = angle2;
        while angle2 < STARTING_DIRECTION {
            angle2 += TWO_PI;
        }
        angle1.partial_cmp(&angle2).unwrap()
    });

    let mut number_of_vaporized_asteroids = 0;
    'outer: loop {
        for angle in &angles {
            let some_asteroids = asteroids_by_relative_angle.get_mut(&to_hashable_f64(*angle));
            if let Some(asteroids) = some_asteroids {
                if asteroids.is_empty() {
                    continue;
                }
                let closest_asteroid = *asteroids
                    .iter()
                    .min_by_key(|asteroid| asteroid.x + -asteroid.y)
                    .unwrap();
                asteroids.remove(&closest_asteroid);
                number_of_vaporized_asteroids += 1;
                if number_of_vaporized_asteroids == n {
                    break 'outer closest_asteroid;
                }
            }
        }
    }
}

fn parse_asteroid_locations(stdin_handle: StdinLock) -> Result<Vec<Point>> {
    let mut asteroids = Vec::new();
    for (y, row) in stdin_handle.lines().enumerate() {
        for (x, c) in row?.chars().enumerate() {
            if c == '#' {
                asteroids.push(Point::new(x as i64, y as i64));
            }
        }
    }
    Ok(asteroids)
}

fn optimal_asteroid(
    asteroids: &[Point],
) -> Option<(AsteroidCount, HashMap<String, HashSet<Point>>)> {
    let mut best_candidate: Option<(AsteroidCount, HashMap<String, HashSet<Point>>)> = None;

    for candidate_asteroid in asteroids {
        let asteroids_by_relative_angle =
            get_asteroids_by_relative_angle(candidate_asteroid, asteroids);
        let asteroid_count =
            AsteroidCount::new(*candidate_asteroid, asteroids_by_relative_angle.len());
        if best_candidate.is_none()
            || asteroid_count.count > best_candidate.as_ref().unwrap().0.count
        {
            best_candidate = Some((asteroid_count, asteroids_by_relative_angle));
        }
    }
    best_candidate
}

fn get_asteroids_by_relative_angle(
    asteroid: &Point,
    asteroids: &[Point],
) -> HashMap<String, HashSet<Point>> {
    let mut asteroids_by_relative_angle = HashMap::new();
    for comparator_asteroid in asteroids {
        if let Some(angle) = angle_between(comparator_asteroid, asteroid) {
            asteroids_by_relative_angle
                .entry(to_hashable_f64(angle))
                .or_insert_with(HashSet::new)
                .insert(*comparator_asteroid);
        }
    }
    asteroids_by_relative_angle
}

fn angle_between(asteroid1: &Point, asteroid2: &Point) -> Option<f64> {
    let x = asteroid1.x - asteroid2.x;
    let y = asteroid1.y - asteroid2.y;
    if x == 0 && y == 0 {
        return None;
    }
    Some((y as f64).atan2(x as f64))
}

fn to_hashable_f64(f: f64) -> String {
    format!("{:.20}", f)
}
