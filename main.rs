use std::io::Write;

fn main() {
    // temporary setting
    let pin_num = 2;  // number of pins
    let color_num = 3;  // number of colors
    let duplicate = false;  // code dose not have same colors

    // manually enumeration of all codes
    let all_codes: Vec<Vec<usize>> = vec![vec![0, 1], vec![0, 2], vec![1, 0], vec![1, 2], vec![2, 0], vec![2, 1]];

    // main process
    let candidates = all_codes.clone();
    while candidates.len() > 1 {
        let guess = policy(candidates);
        let hint = trial(guess);
        todo!("update_candidates()");
    }
    assert_eq!(candidates.len(), 1);

    // post process
    let secret = &candidates[0];
    println!("Your secret is {:?}", secret);
}

// select guess from guess set
fn policy(guess_set: Vec<Vec<usize>>) -> Vec<usize> {
    guess_set[0].clone()
}

// get a hint according to the guess from user input
fn trial(guess: Vec<usize>) -> (usize, usize) {
    println!("Guess is {:?}", guess);
    print!("input hit and blow (format: hit blow)> ");
    std::io::stdout().flush().unwrap();
    let mut input_string = String::new();
    std::io::stdin().read_line(&mut input_string).ok();
    let parts: Vec<&str> = input_string.split_whitespace().collect();
    let hit = parts[0].parse().expect("Hit should be an integer number.");
    let blow = parts[1].parse().expect("Blow should be an integer number.");
    (hit, blow)
}
