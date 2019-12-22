use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
type Pixel = u8;

const LAYER_WIDTH: usize = 25;
const LAYER_HEIGHT: usize = 6;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let pixels = stdin_handle
        .lines()
        .next()
        .unwrap()?
        .chars()
        .map(|c| c.to_digit(10).unwrap() as Pixel)
        .collect::<Vec<Pixel>>();

    let output = pixels
        .chunks_exact(LAYER_WIDTH * LAYER_HEIGHT)
        .min_by_key(|chunk| chunk.iter().filter(|&&pixel| pixel == 0).count())
        .unwrap()
        .iter()
        .fold([0, 0], |mut counts, &pixel| {
            if pixel == 1 {
                counts[0] += 1;
            }
            if pixel == 2 {
                counts[1] += 1;
            }
            counts
        })
        .iter()
        .product::<u64>();

    writeln!(stdout_handle, "{}", output)?;
    Ok(())
}
