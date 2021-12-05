pub fn p1(input: String) {
    geral(input, true);
}

pub fn p2(input: String) {
    geral(input,false);
}
fn geral(input: String, is_p1 : bool) {
    let (coordenadas, coordenadas_verticais, max_xy) = process_lines(input, is_p1);
    let mut map = vec![vec![0; (max_xy[0] + 1) as usize]; (max_xy[1] + 1) as usize];
    //horizontal/vertical processing
    for coord in coordenadas {
        let biggest_diff = {
            let diff_x = (coord[0][0] - coord[1][0]).abs();
            let diff_y = (coord[0][1] - coord[1][1]).abs();
            if diff_x > diff_y {
                0
            } else {
                1
            }
        };

        let mut i = coord[0][biggest_diff];
        loop {
            //draw map
            if biggest_diff == 0 {
                map[coord[0][1] as usize][i as usize] += 1;
            } else {
                map[i as usize][coord[0][0] as usize] += 1;
            }

            //loop stuff
            if i == coord[1][biggest_diff] {
                break;
            }
            if coord[0][biggest_diff] < coord[1][biggest_diff] {
                i += 1;
            }
            if coord[0][biggest_diff] > coord[1][biggest_diff] {
                i -= 1;
            }
        }
    }

    //diagonal processing
    for coord in &coordenadas_verticais {
        let lowest_diff = {
            let diff_x = (coord[0][0] - coord[1][0]).abs();
            let diff_y = (coord[0][1] - coord[1][1]).abs();
            if diff_x > diff_y {
                1
            } else {
                0
            }
        };
        let biggest_diff = if lowest_diff == 1 { 0 } else { 1 };
        let mut i = coord[0][lowest_diff];
        let mut f = coord[0][biggest_diff];
        loop {
            //draw map
            if lowest_diff == 0 {
                map[f as usize][i as usize] += 1;
            } else {
                map[i as usize][f as usize] += 1;
            }

            //loop stuff
            if i == coord[1][lowest_diff] {
                break;
            }
            if coord[0][lowest_diff] < coord[1][lowest_diff] {
                i += 1;
            }
            if coord[0][lowest_diff] > coord[1][lowest_diff] {
                i -= 1;
            }
            if coord[0][biggest_diff] < coord[1][biggest_diff] {
                f += 1;
            }
            if coord[0][biggest_diff] > coord[1][biggest_diff] {
                f -= 1;
            }
        }
    }
    let mut counter = 0;
    for map_line in &map {
        //count the stuff
        for map_item in map_line {
            if *map_item > 1 {
                counter += 1;
            }
        }
    }
    println!("blocks with more than 1 line: {}", counter);
}

fn process_lines(
    input: String,
    only_vert_horiz: bool,
) -> (Vec<[[i32; 2]; 2]>, Vec<[[i32; 2]; 2]>, [i32; 2]) {
    let mut coordenadas = vec![];
    let mut coordenadas_verticais = vec![];
    let mut max_xy = [0, 0];
    for line in input.lines() {
        let cords = line.split(" -> ").collect::<Vec<_>>();
        if cords.len() != 2 {
            panic!("Wrong size, there needs to be 2 pairs of coordinates");
        }
        let first = cords[0].split(",").collect::<Vec<_>>();
        let last = cords[1].split(",").collect::<Vec<_>>();
        let new = [
            [
                first[0].parse::<i32>().unwrap(),
                first[1].parse::<i32>().unwrap(),
            ],
            [
                last[0].parse::<i32>().unwrap(),
                last[1].parse::<i32>().unwrap(),
            ],
        ];
        let mut get_maxes = || {
            max_xy[0] = max_xy[0].max(new[0][0]).max(new[1][0]);
            max_xy[1] = max_xy[1].max(new[0][1]).max(new[1][1]);
        };
        if !(!(new[0][0] == new[1][0] || new[0][1] == new[1][1])) {
            get_maxes();
            coordenadas.push(new);
        } else if !only_vert_horiz {
            get_maxes();
            coordenadas_verticais.push(new);
        }
    }
    (coordenadas, coordenadas_verticais, max_xy)
}
