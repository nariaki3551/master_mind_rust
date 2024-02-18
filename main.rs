use std::io::Write;

fn main() {
    // temporary setting
    let pin_num = 2;  // number of pins
    let color_num = 3;  // number of colors
    let duplicate = false;  // code dose not have same colors
    println!(
        "Context: pin_num: {}, color_num: {}, duplicate: {}",
        pin_num, color_num, duplicate
    );
    println!(
        "You decide the secret code with colors from 0 to {}, and I will guess it. Let's start!",
        color_num - 1
    );

    // manually enumeration of all codes
    let all_codes: Vec<Vec<usize>> = vec![vec![0, 1], vec![0, 2], vec![1, 0], vec![1, 2], vec![2, 0], vec![2, 1]];

    // main process
    let mut candidates = all_codes.clone();
    while candidates.len() > 1 {
        let guess = policy(&candidates);
        let hint = trial(&guess);
        candidates.retain(|x| calc_hint(x, &guess) == hint); // update candidates
    }
    assert_eq!(candidates.len(), 1);

    // post process
    let secret = &candidates[0];
    println!("Your secret is {:?}", secret);
}

// select guess from guess set
fn policy(guess_set: &[Vec<usize>]) -> Vec<usize> {
    guess_set[0].clone()
}

// get a hint according to the guess from user input
fn trial(guess: &Vec<usize>) -> (usize, usize) {
    println!("Guess is {:?}", guess);
    print!("input hit and blow (format: hit blow)> ");
    std::io::stdout().flush().unwrap();
    let mut input_string = String::new();
    std::io::stdin().read_line(&mut input_string).ok();
    let parts: Vec<&str> = input_string.split_whitespace().collect();
    let hit = parts[0].parse().expect("Hit should be an integer number.");
    let blow = parts[1].parse().expect("Blow should be an integer number.");
    println!("Hint is {:?}", (hit, blow));
    (hit, blow)
}

// calculate hint from two codes
fn calc_hint(code: &[usize], guess: &[usize]) -> (usize, usize) {
    let mut hit = 0;
    let mut blow = 0;
    let mut code_color_counts = [0; 3]; // array of the number of i-th colors which code has
    let mut guess_color_counts = [0; 3]; // array of the number of i-th colors which guess has
    for i in 0..2 {
        if code[i] == guess[i] {
            hit += 1;
        } else {
            code_color_counts[code[i]] += 1;
            guess_color_counts[guess[i]] += 1;
        }
    }
    for i in 0..3 {
        blow += std::cmp::min(code_color_counts[i], guess_color_counts[i]);
    }
    (hit, blow)
}
