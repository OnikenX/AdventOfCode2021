use crate::downloader::{download, get_token};
use std::{collections::HashMap, panic, usize, vec};

pub(crate) fn p1() {
    solution(true);
}
pub(crate) fn p2() {
    solution(false);
}

fn solution(part1: bool) {
    let input = 
"2199943210
3987894921
9856789892
8767896789
9899965678"
        .to_string();
    let input = download(2021, 9, &get_token());
    let mut map = vec![];

    //get values in the map
    for line in input.lines() {
        let mut line_nums = vec![];
        line.chars()
            .for_each(|f| line_nums.push(f.to_string().parse::<u8>().unwrap()));
        map.push(line_nums);
    }

    //find lows
    let mut lows = HashMap::new();
    let line_max = map[0].len();
    let high_max = map.len();
    //index high
    for ihi in 0..high_max {
        //index line
        let mut iline = if ihi % 2 == 0 { 0usize } else { 1usize };
        //0 - center ; 1 - top ; 2 - left ; 3 - right ; 4 -bottom

        while iline < line_max {
            let mut ihi = ihi;
            let mut ili = iline;
            loop {
                let mut values_around = [10u8; 5];
                values_around[0] = map[ihi][ili];
                if ihi as i64 - 1 >= 0 {
                    //top
                    values_around[1] = map[ihi - 1][ili];
                }
                if ili as i64 - 1 >= 0 {
                    //left
                    values_around[2] = map[ihi][ili - 1];
                }
                if ili + 1 < line_max {
                    //right
                    values_around[3] = map[ihi][ili + 1];
                }
                if ihi + 1 < high_max {
                    //bottom
                    values_around[4] = map[ihi + 1][ili];
                }
                let mut values_iter = values_around.iter().enumerate();
                let mut lowest = (0usize, *values_iter.next().unwrap().1);
                while let Some(x) = values_iter.next() {
                    if *x.1 < lowest.1 {
                        lowest = (x.0, *x.1);
                    }
                }
                if lowest.0 == 0 {
                    if lowest.1 < 9 {
                        lows.insert((ihi, ili), lowest.1);
                    }
                    break;
                }
                match lowest.0 {
                    1 => ihi -= 1,
                    2 => ili -= 1,
                    3 => ili += 1,
                    4 => ihi += 1,
                    _ => panic!("Position not possible!"),
                }
            }
            iline += 2;
        }
    }
    let mut sum = 0u64;
    for low in &lows {
        sum += *low.1 as u64 + 1;
    }
    if part1 {
        println!("sum: {}", sum);
    } else {
        let mut top3 = [0usize; 3];

        for low in &lows {
            let mut basin = HashMap::new();
            explore_point(&map, &mut basin, low.0.clone());
            let min = top3.iter_mut().min().unwrap();

            if basin.len() > *min{
                *min = basin.len();
            }
        }
        let mut mult = 1usize;
        for item in top3{
            if item != 0 {
                mult *= item;
            }
        }
        println!("mult of {:?} = {}", &top3, &mult);
    }
}

fn explore_point(map: &Vec<Vec<u8>>, basin: &mut HashMap<(usize, usize), u8>, point: (usize, usize)) {
    if map[point.0][point.1] >= 9 {
        return;
    }
    if let None = basin.insert(point, map[point.0][point.1]) {
        if point.0 as i64 - 1 >= 0 {
            //top
            explore_point(map, basin, (point.0 - 1, point.1));
        }
        if point.1 as i64 - 1 >= 0 {
            //left
            explore_point(map, basin, (point.0, point.1-1));
        }
        if point.1 + 1 < map[0].len() {
            //right
            explore_point(map, basin, (point.0, point.1+1));
        }
        if point.0 + 1 < map.len() {
            //bottom
            explore_point(map, basin, (point.0 + 1, point.1));
        }
    }
}
