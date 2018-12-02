use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn main() -> Result<()> {
    let mut result = String::new();
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    'outer: for word1 in input.lines().clone() {
        for word2 in input.lines() {
            if word1.len() == word2.len()
                && levenshtein_substitution_distance(&word1[..], &word2[..]) == 1
            {
                for (index, character) in word1.chars().enumerate() {
                    if character == word2.chars().nth(index).unwrap() {
                        result.push(character)
                    }
                }
                break 'outer;
            }
        }
    }
    writeln!(io::stdout(), "{}", result)?;
    Ok(())
}

fn levenshtein_substitution_distance(a: &str, b: &str) -> usize {
    let len_a = a.chars().count();
    let len_b = b.chars().count();

    let row: Vec<usize> = vec![0; len_b + 1];
    let mut matrix: Vec<Vec<usize>> = vec![row; len_a + 1];

    for i in 0..len_a {
        matrix[i + 1][0] = matrix[i][0] + 1;
    }

    for i in 0..len_b {
        matrix[0][i + 1] = matrix[0][i] + 1;
    }

    for (i, ca) in a.chars().enumerate() {
        for (j, cb) in b.chars().enumerate() {
            let alternatives = [matrix[i][j] + if ca == cb { 0 } else { 1 }];
            matrix[i + 1][j + 1] = *alternatives.iter().min().unwrap();
        }
    }
    matrix[len_a][len_b]
}
