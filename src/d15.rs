use std::{collections::VecDeque, rc::Rc};

use ansi_term::{Color, Style};

const SAMPLE: &str = "
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
";

pub fn p1(input: &String) {
    solution(true, input);
}

type RcNode = Rc<Node>;
struct Node {
    this_coord: (usize, usize), //y,x
    prev_rc: Option<RcNode>,
    accumolado: u32,
}
impl Node {
    fn new(this_coord: (usize, usize), prev_rc: Option<RcNode>, accumolado: u32) -> RcNode {
        let node = Node {
            this_coord,
            prev_rc: prev_rc,
            accumolado,
        };
        Rc::new(node)
    }
}

fn solution(part1: bool, input: &String) {
    let mut map = vec![];
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        map.push(
            line.chars()
                .map(|c| c.to_string().parse::<u32>().unwrap())
                .collect::<Vec<_>>(),
        );
    }

    let mut lowest_risk: u32;
    {
        let line: u32 = map[0].iter().sum();
        let mut acc = 0;
        for i in 1..map.len() {
            acc += map[i][map[0].len() - 1];
        }
        lowest_risk = line + acc;
    }

    let process_point =
        |paths: &mut VecDeque<RcNode>, this_node: RcNode, coords_yx: (usize, usize)| {
            if this_node.prev_rc.is_some() {
                if verify_repetition(this_node.prev_rc.clone().unwrap(), coords_yx.clone()) {
                    return;
                }
            }
            let yx = this_node.this_coord;
            paths.push_back(Node::new(
                coords_yx,
                Some(this_node.clone()),
                this_node.accumolado + map[yx.0][yx.1],
            ));
        };

    let mut paths = VecDeque::new();
    paths.push_back(Node::new((0, 0), None, 0));
    while !paths.is_empty() {
        println!("entrou: {}", &paths.len());
        let actual = paths.pop_front().unwrap();
        let (y, x) = actual.this_coord.clone();

        //final position
        if y == map.len() - 1 && x == map[0].len() - 1 {
            lowest_risk = lowest_risk.min(actual.accumolado);
            // print_path(&map, actual.clone());
            println!("encontrou um fim");
        } else {
            if y + 1 < map.len() {
                //bottom
                process_point(&mut paths, actual.clone(), (y + 1, x));
                // if x + 1 < map[0].len() {
                //     //bottom right
                //     process_point(&mut paths, actual.clone(), (y + 1, x + 1));
                // }
                // if x as i64 - 1 >= 0 {
                //     //bottom left
                //     process_point(&mut paths, actual.clone(), (y + 1, x - 1));
                // }
            }

            // if y as i64 - 1 >= 0 {
            //     //top
            //     process_point(&mut paths, actual.clone(), (y - 1, x));
            //     // if x + 1 < map[0].len() {
            //     //     //top right
            //     //     process_point(&mut paths, actual.clone(), (y - 1, x + 1));
            //     // }
            //     // if x as i64 - 1 >= 0 {
            //     //     //top left
            //     //     process_point(&mut paths, actual.clone(), (y - 1, x - 1));
            //     // }
            // }

            if x as i64 - 1 >= 0 {
                //left
                // process_point(&mut paths, actual.clone(), (y, x - 1));
            }
            if x + 1 < map[0].len() {
                //right
                process_point(&mut paths, actual.clone(), (y, x + 1));
            }
        }
    }
    println!("accumulado: {}", lowest_risk);
}
fn verify_repetition(prev_rc: RcNode, coords_yx: (usize, usize)) -> bool {
    if prev_rc.this_coord == coords_yx {
        true
    } else if prev_rc.prev_rc.is_none() {
        false
    } else {
        verify_repetition(prev_rc.prev_rc.clone().unwrap(), coords_yx)
    }
}
fn get_path_vec(prev_rc: RcNode) -> Vec<(usize, usize)> {
    if prev_rc.prev_rc.is_none() {
        vec![prev_rc.this_coord.clone()]
    } else {
        let mut vec = get_path_vec(prev_rc.prev_rc.clone().unwrap());
        vec.push(prev_rc.this_coord.clone());
        vec
    }
}

fn print_path(map: &Vec<Vec<u32>>, final_rc: RcNode) {
    let coords = get_path_vec(final_rc.prev_rc.clone().unwrap());
    let paint = Style::new().bold().fg(Color::RGB(155, 0, 155));
    for (y, line) in map.iter().enumerate() {
        for (x, item) in line.iter().enumerate() {
            if coords.iter().any(|f| (y, x) == *f) {
                print!("{}", paint.paint(item.to_string()));
            } else {
                print!("{}", item);
            }
        }
        println!()
    }
}
