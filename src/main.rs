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
    #[arg(long, value_enum, default_value_t = def::Mode::Guess, help = "mode")]
    mode: def::Mode,
    #[arg(short, long, help = "benchmark mode")]
    benchmark: bool,
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
        mode: cli.mode,
        benchmark: cli.benchmark,
    };
    println!("{:?}", context);
    println!(
        "You decide the secret code with colors from 0 to {}, and I will guess it. Let's start!",
        context.color_num - 1
    );
    print_notation(&context);
    let start = std::time::Instant::now();

    // generate all codes from context
    let all_codes = utils::get_all_codes(&context);
    debug!("finish get_all_codes");

    step(&all_codes, 0, &all_codes, &context);
    println!("Elapsed Time: {:.4} sec", start.elapsed().as_secs_f32());
}

fn step(candidates: &def::CodeSet, depth: usize, all_codes: &def::CodeSet, context: &def::Context) {
    let guess = match context.policy {
        def::Policy::Firstpick => policy::first_pick(candidates),
        def::Policy::Minmax => policy::minmax(candidates, all_codes, context),
    };
    let map = utils::calc_hint_based_candidates_map(candidates, &guess, context);
    match context.mode {
        def::Mode::Guess => {
            debug!("Number of candidates: {}", candidates.len());
            let hint = utils::trial(&guess);
            assert!(!map[&hint].is_empty(), "User input hints must be wrong.");
            if map[&hint].len() == 1 {
                println!("Your secret is {:?}", map[&hint][0]);
            } else {
                step(&map[&hint], depth + 1, all_codes, context);
            }
        }
        def::Mode::Mktree => {
            print_trial(depth, &guess, context);
            for (hint, candidates) in &map {
                print_hint(depth, hint, candidates.len(), context);
                assert_ne!(map[&hint].len(), 0);
                if map[&hint].len() == 1 {
                    let turn = depth + if hint.0 == context.pin_num { 1 } else { 0 };
                    print_secret(depth, &map[&hint][0], turn, context);
                } else {
                    step(&map[&hint], depth + 1, all_codes, context);
                }
            }
        }
    }
}

fn print_notation(context: &def::Context) {
    match context.mode {
        def::Mode::Guess => {}
        def::Mode::Mktree => {
            if !context.benchmark {
                println!(
                    "Notation:\n#c=: number of candidates is\ng= : guess is\n#t= : number of total turns is\n"
                );
            }
        }
    }
}

fn print_trial(depth: usize, guess: &def::Code, context: &def::Context) {
    if !context.benchmark {
        println!(
            "{}Trial {}: g={:?}",
            " ".repeat(4 * depth),
            depth + 1,
            guess
        );
    }
}

fn print_hint(depth: usize, hint: &def::Hint, candidate_num: usize, context: &def::Context) {
    if !context.benchmark {
        println!(
            "{}-> hint = {:?}, #c={}",
            " ".repeat(4 * (depth + 1)),
            hint,
            candidate_num
        );
    }
}

fn print_secret(depth: usize, secret: &def::Code, turn: usize, context: &def::Context) {
    if !context.benchmark {
        println!(
            "{}secret is {:?} #t={}",
            " ".repeat(4 * (depth + 2)),
            secret,
            turn
        );
    }
}
