use crate::def;

use itertools::Itertools;
use std::collections::HashMap;
use std::io::Write;

// enumerate all codes according to context
pub fn get_all_codes(context: &def::Context) -> def::CodeSet {
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

// calculate hint from two codes
pub fn calc_hint(code: &def::Code, guess: &def::Code, context: &def::Context) -> def::Hint {
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
    for i in 0..context.color_num {
        blow += std::cmp::min(code_color_counts[i], guess_color_counts[i]);
    }
    (hit, blow)
}

// create map whose key is hint vaule is number of code set
pub fn calc_hint_based_candidate_num_map(
    candidates: &def::CodeSet,
    guess: &def::Code,
    context: &def::Context,
) -> HashMap<def::Hint, i16> {
    let mut map = HashMap::new();
    for code in candidates {
        let hint = calc_hint(code, guess, context);
        let candidate_num = map.entry(hint).or_insert(0);
        *candidate_num += 1;
    }
    map
}

// create map whose key is hint vaule is code set
pub fn calc_hint_based_candidates_map(
    candidates: &def::CodeSet,
    guess: &def::Code,
    context: &def::Context,
) -> HashMap<def::Hint, def::CodeSet> {
    let mut map = HashMap::new();
    for code in candidates {
        let hint = calc_hint(code, guess, context);
        map.entry(hint).or_insert_with(Vec::new).push(code.clone());
    }
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
        };
        assert_eq!(
            calc_hint(&vec![0_usize, 0_usize], &vec![1_usize, 1_usize], &context),
            (0, 0)
        );
        assert_eq!(
            calc_hint(&vec![0_usize, 0_usize], &vec![0_usize, 1_usize], &context),
            (1, 0)
        );
        assert_eq!(
            calc_hint(&vec![0_usize, 0_usize], &vec![1_usize, 0_usize], &context),
            (1, 0)
        );
        assert_eq!(
            calc_hint(&vec![0_usize, 0_usize], &vec![0_usize, 0_usize], &context),
            (2, 0)
        );
        assert_eq!(
            calc_hint(&vec![0_usize, 1_usize], &vec![1_usize, 0_usize], &context),
            (0, 2)
        );
    }

    #[test]
    fn test_calc_hint_6color_3pin() {
        let context = def::Context {
            pin_num: 3,
            color_num: 6,
            duplicate: true,
        };
        assert_eq!(
            calc_hint(
                &vec![0_usize, 0_usize, 4_usize],
                &vec![1_usize, 1_usize, 5_usize],
                &context
            ),
            (0, 0)
        );
        assert_eq!(
            calc_hint(
                &vec![0_usize, 0_usize, 4_usize],
                &vec![0_usize, 1_usize, 5_usize],
                &context
            ),
            (1, 0)
        );
        assert_eq!(
            calc_hint(
                &vec![0_usize, 0_usize, 4_usize],
                &vec![1_usize, 0_usize, 5_usize],
                &context
            ),
            (1, 0)
        );
        assert_eq!(
            calc_hint(
                &vec![0_usize, 0_usize, 4_usize],
                &vec![0_usize, 0_usize, 5_usize],
                &context
            ),
            (2, 0)
        );
        assert_eq!(
            calc_hint(
                &vec![0_usize, 1_usize, 4_usize],
                &vec![1_usize, 0_usize, 5_usize],
                &context
            ),
            (0, 2)
        );
    }
}
