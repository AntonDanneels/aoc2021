use std::collections::HashMap;

fn num_children(start: u64, days: u64, memory: &mut HashMap<(u64, u64), u64>) -> u64 {
    if let Some(x) = memory.get(&(start, days)) {
        return *x;
    }

    if days <= start {
        return 0;
    }

    let score = num_children(7, days - start, memory) + 1 + num_children(9 , days - start, memory);
    memory.insert((start, days), score);

    score
}

fn main() {
    //let input_data = vec![3,4,3,1,2];
    let input_data = vec![4,1,1,1,5,1,3,1,5,3,4,3,3,1,3,3,1,5,3,2,4,4,3,4,1,4,2,2,1,3,5,1,1,3,2,5,1,1,4,2,5,4,3,2,5,3,3,4,5,4,3,5,4,2,5,5,2,2,2,3,5,5,4,2,1,1,5,1,4,3,2,2,1,2,1,5,3,3,3,5,1,5,4,2,2,2,1,4,2,5,2,3,3,2,3,4,4,1,4,4,3,1,1,1,1,1,4,4,5,4,2,5,1,5,4,4,5,2,3,5,4,1,4,5,2,1,1,2,5,4,5,5,1,1,1,1,1,4,5,3,1,3,4,3,3,1,5,4,2,1,4,4,4,1,1,3,1,3,5,3,1,4,5,3,5,1,1,2,2,4,4,1,4,1,3,1,1,3,1,3,3,5,4,2,1,1,2,1,2,3,3,5,4,1,1,2,1,2,5,3,1,5,4,3,1,5,2,3,4,4,3,1,1,1,2,1,1,2,1,5,4,2,2,1,4,3,1,1,1,1,3,1,5,2,4,1,3,2,3,4,3,4,2,1,2,1,2,4,2,1,5,2,2,5,5,1,1,2,3,1,1,1,3,5,1,3,5,1,3,3,2,4,5,5,3,1,4,1,5,2,4,5,5,5,2,4,2,2,5,2,4,1,3,2,1,1,4,4,1,5];
    let days = 256;

    let mut memory = HashMap::new();
    let score = input_data.iter()
        .fold(input_data.len() as u64, |accum, value| accum + num_children(*value as u64, days as u64, &mut memory));

    println!("Total: {}", score);
}
