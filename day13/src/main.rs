use std::collections::HashSet;

#[derive(Debug)]
enum FoldDirection {
    UP,
    LEFT,
}

fn main() {
    let data = include_str!("day13.txt");
    let mut lines = data.lines();

    let mut points = HashSet::new();
    while let Some(line) = lines.next() {
        if line.trim().is_empty() {
            break;
        }
        let split = line.split(",").collect::<Vec<&str>>();
        let x = split[0].parse::<u32>().unwrap();
        let y = split[1].parse::<u32>().unwrap();

        points.insert((x, y));

        println!("Point: {:?}", (x, y));
    }

    let mut folds = Vec::new();
    while let Some(line) = lines.next() {
        if line.trim().is_empty() {
            break;
        }

        let fold = line.trim_start_matches("fold along ");
        let split = fold.split("=").collect::<Vec<&str>>();

        let dir = match split[0] {
            "x" => FoldDirection::LEFT,
            "y" => FoldDirection::UP,
            _ => panic!(),
        };

        let offset = split[1].parse::<u32>().unwrap();

        println!("Fold: {:?}", (&dir, offset));
        folds.push((dir, offset));
    }

    for (dir, offset) in folds.iter() {
        points = points
            .iter()
            .map(|(x, y)| match dir {
                FoldDirection::UP => {
                    if y < offset {
                        return (*x, *y);
                    } else {
                        let delta = y - offset;
                        return (*x, *y - 2 * delta);
                    }
                }
                FoldDirection::LEFT => {
                    if x < offset {
                        return (*x, *y);
                    } else {
                        let delta = x - offset;
                        return (*x - 2 * delta, *y);
                    }
                }
            })
            .collect();
    }

    println!("{:?}", points);
    println!("{:?}", points.len());

    let x_max: u32 = points.iter().map(|(x, _)| *x).max().unwrap();
    let y_max: u32 = points.iter().map(|(_, y)| *y).max().unwrap();

    println!("{}, {}", x_max, y_max);

    for y in 0..(y_max + 1) {
        for x in 0..(x_max + 1) {
            if points.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
