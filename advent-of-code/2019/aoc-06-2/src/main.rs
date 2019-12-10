use std::collections::HashMap;
use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

struct PathFinder {
    san_ancestors: Option<Vec<String>>,
    you_ancestors: Option<Vec<String>>,
}

impl PathFinder {
    fn new() -> Self {
        Self {
            san_ancestors: None,
            you_ancestors: None,
        }
    }
}

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin_handle = stdin.lock();
    let mut stdout_handle = stdout.lock();

    let tree = stdin_handle
        .lines()
        .filter_map(std::result::Result::ok)
        .fold(HashMap::new(), |mut map, s| {
            let mut parsed_input = s
                .split(')')
                .map(str::to_string)
                .take(2)
                .collect::<Vec<String>>();
            let orbiter = parsed_input.pop().unwrap();
            let orbitee = parsed_input.pop().unwrap();
            map.entry(orbitee).or_insert_with(Vec::new).push(orbiter);
            map
        });

    let mut pathfinder = PathFinder::new();
    dfs("COM".to_string(), &tree, Vec::new(), &mut pathfinder);
    if let Some(output) = number_of_orbital_transfers(&pathfinder) {
        writeln!(stdout_handle, "{}", output)?;
    }
    Ok(())
}

fn dfs(
    key: String,
    tree: &HashMap<String, Vec<String>>,
    ancestors: Vec<String>,
    pathfinder: &mut PathFinder,
) {
    if pathfinder.san_ancestors.is_some() && pathfinder.you_ancestors.is_some() {
        return;
    }
    if key == "SAN" {
        pathfinder.san_ancestors = Some(ancestors.clone())
    }
    if key == "YOU" {
        pathfinder.you_ancestors = Some(ancestors.clone())
    }

    if let Some(children) = tree.get(&key) {
        for child in children {
            let mut new_ancestors = ancestors.clone();
            new_ancestors.push(key.clone());
            dfs(child.to_string(), tree, new_ancestors, pathfinder)
        }
    }
}

fn number_of_orbital_transfers(pathfinder: &PathFinder) -> Option<usize> {
    let san_ancestors = pathfinder.san_ancestors.as_ref()?;
    let you_ancestors = pathfinder.you_ancestors.as_ref()?;

    let (smaller, bigger) = if san_ancestors.len() < you_ancestors.len() {
        (san_ancestors, you_ancestors)
    } else {
        (you_ancestors, san_ancestors)
    };

    let number_of_common_ancestors = (0..smaller.len())
        .filter(|&i| smaller[i] == bigger[i])
        .count();
    Some(smaller.len() + bigger.len() - 2 * number_of_common_ancestors)
}
