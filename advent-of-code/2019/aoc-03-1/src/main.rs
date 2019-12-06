use std::cmp;
use std::convert::From;
use std::convert::TryInto;
use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Copy, Clone)]
struct Step {
    direction: Direction,
    distance: u64,
}

impl Step {
    fn new(direction: Direction, distance: u64) -> Self {
        Step {
            direction,
            distance,
        }
    }
}

#[derive(Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }

    fn manhattan_distance(&self, other: &Point) -> u64 {
        ((self.x - other.x).abs() + (self.y - other.y).abs())
            .try_into()
            .unwrap()
    }
}

struct StepLine {
    point: Point,
    step: Step,
}

impl StepLine {
    fn new(point: Point, step: Step) -> Self {
        StepLine { point, step }
    }
}

#[derive(PartialEq)]
enum Orientation {
    Vertical,
    Horizontal,
}

struct Line {
    orientation: Orientation,
    start: Point,
    end: Point,
}

impl From<StepLine> for Line {
    fn from(stepline: StepLine) -> Self {
        let (orientation, end_x, end_y) = match &stepline.step.direction {
            Direction::Up => (
                Orientation::Vertical,
                stepline.point.x,
                stepline.point.y + stepline.step.distance as i64,
            ),
            Direction::Right => (
                Orientation::Horizontal,
                stepline.point.x + stepline.step.distance as i64,
                stepline.point.y,
            ),
            Direction::Down => (
                Orientation::Vertical,
                stepline.point.x,
                stepline.point.y - stepline.step.distance as i64,
            ),
            Direction::Left => (
                Orientation::Horizontal,
                stepline.point.x - stepline.step.distance as i64,
                stepline.point.y,
            ),
        };
        Line::new(
            orientation,
            Point::new(stepline.point.x, stepline.point.y),
            Point::new(end_x, end_y),
        )
    }
}

impl Line {
    fn new(orientation: Orientation, start: Point, end: Point) -> Self {
        Line {
            orientation,
            start,
            end,
        }
    }

    fn get_intersection(&self, other: &Line) -> Option<Point> {
        if (self.start.x == 0 && other.start.x == 0 && self.start.y == 0 && other.start.y == 0)
            || self.orientation == other.orientation
        {
            return None;
        }

        let (horizontal, vertical) = if self.orientation == Orientation::Horizontal {
            (self, other)
        } else {
            (other, self)
        };

        if horizontal.start.y <= cmp::max(vertical.start.y, vertical.end.y)
            && horizontal.start.y >= cmp::min(vertical.start.y, vertical.end.y)
            && vertical.start.x <= cmp::max(horizontal.start.x, horizontal.end.x)
            && vertical.start.x >= cmp::min(horizontal.start.x, horizontal.end.x)
        {
            return Some(Point::new(vertical.start.x, horizontal.start.y));
        }
        None
    }
}

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let steps_of_wires = get_steps_of_wires(stdin_handle)?;
    let lines_of_wires = get_lines_of_wire(steps_of_wires);

    let mut intersections = Vec::new();
    for first_wire_line in &lines_of_wires[0] {
        for second_wire_line in &lines_of_wires[1] {
            if let Some(intersection) = first_wire_line.get_intersection(second_wire_line) {
                intersections.push(intersection)
            }
        }
    }

    let origin = Point::new(0, 0);
    let closest_intersection = intersections
        .iter()
        .map(|intersection| intersection.manhattan_distance(&origin))
        .min()
        .unwrap();

    writeln!(stdout_handle, "{}", closest_intersection)?;
    Ok(())
}

fn get_steps_of_wires(stdin_handle: std::io::StdinLock) -> Result<Vec<Vec<Step>>> {
    let mut inputs = Vec::new();
    let lines = stdin_handle.lines().map(|s| s.unwrap());
    for line in lines {
        let mut input = Vec::new();
        for step in line.split(',') {
            let direction = match &step[0..1] {
                "R" => Direction::Right,
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                _ => unimplemented!(),
            };
            let distance = &step[1..].parse::<u64>()?;
            let step = Step::new(direction, *distance);
            input.push(step);
        }
        inputs.push(input);
    }
    Ok(inputs)
}

fn get_lines_of_wire(steps_of_wires: Vec<Vec<Step>>) -> Vec<Vec<Line>> {
    let mut lines_of_wires = vec![vec![], vec![]];

    for wire_number in 0..=1 {
        let mut point = Point::new(0, 0);
        let wire_steps = &steps_of_wires[wire_number];
        let wire_lines = &mut lines_of_wires[wire_number];

        for &step in wire_steps {
            let line: Line = StepLine::new(point.clone(), step).into();
            point.x = line.end.x;
            point.y = line.end.y;
            wire_lines.push(line)
        }
    }
    lines_of_wires
}
