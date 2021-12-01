use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn count_increases(data: &Vec<u32>) -> u32 {
    let mut previous: Option<u32> = None;
    let mut increases = 0;
    for current in data.iter() {
        if let Some(prev) = previous {
            if *current > prev {
                increases += 1;
            }
        }

        previous = Some(*current);
    }

    increases
}

fn create_sliding_window(data: &Vec<u32>, size: usize) -> Result<Vec<u32>, ()> {
    if size < 1 {
        return Err(());
    }
    if size as usize > data.len() {
        return Err(());
    }

    let mut result: Vec<u32> = Vec::new();

    let mut sum: u32 = 0;
    for i in 0..size {
        sum += data.get(i as usize).expect("guard check incomplete");
    }

    result.push(sum);

    for i in size..data.len() {
        sum -= data.get(i - size).expect("");
        sum += data.get(i).expect("");

        result.push(sum);
    }

    Ok(result)
}

fn main() -> io::Result<()> {
    let file = File::open("day01.txt")?;
    let reader = BufReader::new(file);

    let mut data = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Invalid file");
        let current = line.parse::<u32>().expect("Invalid value");
        data.push(current);
    }

    println!("Part one: {}", count_increases(&data));

    let window_size = 3;
    let data2 = create_sliding_window(&data, window_size).expect("Invalid algo");
    println!("Part two: {}", count_increases(&data2));

    Ok(())
}
