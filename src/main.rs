mod def;
mod policy;
mod utils;

use clap::Parser;

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
    let context = def::Context {
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
    let all_codes = utils::get_all_codes(&context);

    // main process
    let mut candidates = all_codes.clone();
    while candidates.len() > 1 {
        let guess = policy::first_pick(&candidates);
        let hint = utils::trial(&guess);
        candidates.retain(|x| utils::calc_hint(x, &guess, &context) == hint); // update candidates
    }
    assert_eq!(candidates.len(), 1);

    // post process
    let secret = &candidates[0];
    println!("Your secret is {:?}", secret);
}
