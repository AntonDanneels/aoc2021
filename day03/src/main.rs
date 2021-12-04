use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("day03.txt")?;
    let reader = BufReader::new(file);

    let mut counts = Vec::new();
    let mut values = Vec::new();
    let mut size = 0;

    for line in reader.lines() {
        let line = line.expect("Invalid file");
        if size == 0 {
            size = line.len();
            counts.resize(size, 0);
        }

        let val = i32::from_str_radix(line.as_str(), 2).expect("");
        values.push(val);

        for idx in 0..size {
            if let Some(x) = counts.get_mut(idx) {
                if ((val >> size - 1 - idx) & 0b1) == 1 {
                    *x += 1;
                } else {
                    *x -= 1;
                }
            }
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;
    for (i, count) in counts.iter().enumerate() {
        if *count > 0 {
            gamma |= 1 << (counts.len() - 1 - i);
        } else {
            epsilon |= 1 << (counts.len() - 1 - i);
        }
    }

    println!("{} {} {}", gamma, epsilon, gamma * epsilon);

    println!("{:?}", counts);

    let mut idx = 0;
    let mut candidates = values.clone();
    loop {
        if idx > size {
            panic!("Out of counts without a result!");
        }

        let mut counts = Vec::new();
        counts.resize(size, 0);

        for val in candidates.iter() {
            for idx in 0..size {
                if let Some(x) = counts.get_mut(idx) {
                    if ((val >> size - 1 - idx) & 0b1) == 1 {
                        *x += 1;
                    } else {
                        *x -= 1;
                    }
                }
            }
        }

        let bit = if *counts.get(idx).unwrap() >= 0 {
            1
        } else {
            0
        };

        candidates = candidates.iter()
            .filter(|val| ((*val >> size - 1 - idx) & 0b1) == bit)
            .cloned()
            .collect();

        if candidates.len() == 1 {
            println!("{:?}", candidates);
            break;
        }
        idx += 1;
    }

    let mut idx = 0;
    let mut candidates = values.clone();
    loop {
        if idx > size {
            panic!("Out of counts without a result!");
        }

        let mut counts = Vec::new();
        counts.resize(size, 0);

        for val in candidates.iter() {
            for idx in 0..size {
                if let Some(x) = counts.get_mut(idx) {
                    if ((val >> size - 1 - idx) & 0b1) == 0 {
                        *x += 1;
                    } else {
                        *x -= 1;
                    }
                }
            }
        }

        let bit = if *counts.get(idx).unwrap() > 0 {
            1
        } else {
            0
        };

        candidates = candidates.iter()
            .filter(|val| ((*val >> size - 1 - idx) & 0b1) == bit)
            .cloned()
            .collect();

        if candidates.len() == 1 {
            break;
        }

        idx += 1;
    }
    println!("{:?}", candidates);

    Ok(())
}
