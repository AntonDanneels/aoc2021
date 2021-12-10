fn main() {
    let data = include_str!("day10.txt");

    let mut scores = data.lines().map(|line| {
        let mut expected_tokens = Vec::new();
        println!("Processing line: {}", line);
        let s = line.chars().map(|c| {
            match c {
                '(' => expected_tokens.push(')'),
                '[' => expected_tokens.push(']'),
                '{' => expected_tokens.push('}'),
                '<' => expected_tokens.push('>'),
                _ => {
                    let expected = expected_tokens.iter().last().unwrap();

                    if c != *expected {
                        //println!("Expected {}, got {}", *expected, c);

                        return match c {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => 0
                        };
                    } else {
                        expected_tokens.remove(expected_tokens.len() - 1);
                    }
                }
            };

            0
        }
        ).collect::<Vec<u64>>().clone()
        .iter()
        .filter(|x| **x != 0)
        .map(|x| *x)
        .collect::<Vec<u64>>();

        // Part 1
        if let Some(_) = s.first() {
            //return *x;
            return 0;
        }
        println!("Expected tokens: {:?}", expected_tokens);

        expected_tokens.iter().rev().fold(0, |a, c| {
            match c {
                ')' => a * 5 + 1,
                ']' => a * 5 + 2,
                '}' => a * 5 + 3,
                '>' => a * 5 + 4,
                _ => panic!()
            }
        })
    }).collect::<Vec<u64>>()
    .iter()
    .filter(|x| **x != 0)
    .map(|x| *x)
    .collect::<Vec<u64>>();

    println!("{:?}", scores);
    // Part 1
    //println!("{:?}", scores.iter().sum::<u64>());
    // Part 2
    scores.sort();
    println!("{:?}", scores[scores.len() / 2]);
}
