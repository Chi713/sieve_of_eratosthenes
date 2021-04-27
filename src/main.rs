//use math::round;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let range = 1000000;
    let max_check = (range as f64).sqrt().trunc() as usize;
    let mut list = vec![false; range -1];
    let mut final_list: Vec<usize> = Vec::new();

    //println!("max_check: {}", max_check);

    for i in 0..(max_check-1) {
        
        //println!("i: {}\n", i);
        
        if list[i] == false { 
            for j in i..(range-1) {
                //println!("j: {}",j);
                //println!("j mod(i): {}", ((j+2) % (i+2)));
                //println!("list at index:{}", list[j]);
                if list[j] == false {
                  list[j] = ((j+2) % (i+2) == 0) & (j!= i);
                }
                //println!("list at index:{}", list[j]);
            }
        }
        //println!("{:?}", list);
        //println!("{}", i);
    }
    //println!("final: {:?}", list);
   for (pos, value) in list.iter().enumerate() {
      //println!("{:?}", pos);
      if !*value {
        final_list.push(2+pos);
      }
      //final_list[pos] = (2+pos) * (*value==false) as usize; 
    } 

    let duration = start.elapsed();
    println!("{}", final_list[final_list.len()-1]);
    println!("Time Elapsed: {:?}", duration);
}
