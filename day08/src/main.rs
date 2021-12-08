use std::collections::HashSet;

fn main() {
    let data = include_str!("day08.txt");

    let mut unique_occurances = 0;
    let mut total_sum = 0;
    for line in data.lines() {
        let challenge: Vec<&str> = line.split(" | ")
            .collect();

        let challenge: Vec<Vec<HashSet<char>>> = challenge.iter()
            .map(|x| {
                x.split_whitespace()
                .collect::<Vec<&str>>()
                .iter()
                .map(|entry| {
                    let mut result = HashSet::new();

                    entry.chars().for_each(|c| { result.insert(c); });

                    return result;
                }).collect()
            }).collect();

        let input = challenge.get(0).unwrap();
        let output = challenge.get(1).unwrap();

        let num_1: &HashSet<char> = input.iter().filter(|x| x.len() == 2)
            .collect::<Vec<&HashSet<char>>>()
            .get(0).unwrap();

        let num_4: &HashSet<char> = input.iter().filter(|x| x.len() == 4)
            .collect::<Vec<&HashSet<char>>>()
            .get(0).unwrap();

        let num_2: &HashSet<char> = input.iter().filter(|x| x.len() == 5)
            .collect::<Vec<&HashSet<char>>>()
            .iter()
            .filter(|x| x.intersection(num_4).collect::<HashSet<&char>>().len() == 2)
            .map(|x| *x)
            .collect::<Vec<&HashSet<char>>>()
            .get(0).unwrap();

        let num_3: &HashSet<char> = input.iter().filter(|x| x.len() == 5)
            .collect::<Vec<&HashSet<char>>>()
            .iter()
            .filter(|x| x.intersection(num_1).collect::<HashSet<&char>>().len() == 2)
            .map(|x| *x)
            .collect::<Vec<&HashSet<char>>>()
            .get(0).unwrap();

        let num_6: &HashSet<char> = input.iter().filter(|x| x.len() == 6)
            .collect::<Vec<&HashSet<char>>>()
            .iter()
            .filter(|x| x.intersection(num_1).collect::<HashSet<&char>>().len() != 2)
            .map(|x| *x)
            .collect::<Vec<&HashSet<char>>>()
            .get(0).unwrap();

        let num_9: &HashSet<char> = input.iter().filter(|x| x.len() == 6)
            .collect::<Vec<&HashSet<char>>>()
            .iter()
            .filter(|x| x.intersection(num_3).collect::<HashSet<&char>>().len() == 5)
            .map(|x| *x)
            .collect::<Vec<&HashSet<char>>>()
            .get(0).unwrap();

        let output_num: u32 = output.iter().map(|x| {
            // Part 1
            if x.len() != 5 && x.len() != 6 {
                unique_occurances += 1;
            }

            match x.len() {
                2 => return 1,
                4 => return 4,
                3 => return 7,
                7 => return 8,

                5 => {
                    if x.intersection(num_2).collect::<HashSet<&char>>().len() == x.len() {
                        return 2;
                    }
                    if x.intersection(num_3).collect::<HashSet<&char>>().len() == x.len() {
                        return 3;
                    }

                    return 5;
                }

                6 => {
                    if x.intersection(num_6).collect::<HashSet<&char>>().len() == x.len() {
                        return 6;
                    }
                    if x.intersection(num_9).collect::<HashSet<&char>>().len() == x.len() {
                        return 9;
                    }

                    return 0;
                }

                _ => {}
            };

            10
        }).rev()
        .enumerate()
        .map(|(i, x)| x * (10 as u32).pow(i as u32))
        .fold(0, |x, a| a + x);

        total_sum += output_num;
    }

    println!("{}", unique_occurances);
    println!("{}", total_sum);
}
