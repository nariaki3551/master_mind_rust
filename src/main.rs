use clap::Parser;
use itertools::Itertools;
use std::io::Write;

type Code = Vec<usize>;
type CodeSet = Vec<Code>;
type Hint = (usize, usize);

#[derive(Debug)]
struct Context {
    color_num: usize,
    pin_num: usize,
    duplicate: bool,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(help = "number of colors")]
    color_num: usize,
    #[arg(help = "number of pins")]
    pin_num: usize,
    #[arg(short, long, help = "codes do not have duplicate colors")]
    non_duplicate: bool,
}

fn main() {
    // parse command line arguments
    let cli = Args::parse();
    let context = Context {
        color_num: cli.color_num,
        pin_num: cli.pin_num,
        duplicate: !cli.non_duplicate,
    };
    println!("{:?}", context);
    println!(
        "You decide the secret code with colors from 0 to {}, and I will guess it. Let's start!",
        context.color_num - 1
    );

    // enumerate all codes
    let all_codes = get_all_codes(&context);

    // main process
    let mut candidates = all_codes.clone();
    while candidates.len() > 1 {
        let guess = policy(&candidates);
        let hint = trial(&guess);
        candidates.retain(|x| calc_hint(x, &guess, &context) == hint); // update candidates
    }
    assert_eq!(candidates.len(), 1);

    // post process
    let secret = &candidates[0];
    println!("Your secret is {:?}", secret);
}

// enumerate all codes according to context
fn get_all_codes(context: &Context) -> CodeSet {
    match context.duplicate {
        true => (0..context.pin_num)
            .map(|_| 0..context.color_num)
            .multi_cartesian_product()
            .collect(),
        false => (0..context.color_num)
            .permutations(context.pin_num)
            .collect(),
    }
}

// select guess from guess set
fn policy(guess_set: &CodeSet) -> Code {
    guess_set[0].clone()
}

// get a hint according to the guess from user input
fn trial(guess: &Code) -> Hint {
    println!("Guess: {:?}", guess);
    print!("input hit and blow (format: hit blow)> ");
    std::io::stdout().flush().unwrap();
    let mut input_string = String::new();
    std::io::stdin().read_line(&mut input_string).ok();
    let parts: Vec<&str> = input_string.split_whitespace().collect();
    let hit = parts[0].parse().expect("Hit should be an integer number.");
    let blow = parts[1].parse().expect("Blow should be an integer number.");
    println!("Hint: {:?}", (hit, blow));
    (hit, blow)
}

// calculate hint from two codes
fn calc_hint(code: &Code, guess: &Code, context: &Context) -> Hint {
    let mut hit = 0;
    let mut blow = 0;
    let mut code_color_counts = vec![0; context.color_num]; // array of the number of i-th colors which code has
    let mut guess_color_counts = vec![0; context.color_num]; // array of the number of i-th colors which guess has
    for i in 0..context.pin_num {
        if code[i] == guess[i] {
            hit += 1;
        } else {
            code_color_counts[code[i]] += 1;
            guess_color_counts[guess[i]] += 1;
        }
    }
    for i in 0..context.color_num{
        blow += std::cmp::min(code_color_counts[i], guess_color_counts[i]);
    }
    (hit, blow)
}
