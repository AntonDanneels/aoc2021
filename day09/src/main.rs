fn main() {
    let data: Vec<Vec<u32>> = include_str!("day09.txt")
        .lines()
        .map(|x| {
            x.chars()
                .map(|y| y.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    const NEIGHBORS : [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut minima = Vec::new();
    data.iter()
        .enumerate()
        .fold(&mut minima, |r, (y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, val)| {
                    for (xx, yy) in NEIGHBORS {
                        let r = data.get((y as i32 + yy) as usize);
                        if r.is_none() {
                            continue;
                        }
                        let r = r.unwrap();
                        let n = r.get((x as i32+ xx) as usize);
                        if let Some(n) = n {
                            if n <= val {
                                return None;
                            }
                        }
                    }
                    Some((x, y))
                })
            .filter(|p| p.is_some())
            .map(|x| x.unwrap())
            .for_each(|p| r.push(p));

            r
    });

    // Part 1
    let score: usize = minima.iter().map(|(x, y)| {
        (*data.get(*y).unwrap().get(*x).unwrap() + 1) as usize
    }).sum();

    println!("Minima score: {:?}", score);

    let mut visited = vec![vec![false; data.get(0).unwrap().len()]; data.len()];

    let mut sizes: Vec<u32> = minima.iter().map(|(x, y)| {
        let mut left_to_visit = Vec::new();
        left_to_visit.push((*x, *y));
        visited[*y][*x] = true;

        let mut size = 0;

        loop {
            if left_to_visit.len() == 0 {
                break;
            }
            let (x, y) = left_to_visit.remove(0);
            size += 1;
            for (xx, yy) in NEIGHBORS {
                let yy = (y as i32 + yy) as usize;
                let r = data.get(yy);
                if r.is_none() {
                    continue;
                }
                let r = r.unwrap();
                let xx = (x as i32 + xx) as usize;
                let n = r.get(xx);
                if let Some(n) = n {
                    if *n != 9  && !visited[yy][xx] {
                        visited[yy][xx] = true;
                        left_to_visit.push((xx, yy));
                    }
                }
            }
        }

        size
    }).collect();

    sizes.sort_by(|a, b| a.cmp(b));
    let score = sizes.iter().rev().take(3).fold(1, |a, v| a * v);

    println!("Bassin score: {:?}", score);
}
