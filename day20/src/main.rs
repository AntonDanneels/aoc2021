use std::collections::HashSet;

#[derive(Debug)]
struct Image {
    set_pixels: HashSet<(i32, i32)>,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

fn render_image(img: &Image) {
    for y in img.y - 1..img.h + 1 {
        for x in img.x - 1..img.w + 1 {
            if img.set_pixels.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!(".");
    }
}

fn enhance_img(input_img: &Image, filter: &Vec<u32>, step: u32) -> Image {
    let mut result_img = Image {
        set_pixels: HashSet::new(),
        x: input_img.x - 1,
        y: input_img.y - 1,
        w: input_img.w + 1,
        h: input_img.h + 1,
    };

    for y in result_img.y..result_img.h + 1 {
        for x in result_img.x..result_img.w + 1 {
            let mut index_in_filter = 0;
            let mut index = 0;
            for yy in -1..2 {
                for xx in -1..2 {
                    let px = x + xx;
                    let py = y + yy;
                    /* Input image is set if the previous image has a lit pixel */
                    if input_img.set_pixels.contains(&(px, py)) ||
                    /* Or if the default value of the filter is 1 and the queried pixel is outside the previous
                     * image */
                     (filter[0] == 1 && step % 2 == 0 && !(px >= input_img.x && px <= input_img.w &&
                                                           py >= input_img.y && py <= input_img.h ))
                    {
                        index_in_filter |= 1 << (8 - index);
                    }
                    index += 1;
                }
            }

            if filter[index_in_filter] == 1 {
                result_img.set_pixels.insert((x, y));
            }
        }
    }

    result_img
}

fn main() {
    //let mut data = include_str!("sample.txt").lines();
    let mut data = include_str!("day20.txt").lines();

    let filter = data
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            '#' => 1,
            '.' => 0,
            _ => panic!("Bad input file"),
        })
        .collect::<Vec<u32>>();

    //skip empty line
    data.next().unwrap();

    let mut input_img = Image {
        set_pixels: HashSet::new(),
        x: 0,
        y: 0,
        w: 0,
        h: 0,
    };
    for (y, line) in data.enumerate() {
        input_img.h = y as i32 + 1;
        for (x, c) in line.chars().enumerate() {
            input_img.w = x as i32 + 1;
            match c {
                '#' => {
                    input_img.set_pixels.insert((x as i32, y as i32));
                }
                _ => {}
            }
        }
    }

    //render_image(&input_img);

    for i in 0..50 {
        input_img = enhance_img(&input_img, &filter, i + 1);
    }

    println!("{}", input_img.set_pixels.len());
}
