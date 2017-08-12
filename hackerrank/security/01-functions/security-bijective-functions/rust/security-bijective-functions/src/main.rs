use std::io::{self, BufRead};
use std::collections::HashSet;

fn main() {
    let mut _discard = String::new();
    read_line(&mut _discard);

    let mut codomain = String::new();
    read_line(&mut codomain);

    // It's injective if all n integers are unique
    // It's surjective if we assume that the codomain consists only of set of values printed
    let mut set = HashSet::new();
    for y in codomain.split(' ') {
        if !set.contains(y) {
            set.insert(y);
        } else {
            return println!("NO");
        }
    }
    println!("YES");
}

fn read_line(mut string: &mut String) {
    let stdin = io::stdin();
    stdin.lock().read_line(&mut string).expect("Could not read line.");
}
