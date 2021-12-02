use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("day02.txt")?;
    let reader = BufReader::new(file);

    //let mut data = Vec::new();
    let mut depth = 0;
    let mut pos = 0;
    let mut aim = 0;
    for line in reader.lines() {
        let line = line.expect("Invalid file");
        let vals: Vec<&str> = line.split(" ").collect();
        let command = vals.get(0).expect("Invalid input file");
        let value = vals.get(1).expect("Invalid input file")
                               .parse::<i32>().expect("Invalid value");

        match *command {
            "forward" => {
                pos += value;
                depth += aim * value;
            },
            "down" => {
                //depth += value;
                aim += value;
            },
            "up" => {
                //depth -= value;
                aim -= value;
            },
            _ => panic!("Unknown command {}", *command)
        };
    }

    println!("Movement: {}", depth * pos);

    Ok(())
}
