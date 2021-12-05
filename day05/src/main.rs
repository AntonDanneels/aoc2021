use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;

#[derive(Debug, Eq, Clone)]
struct Vec2 {
    x: i32,
    y: i32
}

impl PartialEq for Vec2 {
    fn eq(&self, other: &Vec2) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl std::hash::Hash for Vec2 {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        state.write_i32(self.x);
        state.write_i32(self.y);
        state.finish();
    }
}

fn size(a: &Vec2) -> i32 {
    (((a.x * a.x) + (a.y * a.y)) as f64).sqrt() as i32
}

fn dir(a: &Vec2, b: &Vec2) -> Vec2 {
    let x = a.x - b.x;
    let y = a.y - b.y;
    let tmp = Vec2 {x, y};
    let size = size(&tmp);

    if size == 0 {
        return Vec2 { x: 0, y: 0 };
    }

    Vec2 {
        x: (x as f64 / size as f64).round() as i32,
        y: (y as f64 / size as f64).round() as i32
    }
}

fn add(a: &Vec2, b: &Vec2) -> Vec2 {
    Vec2 {
        x: a.x + b.x,
        y: a.y + b.y
    }
}

fn equals(a: &Vec2, b: &Vec2) -> bool {
    a.x == b.x && a.y == b.y
}

fn add_point(map: &mut HashMap<Vec2, u32>, point: &Vec2) {
    if let Some(s) = map.get_mut(point) {
        *s += 1;
    } else {
        map.insert(point.clone(), 1);
    }
}

fn main() -> io::Result<()> {
    let file = File::open("day05.txt")?;
    let reader = BufReader::new(file);

    let mut points: HashMap<Vec2, u32> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split("->").collect();

        if parts.len() != 2 {
            panic!("Invalid parsing");
        }

        let start: Vec<i32> = parts.first().expect("Invalid input data")
            .trim().split(",").collect::<Vec<&str>>()
            .iter()
            .map(|x| x.parse::<i32>().expect(""))
            .collect();

        let start = Vec2 {
            x: start[0],
            y: start[1]
        };

        let end: Vec<i32>= parts.last().expect("Invalid input data")
            .trim().split(",").collect::<Vec<&str>>()
            .iter()
            .map(|x| x.parse::<i32>().expect(""))
            .collect();

        let end = Vec2 {
            x: end[0],
            y: end[1]
        };

        let delta = dir(&end, &start);

        if size(&delta) != 0 {
            add_point(&mut points, &start);
            add_point(&mut points, &end);
            //println!("{:?} -> {:?} ({:?})", start, end, delta);
            let mut point = add(&start, &delta);
            while !equals(&point, &end) {
                //println!("{:?}", point);
                add_point(&mut points, &point);
                point = add(&point, &delta);
            }
        }
    }

    let score: usize = points.values()
        .filter(|x| **x >= 2)
        .count();

    println!("Score: {:?}", score);

    Ok(())
}
