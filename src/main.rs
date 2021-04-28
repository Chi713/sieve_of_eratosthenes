//use math::round;
use std::time::Instant;
use std::io;
mod sieve;

fn main() {
    // Prepare input for sieving function
    println!("enter value");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("failed to read line");
    //println!("{:?}", guess);
    let len = guess.trim_end_matches(&['\r', '\n'][..]).len();
    guess.truncate(len);

    let range: usize = guess.parse().unwrap();
    
    // Start function timer
    let start = Instant::now();

    //let range = 1000000;

    let result = sieve::gen(range);
    // Stop function timer
    let duration = start.elapsed();
    println!("value: {}", result[result.len()-1]);
    println!("binary: {:#034b}", result[result.len()-1]);
    println!("Time Elapsed: {:?}", duration);
}
