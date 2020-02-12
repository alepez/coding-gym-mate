fn main() {
    use std::io::{self, BufRead};

    io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .for_each(|line| {
            let rev: String = line.chars().rev().collect();
            println!("{}", rev);
        });
}

