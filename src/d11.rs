use ansi_term::{Color, Style};

use crate::downloader::{download, get_token};

fn solution(part1: bool) {
    // let input = INPUT.to_string();
    let input = download(2021, 11, &get_token());
    let mut map = vec![];
    input.lines().for_each(|f| {
        let mut line = vec![];
        f.chars().for_each(|f| {
            line.push(f.to_string().parse::<u8>().unwrap());
        });
        map.push(line);
    });
    let mut flash_counter = 0u32;
    let mut iter = 0;
    loop {
        iter += 1;
        //increments all
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                map[y][x] += 1;
            }
        }

        for y in 0..map.len() {
            for x in 0..map[0].len() {
                if map[y][x] >= 10 {
                    flash_it(&mut map, &mut flash_counter, y, x);
                }
            }
        }

        if part1 {
            if iter == 100 {
                break;
            }
        } else {
            let mut zero_counter = 0usize;
            map.iter().for_each(|f| {
                f.iter().for_each(|f| {
                    if *f == 0 {
                        zero_counter += 1;
                    }
                })
            });
            if zero_counter == map[0].len() * map.len() {
                println!("Iter: {}", iter);
                break;
            }
        }
    }
    println!("Flashes: {}", flash_counter);
}

fn flash_it(map: &mut Vec<Vec<u8>>, flash_counter: &mut u32, y: usize, x: usize) {
    map[y][x] = 0;
    *flash_counter += 1;
    let mut process_point = |map: &mut Vec<Vec<u8>>, y: usize, x: usize| {
        if map[y][x] != 0 {
            map[y][x] += 1;
            if map[y][x] >= 10 {
                flash_it(map, flash_counter, y, x);
            }
        }
    };

    if y + 1 < map.len() {
        //bottom
        process_point(map, y + 1, x);
        if x + 1 < map[0].len() {
            //bottom right
            process_point(map, y + 1, x + 1);
        }
        if x as i64 - 1 >= 0 {
            //bottom left
            process_point(map, y + 1, x - 1);
        }
    }

    if y as i64 - 1 >= 0 {
        //top
        process_point(map, y - 1, x);
        if x + 1 < map[0].len() {
            //top right
            process_point(map, y - 1, x + 1);
        }
        if x as i64 - 1 >= 0 {
            //top left
            process_point(map, y - 1, x - 1);
        }
    }

    if x as i64 - 1 >= 0 {
        //left
        process_point(map, y, x - 1);
    }
    if x + 1 < map[0].len() {
        //right
        process_point(map, y, x + 1);
    }
}

fn print_map(map: &Vec<Vec<u8>>) {
    let bold = Style::new().bold().fg(Color::RGB(155, 0, 155));
    map.iter().for_each(|f| {
        f.iter().for_each(|item| {
            if *item == 0 {
                print!("{}", bold.paint(item.to_string()))
            } else {
                print!("{}", &item);
            }
        });
        print!("\n");
    });
}

const INPUT: &str = "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

pub fn p1() {
    solution(true);
}
pub fn p2() {
    solution(false);
}
