mod def;
mod policy;
mod utils;

use clap::Parser;
use log::debug;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(default_value_t = 6, help = "number of colors")]
    color_num: usize,
    #[arg(default_value_t = 4, help = "number of pins")]
    pin_num: usize,
    #[arg(short, long, help = "codes do not have duplicate colors")]
    non_duplicate: bool,
    #[arg(long, value_enum, default_value_t = def::Policy::Minmax, help = "policy")]
    policy: def::Policy,
    #[arg(long, value_enum, default_value_t = def::Mode::Guess, help = "mode")]
    mode: def::Mode,
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
    };

    match context.mode {
        def::Mode::Guess => main_guess(context),
        def::Mode::Mktree => main_mktree(context, true),
        def::Mode::Benchmark => main_benchmark(context),
    }
}

fn main_guess(context: def::Context) {
    println!("{:?}", context);
    println!(
        "You decide the secret code with colors from 0 to {}, and I will guess it. Let's start!",
        context.color_num - 1
    );

    // generate all codes from context
    let all_codes = utils::get_all_codes(&context);
    debug!("finish get_all_codes: size {}", all_codes.len());

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
    println!("Your secret is {:?}", &candidates[0]);
}

fn main_mktree(context: def::Context, verbose: bool) {
    if verbose {
        println!("{:?}", context);
        print_notation();
    }

    // generate all codes from context
    let all_codes = utils::get_all_codes(&context);
    debug!("finish get_all_codes: size {}", all_codes.len());
    mktree_step(&all_codes, 1, &all_codes, &context, verbose);
}

fn mktree_step(
    candidates: &def::CodeSet,
    depth: usize,
    all_codes: &def::CodeSet,
    context: &def::Context,
    verbose: bool,
) {
    let guess = match context.policy {
        def::Policy::Firstpick => policy::first_pick(candidates),
        def::Policy::Minmax => policy::minmax(candidates, all_codes, context),
    };
    if verbose {
        print_trial(depth, &guess);
    }
    let map = utils::calc_hint_based_candidates_map(candidates, &guess, context);
    for (hint, candidates) in &map {
        if verbose {
            print_hint(depth, hint, candidates.len());
        }
        if candidates.len() == 1 {
            let turn = depth + if hint.0 == context.pin_num { 0 } else { 1 };
            if verbose {
                print_secret(depth, &map[hint][0], turn);
            }
        } else {
            mktree_step(&map[hint], depth + 1, all_codes, context, verbose);
        }
    }
}

fn main_benchmark(context: def::Context) {
    let context_set = [(2, 2), (4, 2), (4, 4), (6, 2), (6, 4)];
    // let context_set = [(2, 2), (4, 2), (4, 4), (6, 2)];
    println!("color_num,pin_num,duplicate,policy,run,elapsed");
    for (color_num, pin_num) in context_set {
        for i in 0..10 {
            let exp_context = def::Context {
                color_num,
                pin_num,
                duplicate: context.duplicate,
                policy: context.policy.clone(),
                mode: context.mode.clone(),
            };
            let start = std::time::Instant::now();
            main_mktree(exp_context, false);
            let elapsed = start.elapsed().as_secs_f32();
            println!(
                "{},{},{},{:?},{},{}",
                color_num, pin_num, context.duplicate, context.policy, i, elapsed
            );
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
