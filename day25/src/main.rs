use std::collections::HashSet;

fn render_map(hcreatures: &HashSet<(usize, usize)>, vcreatures: &HashSet<(usize, usize)>, width: usize, height: usize) {
    for y in 0..height {
        for x in 0..width {
            if hcreatures.contains(&(x,y)) {
                print!(">");
            } else if vcreatures.contains(&(x,y)) {
                print!("v");
            } else {
                print!(".");
            }
        }

        println!();
    }
    println!();
}

fn main() {
    let data = include_str!("day25.txt").lines().collect::<Vec<&str>>();

    let height = data.len();
    let width = data[0].len();

    let mut hcreatures = HashSet::new();
    let mut vcreatures = HashSet::new();
    for (y, line) in data.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '>' => hcreatures.insert((x, y)),
                'v' => vcreatures.insert((x, y)),
                _ => false,
            };
        }
    }

    println!("{:?}", hcreatures);
    println!("{:?}", vcreatures);

    render_map(&hcreatures, &vcreatures, width, height);

    let mut steps = 0;

    loop {
        let mut has_moved = false;

        let mut hcreatures_new = HashSet::new();
        let mut vcreatures_new = HashSet::new();

        for (x,y) in hcreatures.iter() {
            let new_pos = ((x + 1) % width, *y);
            if hcreatures.contains(&new_pos) || vcreatures.contains(&new_pos) {
                hcreatures_new.insert((*x, *y));
            } else {
                has_moved = true;
                hcreatures_new.insert(new_pos);
            }
        }

        for (x,y) in vcreatures.iter() {
            let new_pos = (*x, (y + 1) % height);
            if hcreatures_new.contains(&new_pos) || vcreatures.contains(&new_pos) {
                vcreatures_new.insert((*x, *y));
            } else {
                has_moved = true;
                vcreatures_new.insert(new_pos);
            }
        }

        //render_map(&hcreatures_new, &vcreatures_new, width, height);

        hcreatures = hcreatures_new;
        vcreatures = vcreatures_new;

        steps += 1;
        if !has_moved {
            break;
        }
    }

    println!("Stopped moving after {} moves", steps);
}
