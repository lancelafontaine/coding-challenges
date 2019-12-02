use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let fuel_total: u64 = stdin_handle
        .lines()
        .filter_map(std::result::Result::ok)
        .map(|s| s.parse::<u64>())
        .filter_map(std::result::Result::ok)
        .map(fuel_by_mass)
        .sum();

    writeln!(stdout_handle, "{}", fuel_total)?;
    Ok(())
}

fn fuel_by_mass(module_mass: u64) -> u64 {
    let fuel_mass = module_mass / 3;
    let fuel_mass = if fuel_mass > 2 { fuel_mass - 2 } else { 0 };

    if fuel_mass > 0 {
        fuel_mass + fuel_by_mass(fuel_mass)
    } else {
        fuel_mass
    }
}
