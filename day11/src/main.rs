
struct Map {
    status: Vec<Vec<u32>>,
    flashed: Vec<Vec<bool>>,
    width: i32,
    height: i32,
    flashes: u32
}

impl Map {

    fn step(&mut self) -> bool {
        self.flashed = vec![vec![false; self.width as usize]; self.height as usize];

        for y in 0..self.height {
            for x in 0..self.width {
                self.mark_position(x as i32, y as i32);
            }
        }

        let mut all_flash = true;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.status[y as usize][x as usize] > 9 {
                    self.status[y as usize][x as usize] = 0;
                    self.flashes += 1;
                } else {
                    all_flash = false;
                }
            }
        }

        all_flash
    }

    fn mark_position(&mut self, x: i32, y: i32) {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return;
        }

        let y = y as usize;
        let x = x as usize;

        self.status[y][x] += 1;

        if self.status[y][x] > 9 && !self.flashed[y][x] {
            self.flashed[y][x] = true;
            for i in -1..2 {
                for j in -1..2 {
                    if i == 0 && j == 0 {
                        continue;
                    }

                    self.mark_position(x as i32 + i, y as i32 + j);
                }
            }
        }
    }

    fn print(&self) {
        self.status.iter().for_each(|line| println!("{:?}", line));
    }
}

fn main() {
    let map = include_str!("day11.txt")
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect::<Vec<Vec<u32>>>();

    let height = map.len();
    let width = map[0].len();

    let mut map = Map {
        status: map,
        flashed: Vec::new(),
        width: width as i32,
        height: height as i32,
        flashes: 0
    };

    for i in 0..10000 {
        if map.step() {
            println!("All flash at step {}", i + 1);
            break;
        }
    }
    map.print();

    println!("{:?}", map.flashes);
}
