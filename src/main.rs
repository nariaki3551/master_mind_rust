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

    match context.mode {
        def::Mode::Mktree => main_mktree(&context),
        def::Mode::Guess => {
            println!(
                "You decide the secret code with colors from 0 to {}, and I will guess it. Let's start!",
                context.color_num - 1
            );
            main_guess(&context);
        }
    }
}

fn main_guess(context: &def::Context) {
    // generate all codes from context
    let all_codes = utils::get_all_codes(context);
    debug!("finish get_all_codes");

    // main process
    let mut candidates = all_codes.clone();
    while candidates.len() > 1 {
        debug!("Number of candidates: {}", candidates.len());
        let guess = match context.policy {
            def::Policy::Firstpick => policy::first_pick(&candidates),
            def::Policy::Minmax => policy::minmax(&candidates, &all_codes, context),
        };
        let hint = utils::trial(&guess);
        candidates.retain(|x| utils::calc_hint(x, &guess, context) == hint); // update candidates
    }
    assert_eq!(candidates.len(), 1);

    // post process
    let secret = &candidates[0];
    println!("Your secret is {:?}", secret);
}

fn main_mktree(context: &def::Context) {
    if !context.benchmark {
        print_notation();
    }

    let start = std::time::Instant::now();

    // generate all codes from context
    let all_codes = utils::get_all_codes(context);
    debug!("finish get_all_codes");
    mktree_step(&all_codes, 1, &all_codes, context);

    println!("Elapsed Time: {:.4} sec", start.elapsed().as_secs_f32());
}

fn mktree_step(
    candidates: &def::CodeSet,
    depth: usize,
    all_codes: &def::CodeSet,
    context: &def::Context,
) {
    let guess = match context.policy {
        def::Policy::Firstpick => policy::first_pick(candidates),
        def::Policy::Minmax => policy::minmax(candidates, all_codes, context),
    };
    if !context.benchmark {
        print_trial(depth, &guess);
    }
    let map = utils::calc_hint_based_candidates_map(candidates, &guess, context);
    for (hint, candidates) in &map {
        if !context.benchmark {
            print_hint(depth, hint, candidates.len());
        }
        if candidates.len() == 1 {
            let turn = depth + if hint.0 == context.pin_num { 0 } else { 1 };
            if !context.benchmark {
                print_secret(depth, &map[hint][0], turn);
            }
        } else {
            mktree_step(&map[hint], depth + 1, all_codes, context);
        }
    }
}

fn print_notation() {
    println!(
        "Notation:\n#c=: number of candidates is\ng= : guess is\n#t= : number of total turns is\n"
    );
}
fn print_trial(depth: usize, guess: &def::Code) {
    println!(
        "{}Trial {}: g={:?}",
        " ".repeat(4 * depth),
        depth + 1,
        guess
    );
}

fn print_hint(depth: usize, hint: &def::Hint, candidate_num: usize) {
    println!(
        "{}-> hint = {:?}, #c={}",
        " ".repeat(4 * (depth + 1)),
        hint,
        candidate_num
    );
}

fn print_secret(depth: usize, secret: &def::Code, turn: usize) {
    println!(
        "{}secret is {:?} #t={}",
        " ".repeat(4 * (depth + 2)),
        secret,
        turn
    );
}
