use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let mut lines = stdin_handle.lines();

    let mut set_number = 1;
    while let Some(input_number) = lines.next().unwrap().ok() {
        let number_of_names = input_number.parse::<usize>()?;
        if number_of_names == 0 {
            break;
        }

        let mut output_array: Vec<Option<String>> = vec![None; number_of_names];

        for i in 0..(number_of_names / 2) + 1 {
            let first_index = i;
            let second_index = number_of_names - i - 1;

            if output_array[first_index].is_none() {
                output_array[first_index] = Some(lines.next().unwrap()?);
            }
            if output_array[second_index].is_none() {
                output_array[second_index] = Some(lines.next().unwrap()?);
            }
        }

        writeln!(stdout_handle, "SET {}", set_number)?;
        set_number += 1;
        for name_option in output_array {
            writeln!(stdout_handle, "{}", name_option.unwrap())?;
        }
    }

    Ok(())
}
