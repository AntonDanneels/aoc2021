use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;

const BOARD_SIZE: usize = 5;

#[derive(Debug)]
struct BingoBoard {
   board: [[u8; BOARD_SIZE] ; BOARD_SIZE],
   numbers: HashMap<u8, (usize, usize)>,
   unmarked: HashMap<u8, ()>
}

impl BingoBoard {
    fn print_board(&self) {
        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                let marked = if self.unmarked.get(&self.board[x][y]).is_some() {
                   ""
                } else {
                   "*"
                };
                print!("\t{}{}\t", self.board[x][y], marked);
            }
            println!();
        }
    }

    fn mark_number(&mut self, num: u8) -> bool {
        if let Some(pos) = self.numbers.get(&num) {
            self.unmarked.remove(&num);

            let mut row_ok = true;
            let mut col_ok = true;
            for i in 0..BOARD_SIZE {
                if self.unmarked.get(&self.board[pos.0][i]).is_some() {
                    row_ok = false;
                }
                if self.unmarked.get(&self.board[i][pos.1]).is_some() {
                    col_ok = false;
                }
            }

            return row_ok || col_ok;
        }

        false
    }
}

fn main() -> io::Result<()> {
    let file = File::open("day04.txt")?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    let numbers: Vec<u8> = lines.next().unwrap()?
        .split(",")
        .map(|x| x.parse().expect("Invalid input data"))
        .collect();
    let mut boards = Vec::new();

    loop {
        /* Skip empty line */
        if let None = lines.next() {
            break;
        }

        let mut board = BingoBoard {
            board: [[0; BOARD_SIZE]; BOARD_SIZE],
            numbers: HashMap::new(),
            unmarked: HashMap::new()
        };

        for row in 0..BOARD_SIZE {
            let values = lines.next().unwrap()?;

            for (col, val) in values.split_whitespace().enumerate() {
                let num = val.parse().expect("Invalid input data");
                board.board[row][col] = num;
                board.numbers.insert(num, (row, col));
                board.unmarked.insert(num, ());
            }
        }

        boards.push(board);
    }

    let mut draw = numbers.iter();
    let mut boards_by_score = Vec::new();
    loop {
        let num = draw.next();
        if num.is_none() {
            break;
        }
        let num = num.unwrap();

        // Only available in nightly...
        /*
        let removed = boards.drain_filter(|&mut board| {
            board.mark_number(*num)
        });
        */

        let mut i = 0;
        while i < boards.len() {
            if boards[i].mark_number(*num) {
                let val = boards.remove(i);
                boards_by_score.push((val, num));
            } else {
                i += 1;
            }
        }

        if boards.is_empty() {
            break;
        }
    }

    let winner = boards_by_score.first().unwrap();
    let sum: u32 = winner.0.unmarked.keys()
        .into_iter()
        .map(|x| *x as u32)
        .reduce(|a, b| a + b)
        .unwrap();

    println!("Winner Sum: {}", sum);
    println!("Winner Score: {}", sum * *winner.1 as u32);

    let loser = boards_by_score.last().unwrap();
    let sum: u32 = loser.0.unmarked.keys()
        .into_iter()
        .map(|x| *x as u32)
        .reduce(|a, b| a + b)
        .unwrap();

    println!("Loser Sum: {}", sum);
    println!("Loser Score: {}", sum * *loser.1 as u32);

    Ok(())
}
