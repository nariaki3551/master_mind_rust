use crate::def;

use itertools::Itertools;
use hashbrown::HashMap;
use std::io::Write;
use once_cell::sync::Lazy;
use std::sync::Mutex;


// enumerate all codes according to context
pub fn get_all_codes(context: &def::Context) -> def::CodeSet {
    match context.duplicate {
        true => (0..context.pin_num)
            .map(|_| 0..context.color_num)
            .multi_cartesian_product()
            .collect(),
        false => (0..context.color_num)
            .permutations(context.pin_num as usize)
            .collect(),
    }
}

// get a hint according to the guess from user input
pub fn trial(guess: &def::Code) -> def::Hint {
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

static CODE_COLOR_COUNTS: Lazy<Mutex<Vec<u8>>> = Lazy::new(|| Mutex::new(Vec::new()));
static GUESS_COLOR_COUNTS: Lazy<Mutex<Vec<u8>>> = Lazy::new(|| Mutex::new(Vec::new()));

// calculate hint from two codes
pub fn calc_hint(code: &def::Code, guess: &def::Code, context: &def::Context) -> def::Hint {
    let mut hit = 0;
    let mut blow = 0;

    let mut code_color_counts = CODE_COLOR_COUNTS.lock().unwrap();
    let mut guess_color_counts = GUESS_COLOR_COUNTS.lock().unwrap();
    code_color_counts.resize(context.color_num as usize, 0);
    guess_color_counts.resize(context.color_num as usize, 0);

    for (&c, &g) in code.iter().zip(guess.iter()) {
        if c == g {
            hit += 1;
        } else {
            code_color_counts[c as usize] += 1;
            guess_color_counts[g as usize] += 1;
        }
    }
    for i in 0..context.color_num as usize {
        blow += std::cmp::min(code_color_counts[i], guess_color_counts[i]);
    }

    // clear vector contents
    code_color_counts.clear();
    guess_color_counts.clear();

    (hit, blow)
}

// create map whose key is hint vaule is number of code set
pub fn calc_hint_based_candidate_num_map(
    candidates: &def::CodeSet,
    guess: &def::Code,
    context: &def::Context,
) -> HashMap<def::Hint, i16> {
    let mut map = HashMap::new();
    candidates.iter().for_each(|code| {
        *map.entry(calc_hint(code, guess, context) /* hint */)
            .or_insert(0) += 1;
    });
    map
}

// create map whose key is hint vaule is code set
pub fn calc_hint_based_candidates_map(
    candidates: def::CodeSet,
    guess: &def::Code,
    context: &def::Context,
) -> HashMap<def::Hint, def::CodeSet> {
    let mut map = HashMap::new();
    candidates.into_iter().for_each(|code| {
        map.entry(calc_hint(&code, guess, context) /* hint */)
            .or_insert_with(Vec::new)
            .push(code);
    });
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_hint_2color_2pin() {
        let context = def::Context {
            pin_num: 2,
            color_num: 2,
            duplicate: true,
            policy: def::Policy::Minmax,
            mode: def::Mode::Guess,
        };
        assert_eq!(
            calc_hint(&vec![0_u8, 0_u8], &vec![1_u8, 1_u8], &context),
            (0, 0)
        );
        assert_eq!(
            calc_hint(&vec![0_u8, 0_u8], &vec![0_u8, 1_u8], &context),
            (1, 0)
        );
        assert_eq!(
            calc_hint(&vec![0_u8, 0_u8], &vec![1_u8, 0_u8], &context),
            (1, 0)
        );
        assert_eq!(
            calc_hint(&vec![0_u8, 0_u8], &vec![0_u8, 0_u8], &context),
            (2, 0)
        );
        assert_eq!(
            calc_hint(&vec![0_u8, 1_u8], &vec![1_u8, 0_u8], &context),
            (0, 2)
        );
    }

    #[test]
    fn test_calc_hint_6color_3pin() {
        let context = def::Context {
            pin_num: 3,
            color_num: 6,
            duplicate: true,
            policy: def::Policy::Minmax,
            mode: def::Mode::Guess,
        };
        assert_eq!(
            calc_hint(&vec![0_u8, 0_u8, 4_u8], &vec![1_u8, 1_u8, 5_u8], &context),
            (0, 0)
        );
        assert_eq!(
            calc_hint(&vec![0_u8, 0_u8, 4_u8], &vec![0_u8, 1_u8, 5_u8], &context),
            (1, 0)
        );
        assert_eq!(
            calc_hint(&vec![0_u8, 0_u8, 4_u8], &vec![1_u8, 0_u8, 5_u8], &context),
            (1, 0)
        );
        assert_eq!(
            calc_hint(&vec![0_u8, 0_u8, 4_u8], &vec![0_u8, 0_u8, 5_u8], &context),
            (2, 0)
        );
        assert_eq!(
            calc_hint(&vec![0_u8, 1_u8, 4_u8], &vec![1_u8, 0_u8, 5_u8], &context),
            (0, 2)
        );
    }
}
