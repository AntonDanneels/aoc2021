use std::collections::HashMap;

fn calc_score(
    p1: u64,
    p2: u64,
    s1: u64,
    s2: u64,
    score_to_win: u64,
    memory: &mut HashMap<(u64, u64, u64, u64), (u64, u64)>,
) -> (u64, u64) {
    if s1 >= score_to_win {
        return (1, 0);
    }
    if s2 >= score_to_win {
        return (0, 1);
    }

    if let Some(score) = memory.get(&(p1, p2, s1, s2)) {
        return *score;
    }

    let mut result: (u64, u64) = (0, 0);
    const BOARD_LOCATIONS: [u64; 10] = [10, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    for d1 in 1..4 {
        for d2 in 1..4 {
            for d3 in 1..4 {
                let index = (p1 + d1 + d2 + d3) % 10;
                let p1_next = BOARD_LOCATIONS[index as usize];
                let s1_next = s1 + p1_next;

                let tmp = calc_score(p2, p1_next, s2, s1_next, score_to_win, memory);
                result = (result.0 + tmp.1, result.1 + tmp.0);
            }
        }
    }
    memory.insert((p1, p2, s1, s2), result);

    result
}

fn main() {
    let p1 = 1;
    let p2 = 3;
    let score_to_win = 21;
    let mut memory = HashMap::new();

    println!("{:?}", calc_score(p1, p2, 0, 0, score_to_win, &mut memory));
}
