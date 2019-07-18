use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let parsed_input = stdin_handle
        .lines()
        .next()
        .unwrap()?
        .split_whitespace()
        .map(str::parse::<u32>)
        .filter_map(std::result::Result::ok)
        .collect::<Vec<u32>>();

    let gold = parsed_input[0];
    let silver = parsed_input[1];
    let copper = parsed_input[2];
    let buying_power = gold * 3 + silver * 2 + copper;

    let best_victory_card = if buying_power >= 8 {
        Some("Province")
    } else if buying_power >= 5 {
        Some("Duchy")
    } else if buying_power >= 2 {
        Some("Estate")
    } else {
        None
    };

    let best_treasure_card = if buying_power >= 6 {
        "Gold"
    } else if buying_power >= 3 {
        "Silver"
    } else {
        "Copper"
    };

    if let Some(victory_card) = best_victory_card {
        write!(stdout_handle, "{} or ", victory_card)?;
    }
    writeln!(stdout_handle, "{}", best_treasure_card)?;
    Ok(())
}
