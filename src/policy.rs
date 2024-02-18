use crate::def;

// select guess from guess set
pub fn first_pick(guess_set: &def::CodeSet) -> def::Code {
    guess_set[0].clone()
}
