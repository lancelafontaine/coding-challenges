use std::collections::HashMap;
use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

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

    let orbit_total = orbit_total(tree, "COM".to_string());
    writeln!(stdout_handle, "{}", orbit_total)?;
    Ok(())
}

fn orbit_total(mut tree: HashMap<String, Vec<String>>, root_key: String) -> usize {
    let mut root_keys = vec![root_key];
    let mut tree_level = Vec::new();
    let mut tree_depth = 1;
    let mut orbit_total = 0;

    loop {
        while !root_keys.is_empty() {
            let key = root_keys.pop().unwrap();
            if let Some(children) = tree.get_mut(&key) {
                tree_level.append(children)
            }
        }

        orbit_total += tree_level.len() * tree_depth;

        if root_keys.is_empty() && tree_level.is_empty() {
            return orbit_total;
        }

        root_keys.append(&mut tree_level);
        tree_depth += 1;
    }
}
