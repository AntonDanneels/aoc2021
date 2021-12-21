use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn diff(&self, other: &Point) -> Point {
        Point {
            x: other.x - self.x,
            y: other.y - self.y,
            z: other.z - self.z,
        }
    }

    fn add(&self, other: &Point) -> Point {
        Point {
            x: other.x + self.x,
            y: other.y + self.y,
            z: other.z + self.z,
        }
    }

    fn size(&self) -> f32 {
        let s = self.x * self.x + self.y * self.y + self.z * self.z;

        (s as f32).sqrt()
    }

    fn dot(&self, other: &Point) -> f32 {
        let s1 = self.size();
        let s2 = self.size();
        let s = s1 * s2;

        let v = self.x * other.x + self.y * other.y + self.z * other.z;

        if s == 0.0 {
            0.0
        } else {
            v as f32 / s as f32
        }
    }

    fn equals(&self, other: &Point) -> bool {
        return self.x == other.x && self.y == other.y && self.z == other.z;
    }

    fn apply_transform(&self, transform: u8) -> Point {
        match transform {
            0 => Point {
                x: self.x,
                y: self.y,
                z: self.z,
            },
            1 => Point {
                x: self.x,
                y: self.z,
                z: -self.y,
            },
            2 => Point {
                x: self.x,
                y: -self.y,
                z: -self.z,
            },
            3 => Point {
                x: self.x,
                y: -self.z,
                z: self.y,
            },

            4 => Point {
                x: self.y,
                y: self.z,
                z: self.x,
            },
            5 => Point {
                x: self.y,
                y: self.x,
                z: -self.z,
            },
            6 => Point {
                x: self.y,
                y: -self.z,
                z: -self.x,
            },
            7 => Point {
                x: self.y,
                y: -self.x,
                z: self.z,
            },

            8 => Point {
                x: self.z,
                y: self.x,
                z: self.y,
            },
            9 => Point {
                x: self.z,
                y: self.y,
                z: -self.x,
            },
            10 => Point {
                x: self.z,
                y: -self.x,
                z: -self.y,
            },
            11 => Point {
                x: self.z,
                y: -self.y,
                z: self.x,
            },

            12 => Point {
                x: -self.x,
                y: self.z,
                z: self.y,
            },
            13 => Point {
                x: -self.x,
                y: self.y,
                z: -self.z,
            },
            14 => Point {
                x: -self.x,
                y: -self.z,
                z: -self.y,
            },
            15 => Point {
                x: -self.x,
                y: -self.y,
                z: self.z,
            },

            16 => Point {
                x: -self.y,
                y: self.x,
                z: self.z,
            },
            17 => Point {
                x: -self.y,
                y: self.z,
                z: -self.x,
            },
            18 => Point {
                x: -self.y,
                y: -self.x,
                z: -self.z,
            },
            19 => Point {
                x: -self.y,
                y: -self.z,
                z: self.x,
            },

            20 => Point {
                x: -self.z,
                y: self.y,
                z: self.x,
            },
            21 => Point {
                x: -self.z,
                y: self.x,
                z: -self.y,
            },
            22 => Point {
                x: -self.z,
                y: -self.y,
                z: -self.x,
            },
            23 => Point {
                x: -self.z,
                y: -self.x,
                z: self.y,
            },

            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct Scanner {
    beacons: HashSet<Point>,
    position: Point,
}

fn parse_scanner(data: &mut dyn Iterator<Item = &str>) -> Option<Scanner> {
    if let Some(_) = data.next() {
        let mut result = Scanner {
            beacons: HashSet::new(),
            position: Point { x: 0, y: 0, z: 0 },
        };

        loop {
            if let Some(line) = data.next() {
                if line.is_empty() {
                    break;
                }
                let p = line
                    .split(",")
                    .map(|p| p.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                result.beacons.insert(Point {
                    x: p[0],
                    y: p[1],
                    z: p[2],
                });
            } else {
                break;
            }
        }
        return Some(result);
    }

    None
}

fn match_scanners(s1: &mut Scanner, s2: &mut Scanner) -> bool {
    //println!("{:?}", s1.beacons);
    for p1 in s1.beacons.iter() {
        for p2 in s2.beacons.iter() {
            for t in 0..24 {
                let new_point = p2.apply_transform(t);

                // Set p2 == p1, needs to match with at least 12 points from s1
                let pos = new_point.diff(&p1);

                let mut count = 0;
                let mut transformed_points = Vec::new();
                for point in s2.beacons.iter() {
                    let new_point = point.apply_transform(t);
                    let point_t = new_point.add(&pos);

                    if s1.beacons.contains(&point_t) {
                        count += 1;
                    }

                    transformed_points.push(point_t);
                }

                if count >= 12 {
                    s2.position = pos;

                    for point_t in transformed_points {
                        s1.beacons.insert(point_t);
                    }

                    return true;
                }
            }
        }
    }

    return false;
}

fn manhattan_distance(p1: &Point, p2: &Point) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs() + (p1.z - p2.z).abs()
}

fn main() {
    let mut data = include_str!("day19.txt").lines();

    let mut scanners = Vec::new();
    while let Some(scanner) = parse_scanner(&mut data) {
        scanners.push(scanner);
    }

    let mut scanner0 = scanners.remove(0);

    let mut total_scanners = Vec::new();
    while scanners.len() > 0 {
        let mut i = 0;
        let size_before = total_scanners.len();
        while i < scanners.len() {
            if match_scanners(&mut scanner0, &mut scanners[i]) {
                let val = scanners.remove(i);
                total_scanners.push(val);
                println!("Matched {} scanners", total_scanners.len());
            } else {
                i += 1;
            }
        }

        if size_before == total_scanners.len() {
            panic!();
        }
    }

    for scanner in total_scanners.iter() {
        println!("{:?}", scanner.position);
    }

    let mut max_distance = 0;
    for i in 0..total_scanners.len() {
        for j in (i + 1)..total_scanners.len() {
            max_distance = std::cmp::max(
                max_distance,
                manhattan_distance(&total_scanners[i].position, &total_scanners[j].position),
            );
        }
    }

    println!("{:?}", max_distance);

    println!("{:?}", scanner0.beacons.len());
}

/*
fn match_scanners(s1: &Scanner, s2: &Scanner) {
    let mut diffs1 = Vec::new();

    for i in 0..s1.beacons.len() {
        for j in (i + 1)..s1.beacons.len() {
            let p1 = &s1.beacons[i];
            let p2 = &s1.beacons[j];
            let diff = Point {
                x: (p1.x - p2.x).abs(),
                y: (p1.y - p2.y).abs(),
                z: (p1.z - p2.z).abs(),
            };
            diffs1.push((diff, (p1, p2)));
        }
    }
    let mut diffs2 = Vec::new();

    for i in 0..s2.beacons.len() {
        for j in (i + 1)..s2.beacons.len() {
            let p1 = &s2.beacons[i];
            let p2 = &s2.beacons[j];
            let diff = Point {
                x: (p1.x - p2.x).abs(),
                y: (p1.y - p2.y).abs(),
                z: (p1.z - p2.z).abs(),
            };
            diffs2.push((diff, (p1, p2)));
        }
    }

    println!("{}", diffs1.len());

    let mut unique_points = HashSet::new();
    let mut unique_points2 = HashSet::new();
    let mut similar_vectors = Vec::new();

    for d1 in diffs1 {
        for d2 in &diffs2 {
            if d1.0.equals(&d2.0) {
                unique_points.insert(d1.1 .0);
                unique_points.insert(d1.1 .1);

                unique_points2.insert(d2.1 .0);
                unique_points2.insert(d2.1 .1);

                similar_vectors.push((d1.1, d2.1));
            }
        }
    }

    let pos_of_scanner = ();

    if unique_points.len() >= 12 {
        for m in similar_vectors {
            //println!("{:?}", m);

            let (p1, p2) = m.0;
            let (p3, p4) = m.1;

            if let None = unique_points.get(p1) {
                continue;
            }
            if let None = unique_points.get(p2) {
                continue;
            }

            let mut scanner_x = p1.x + p3.x;
            let mut scanner_y = p1.y + p3.y;
            let mut scanner_z = p1.z + p3.z;
            if scanner_x - p2.x != p4.x {
                scanner_x = p1.x - p3.x;
            }
            if scanner_y - p2.y != p4.y {
                scanner_y = p1.y - p3.y;
            }
            if scanner_z - p2.z != p4.z {
                scanner_z = p1.z - p3.z;
            }

            //println!("{}, {}, {}", scanner_x, scanner_y, scanner_z);
        }
    }

    println!("{:?}", unique_points.len());
    println!("{:?}", unique_points2.len());

    for point in unique_points {
        println!("{:?}", point);
    }
    println!("-------------------------------------");
    for point in unique_points2 {
        println!("{} {:?}", -point.x + 68, point);
    }
}

*/
