use farey::fraction::Fraction;
use clap::{Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    n: u8
}


pub fn main() {
    let cli = Cli::parse();

    let start: Vec<Fraction<u64>> = vec![(0, 1).into(), (1, 1).into()];

    let mut current = start;
    for _ in 1..cli.n {
        current = mediants(current);
    }

    for item in current {
        print!("{} ", item);
    }
}

fn mediants(original: Vec<Fraction<u64>>) -> Vec<Fraction<u64>> {
    let mut result = Vec::new();

    for index in 0..(original.len() - 1) {
        let first = original[index];
        let second = original[index + 1];

        result.push(first);
        result.push(first.mediant(second));
    }
    result.push(
        original
            .last()
            .expect("original to have a last element")
            .clone(),
    );

    result
}
