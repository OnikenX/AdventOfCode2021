use std::collections::HashSet;

use crate::downloader::{download, get_token};

pub fn p1() {
    solution(true);
}
pub fn p2() {
    solution(false);
}

pub fn solution(part1: bool) {
    const X: usize = 1;
    const Y: usize = 0;
    let input = "\
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
"
    .to_string();
    let input = download(2021, 13, &get_token());

    //[y,x]
    let mut coords = HashSet::new();
    let parser = |str: &str| str.parse::<usize>().unwrap();

    //fonding code
    let folding = |coords: &mut HashSet<[usize; 2]>, fold, idx| {
        let max_index = fold * 2;
        let mut coords_to_fold = vec![];
        let mut coords_to_remove = vec![];
        coords.iter().for_each(|coord_r| {
            if coord_r[idx] > fold {
                coords_to_fold.push(coord_r.clone());
            } else if coord_r[idx] == fold {
                coords_to_remove.push(coord_r.clone());
            }
        });
        while let Some(coord) = coords_to_fold.pop() {
            let mut coord = coord;
            coords.remove(&coord);
            coord[idx] = (coord[idx] as i64 - max_index as i64).abs() as usize;
            coords.insert(coord);
        }
        while let Some(coord) = coords_to_remove.pop() {
             coords.remove(&coord);
        }
    };
    let mut first_time = true;

    //reading input
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let coord = line.split(',').collect::<Vec<_>>();
        //reading coords
        if coord.len() == 2 {
            //in the file the order is x,y, in the coords its y,x
            let x = parser(coord[0]);
            let y = parser(coord[1]);
            coords.insert([y, x]);
        } else {
            if part1 {
                if first_time {
                    first_time = false;
                } else {
                    break;
                }
            }
            //readding the folds
            let folding_line = line.split('=').collect::<Vec<_>>();
            let fold = parser(folding_line[1]);

            if folding_line[0].contains('y') {
                folding(&mut coords, fold, Y);
                println!();
            } else if folding_line[0].contains('x') {
                folding(&mut coords, fold, X);
                println!();
            } else {
                panic!("Unexpected line: `{}`", line);
            }
        }
    }
    draw_table(&coords);
    println!("{}", coords.len());
}

fn draw_table(coords: &HashSet<[usize; 2]>) {
    let mut max_x = 0usize;
    let mut max_y = 0usize;
    coords.iter().for_each(|i| {
        max_y = max_y.max(i[0]);
        max_x = max_x.max(i[1]);
    });
    let mut table = vec![vec!['.'; max_x + 1]; max_y + 1];
    for coord in coords {
        table[coord[0]][coord[1]] = '#';
    }
    for line in table {
        let mut string = String::new();
        line.iter().for_each(|ch| string.push(*ch));
        println!("{}", string);
    }
}
