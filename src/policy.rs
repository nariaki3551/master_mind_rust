use crate::{def, utils};

// select guess from guess set
pub fn first_pick(guess_set: &def::CodeSet) -> def::Code {
    guess_set[0].clone()
}

// select guess from guesses, which has minimum candidates in worst case
pub fn minmax(
    candidates: &def::CodeSet,
    guess_set: &def::CodeSet,
    context: &def::Context,
) -> def::Code {
    let mut best_guess = guess_set[0].clone();
    let mut min_max_candidate_num = i16::MAX;
    for guess in guess_set {
        let map = utils::calc_hint_based_candidate_num_map(candidates, guess, context);
        let max_candidate_num = map.values().max().cloned().unwrap();
        if max_candidate_num < min_max_candidate_num {
            best_guess = guess.clone();
            min_max_candidate_num = max_candidate_num;
        }
    }
    best_guess
}
