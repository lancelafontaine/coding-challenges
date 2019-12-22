use std::char;
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

    let image = pixels.chunks_exact(LAYER_WIDTH * LAYER_HEIGHT).fold(
        vec![2; LAYER_WIDTH * LAYER_HEIGHT],
        |mut image: Vec<Pixel>, layer| {
            for (i, &pixel) in layer.iter().enumerate() {
                if let Some(&existing_pixel) = image.get(i) {
                    if existing_pixel == 2 && pixel != 2 {
                        image[i] = pixel;
                    }
                } else {
                    image.insert(i, pixel);
                }
            }
            image
        },
    );
    for chunk in image.chunks_exact(LAYER_WIDTH) {
        let row = chunk
            .into_iter()
            .map(|pixel| char::from_digit(*pixel as u32, 10).unwrap())
            .map(|c| if c == '0' { ' ' } else { c })
            .collect::<String>();
        writeln!(stdout_handle, "{}", row)?;
    }
    Ok(())
}
