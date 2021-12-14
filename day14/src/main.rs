use std::collections::HashMap;

fn main() {
    let data = include_str!("day14.txt");
    let mut lines = data.lines();

    let mut challenge = lines.next().unwrap().chars();
    let mut template = Vec::new();
    let mut current = challenge.next().unwrap();

    // Parse the challenge into pairs + count the occurence of each char
    let mut char_counts: HashMap<char, u64> = HashMap::new();
    loop {
        if let Some(x) = char_counts.get_mut(&current) {
            *x += 1;
        } else {
            char_counts.insert(current, 1);
        }
        let next = challenge.next();
        if let Some(next) = next {
            template.push(format!("{}{}", current, next));
            current = next;
        } else {
            break;
        }
    }

    // Skip empty line
    lines.next().unwrap();

    // Parse pairs & add them twice
    let mut pairs = HashMap::new();
    for line in lines {
        let split = line.split(" -> ").collect::<Vec<&str>>();
        let start_pair = split[0];
        let target = split[1];

        let p1 = format!("{}{}", start_pair.chars().nth(0).unwrap(), target);
        let p2 = format!("{}{}", target, start_pair.chars().nth(1).unwrap());

        pairs.insert(
            String::from(start_pair),
            (vec![p1, p2], target.chars().nth(0).unwrap()),
        );
    }

    let mut current_counts = HashMap::new();
    println!("{:?}", template);
    for pair in template {
        if let Some(x) = current_counts.get_mut(&pair) {
            *x += 1;
        } else {
            current_counts.insert(pair, 1);
        }
    }
    println!("{:?}", pairs);
    println!("{:?}", char_counts);
    println!("{:?}", current_counts);

    for _ in 0..40 {
        let mut new_counts: HashMap<String, u64> = HashMap::new();

        for (pair, count) in current_counts.iter() {
            if let Some((new_pairs, c)) = pairs.get(pair) {
                for new_pair in new_pairs {
                    if let Some(x) = new_counts.get_mut(new_pair) {
                        *x += *count;
                    } else {
                        new_counts.insert((*new_pair).clone(), *count);
                    }
                }
                if let Some(x) = char_counts.get_mut(c) {
                    *x += count;
                } else {
                    char_counts.insert(*c, 1);
                }
            } else {
                panic!();
            }
        }
        current_counts = new_counts;
    }

    let mut max = u64::MIN;
    let mut min = u64::MAX;
    for key in char_counts.keys() {
        if let Some(count) = char_counts.get(key) {
            if *count < min {
                min = *count;
            }
            if *count > max {
                max = *count;
            }
        }
    }
    println!("{:?}", current_counts);
    println!("{:?}", char_counts);
    println!("{} {}", max, min);
    println!("{}", max - min);
}
