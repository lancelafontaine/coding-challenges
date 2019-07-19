use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let mut lines = stdin_handle.lines();
    let unparsed_fire_safety_limit = lines.next().unwrap()?;
    let fire_safety_limit = unparsed_fire_safety_limit
        .split_whitespace()
        .next()
        .unwrap()
        .parse::<u32>()?;

    let mut number_of_denials = 0;
    let mut current_number_on_terrace = 0;
    for line in lines {
        let line = line?;
        let mut line_iterator = line.split_whitespace();
        let action = line_iterator.next().unwrap();
        let group_size = line_iterator.next().unwrap().parse::<u32>()?;

        match action {
            "enter" => {
                if current_number_on_terrace + group_size > fire_safety_limit {
                    number_of_denials += 1
                } else {
                    current_number_on_terrace += group_size
                }
            }
            "leave" => current_number_on_terrace -= group_size,
            _ => panic!("Unhandled action type"),
        }
    }

    writeln!(stdout_handle, "{}", number_of_denials)?;
    Ok(())
}
