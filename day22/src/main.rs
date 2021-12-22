#[derive(Clone, Debug)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Clone, Debug)]
struct Box {
    // Bottom "left"
    min: Point,
    // Top "right"
    max: Point,
}

fn overlap(x1min: i64, x1max: i64, x2min: i64, x2max: i64) -> bool {
    return x2max >= x1min && x1max >= x2min;
}

impl Point {
    fn new(x: i64, y: i64, z: i64) -> Point {
        Point { x, y, z }
    }
}

impl Box {
    fn intersects(&self, other: &Box) -> bool {
        overlap(self.min.x, self.max.x, other.min.x, other.max.x)
            && overlap(self.min.y, self.max.y, other.min.y, other.max.y)
            && overlap(self.min.z, self.max.z, other.min.z, other.max.z)
    }

    fn new(min: Point, max: Point) -> Box {
        Box { min, max }
    }

    fn volume(&self) -> u64 {
        (self.max.x - self.min.x) as u64
            * (self.max.y - self.min.y) as u64
            * (self.max.z - self.min.z) as u64
    }
}

fn parse_min_max(entry: &str) -> (i64, i64) {
    let split = entry.split("=").collect::<Vec<&str>>();
    let min_max = split[1].split("..").collect::<Vec<&str>>();

    (
        min_max[0].parse::<i64>().unwrap(),
        min_max[1].parse::<i64>().unwrap(),
    )
}

fn parse_cube(line: Option<&str>) -> Option<(Box, bool)> {
    if line.is_none() {
        return None;
    }
    let line = line.unwrap().split(" ").collect::<Vec<&str>>();

    let on = match line[0] {
        "on" => true,
        "off" => false,
        _ => return None,
    };

    let mut result = Box {
        min: Point { x: 0, y: 0, z: 0 },
        max: Point { x: 0, y: 0, z: 0 },
    };

    let coords = line[1].split(",").collect::<Vec<&str>>();
    let (xmin, xmax) = parse_min_max(coords[0]);
    let (ymin, ymax) = parse_min_max(coords[1]);
    let (zmin, zmax) = parse_min_max(coords[2]);

    result.min.x = xmin;
    result.min.y = ymin;
    result.min.z = zmin;

    result.max.x = xmax + 1;
    result.max.y = ymax + 1;
    result.max.z = zmax + 1;

    Some((result, on))
}

fn calc_subsection(x1min: i64, x1max: i64, x2min: i64, x2max: i64, part: u8) -> (i64, i64) {
    match part {
        0 => (x1min, std::cmp::max(x1min, x2min)),
        1 => (std::cmp::max(x1min, x2min), std::cmp::min(x1max, x2max)),
        2 => (std::cmp::min(x1max, x2max), x1max),
        _ => panic!(),
    }
}

fn carve_box(b1: &Box, b2: &Box) -> Vec<Box> {
    let mut new_boxes = Vec::new();
    for x in 0..3 {
        for y in 0..3 {
            for z in 0..3 {
                let (xmin, xmax) = calc_subsection(b1.min.x, b1.max.x, b2.min.x, b2.max.x, x);
                let (ymin, ymax) = calc_subsection(b1.min.y, b1.max.y, b2.min.y, b2.max.y, y);
                let (zmin, zmax) = calc_subsection(b1.min.z, b1.max.z, b2.min.z, b2.max.z, z);

                if xmax > xmin && ymax > ymin && zmax > zmin && x != 1 || y != 1 || z != 1 {
                    new_boxes.push(Box::new(
                        Point::new(xmin, ymin, zmin),
                        Point::new(xmax, ymax, zmax),
                    ));
                }
            }
        }
    }

    new_boxes.into_iter().filter(|b| b.volume() != 0).collect::<Vec<Box>>()
}

fn main() {
    let mut data = include_str!("day22.txt").lines();
    let mut boxes = Vec::new();

    while let Some(x) = parse_cube(data.next()) {
        boxes.push(x);
    }

    /*
    let boxes = boxes.iter().map(|(b, on)| {
        let mut b = b.clone();
        b.min.x = std::cmp::max(-50, b.min.x);
        b.min.y = std::cmp::max(-50, b.min.y);
        b.min.z = std::cmp::max(-50, b.min.z);

        b.max.x = std::cmp::min(51, b.max.x);
        b.max.y = std::cmp::min(51, b.max.y);
        b.max.z = std::cmp::min(51, b.max.z);

        (b, *on)
    }).filter(|(b, _)| {
        b.min.x >= -50 && b.max.x >= -50 &&
        b.min.y >= -50 && b.max.y >= -50 &&
        b.min.z >= -50 && b.max.z >= -50 &&

        b.min.x <= 50 && b.max.x <= 51 &&
        b.min.y <= 50 && b.max.y <= 51 &&
        b.min.z <= 50 && b.max.z <= 51
    }).collect::<Vec<(Box, bool)>>();
    */

    let mut current_on: Vec<Box> = Vec::new();

    for (b, on) in boxes {
        let mut next_on: Vec<Box> = Vec::new();
        for bb in current_on.iter() {
            if bb.intersects(&b) {
                next_on.append(&mut carve_box(&bb, &b));
            } else {
                next_on.push(bb.clone());
            }
        }
        if on {
            next_on.push(b);
        }
        current_on = next_on;
    }

    println!("{:?}", current_on.iter().fold(0, |a, b| a + b.volume()));
}
