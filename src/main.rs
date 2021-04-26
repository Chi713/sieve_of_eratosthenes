//use math::round;

fn main() {
    let range =9 + 1;
    let max_check = (range as f64).sqrt().trunc() as usize;
    let mut list = vec![false; range-1];

    println!("max_check: {}", max_check);

    for i in 2..max_check {
        
        println!("i: {}\n", i);
        
        if list[i-2] == false { 
            for j in 2..range {
                println!("j: {}",j);
                println!("j mod(i): {}", j% i);
                println!("list at index:{}", list[j-2]);

                list[j-2] = list[j-2] ^ (j % i == 0);

                println!("list at index:{}", list[j-2]);
            }
        }
        //println!("{:?}", list);
        //println!("{}", i);
    }
    println!("{:?}", list);
}
