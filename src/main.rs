mod def;
mod policy;
mod utils;

use clap::Parser;
use log::debug;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(help = "number of colors")]
    color_num: usize,
    #[arg(help = "number of pins")]
    pin_num: usize,
    #[arg(short, long, help = "codes do not have duplicate colors")]
    non_duplicate: bool,
    #[arg(long, value_enum, default_value_t = def::Policy::Minmax, help = "policy")]
    policy: def::Policy,
}

fn main() {
    // init logger
    env_logger::init();

    // parse command line arguments
    let cli = Args::parse();
    let context = def::Context {
        color_num: cli.color_num,
        pin_num: cli.pin_num,
        duplicate: !cli.non_duplicate,
        policy: cli.policy,
    };
    println!("{:?}", context);
    println!(
        "You decide the secret code with colors from 0 to {}, and I will guess it. Let's start!",
        context.color_num - 1
    );

    // enumerate all codes
    let all_codes = utils::get_all_codes(&context);
    debug!("finish get_all_codes");

    // main process
    let mut candidates = all_codes.clone();
    while candidates.len() > 1 {
        debug!("Number of candidates: {}", candidates.len());
        let guess = match context.policy {
            def::Policy::Firstpick => policy::first_pick(&candidates),
            def::Policy::Minmax => policy::minmax(&candidates, &all_codes, &context),
        };
        let hint = utils::trial(&guess);
        candidates.retain(|x| utils::calc_hint(x, &guess, &context) == hint); // update candidates
    }
    assert_eq!(candidates.len(), 1);

    // post process
    let secret = &candidates[0];
    println!("Your secret is {:?}", secret);
}
