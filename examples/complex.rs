use clap::Parser;
use farey::{fraction::Fraction, gaussian::Gaussian};
use num::Zero;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    n: u8,
}

pub fn main() {
    let cli = Cli::parse();

    let one: Gaussian<i64> = (1, 0).into();
    let complex: Gaussian<i64> = (0, 1).into();

    let begin: Fraction<Gaussian<i64>> = Fraction::new(one, one);
    let end: Fraction<Gaussian<i64>> = Fraction::new(complex, one);

    let start: Vec<Fraction<Gaussian<i64>>> = vec![begin, end];

    let mut current = start;
    for _ in 1..cli.n {
        current = mediants(current);
    }

    for item in current {
        print!("{} ", item);
    }
}

fn mediants(original: Vec<Fraction<Gaussian<i64>>>) -> Vec<Fraction<Gaussian<i64>>> {
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
