//use math::round;
use std::time::Instant;
use std::io;

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

    let result = sieve(range);
    // Stop function timer
    let duration = start.elapsed();
    println!("value: {}", result[result.len()-1]);
    println!("binary: {:#034b}", result[result.len()-1]);
    println!("Time Elapsed: {:?}", duration);
}


fn sieve(range: usize) -> Vec<usize> {
    let max_check = (range as f64).sqrt().trunc() as usize;
    let mut list = vec![false; range -1];
    let mut final_list: Vec<usize> = Vec::new();

    for i in 0..(max_check-1) {
        
        if list[i] == false { 
            for j in i..(range-1) {
                if list[j] == false {
                  list[j] = ((j+2) % (i+2) == 0) & (j!= i);
                }
            }
        }
    }
    let mut pos: usize = 2;
   for value in list {
      if !value {
        final_list.push(pos);
      }
      pos += 1;
    } 
    // Returns list
    final_list
}
