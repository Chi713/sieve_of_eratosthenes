pub fn gen(range: usize) -> Vec<usize> {
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