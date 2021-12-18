use std::{collections::VecDeque, rc::Rc};

use ansi_term::{Color, Style};

// The cavern is large, but has a very low ceiling, restricting your motion to two dimensions. The shape of the cavern resembles a square; a quick scan of chiton density produces a map of risk level throughout the cave (your puzzle input). For example:
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

// The total risk of this path is 40 (the starting position is never entered, so its risk is not counted).

// What is the lowest total risk of any path from the top left to the bottom right?

pub fn p1(input: &String) {
    solution(true, input /* &SAMPLE.to_string() */);
    // test();
}pub fn p2(input: &String) {
    solution(false, input /* &SAMPLE.to_string() */);
    // test();
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
    // let input = "8".to_string();
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
    if !part1 {
        let inicial_max_x = map[0].len();
        let inicial_max_y = map.len();

        for y in 0..inicial_max_y {
            for rep in 1..5u32 {
                for x in 0..inicial_max_x {
                    let inicial_value = map[y][x];
                    let mut to_replace = inicial_value + rep;
                    if to_replace > 9 {
                        to_replace -= 9;
                    }
                    map[y].push(to_replace);
                }
            }
        }
        for rep in 1..5u32 {
            for y in 0..inicial_max_y {
                let mut line_copy = map[y].clone();
                for item in &mut line_copy {
                    *item += rep;
                    if *item > 9 {
                        *item -= 9;
                    }
                }
                map.push(line_copy);
            }
        }
    }

    print_path(&map, Node::new((0, 0), None, 0));
    let heuristic_map = crate_heuristic_map(&map);
    let mut lowest_risk = vec![
        vec![
            heuristic_map
                .iter()
                .map(|m| m.iter().max().unwrap().clone())
                .max()
                .unwrap()
                .clone();
            map[0].len()
        ];
        map.len()
    ];

    let process_point = |paths: &mut VecDeque<RcNode>,
                         lowest_risk: &mut Vec<Vec<u32>>,
                         this_node: RcNode,
                         yx: (usize, usize)| {
        if this_node.prev_rc.is_some() {
            if verify_repetition_once(this_node.prev_rc.clone().unwrap(), yx.clone()) {
                return;
            }
        }

        let actual_acc = this_node.accumolado + map[yx.0][yx.1];
        if lowest_risk[yx.0][yx.1] < actual_acc {
            return;
        } else if lowest_risk[yx.0][yx.1] > actual_acc {
            lowest_risk[yx.0][yx.1] = actual_acc;
        }

        let node = Node::new(yx, Some(this_node.clone()), actual_acc);
        // print_path(&map, node.clone());
        // println!();
        paths.push_back(node);
    };

    let mut paths = VecDeque::new();
    let mut lowest_path_node = Node::new((0, 0), None, 0);
    paths.push_back(lowest_path_node.clone());
    while let Some(actual) = paths.pop_front() {
        let (y, x) = actual.this_coord.clone();
        if actual.accumolado > lowest_risk[y][x] {
            continue;
        }
        if y == map.len() - 1 && x == map[0].len() - 1 {
            //final position
            if lowest_risk[map.len() - 1][map[0].len() - 1] >= actual.accumolado {
                lowest_path_node = actual;
            }
        } else {
            if y + 1 < map.len() {
                //bottom
                process_point(&mut paths, &mut lowest_risk, actual.clone(), (y + 1, x));
            }
            if (x as i64 - 1) >= 0 {
                //left
                process_point(&mut paths, &mut lowest_risk, actual.clone(), (y, x - 1));
            }

            if x + 1 < map[0].len() {
                //right
                process_point(&mut paths, &mut lowest_risk, actual.clone(), (y, x + 1));
            }
        }
    }
    println!("accumulado: {}", lowest_path_node.accumolado);
    print_path(&map, lowest_path_node);
}

fn verify_repetition_once(prev_rc: RcNode, coords_yx: (usize, usize)) -> bool {
    if prev_rc.this_coord.0 == coords_yx.0 && prev_rc.this_coord.1 == coords_yx.1 {
        true
    } else {
        false
    }
}

fn verify_repetition(prev_rc: RcNode, coords_yx: (usize, usize)) -> bool {
    if prev_rc.this_coord.0 == coords_yx.0 && prev_rc.this_coord.1 == coords_yx.1 {
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
    let coords = get_path_vec(final_rc.clone());
    // println!("{:?}", coords);
    let paint = Style::new().bold().fg(Color::RGB(155, 0, 155));
    let mut buffer = String::new();
    for (y, line) in map.iter().enumerate() {
        for (x, item) in line.iter().enumerate() {
            if coords.iter().any(|f| (y, x) == *f) {
                buffer += &format!("{}", paint.paint(item.to_string()));
            } else {
                buffer += &format!("{}", item);
            }
        }
        buffer += "\n";
    }
    println!("{}", buffer);
}

fn test() {
    let node = Node::new((0, 0), None, 0);
    let node = Node::new((0, 1), Some(node), 0);
    let node = Node::new((0, 2), Some(node), 0);
    let node = Node::new((0, 3), Some(node), 0);
    println!("{}", verify_repetition(node, (0, 1)));
}

fn crate_heuristic_map(map: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut heuristic_map = vec![];
    for y in 0..map.len() {
        let mut heuristic_line = vec![];
        for x in 0..map[0].len() {
            heuristic_line.push(get_diagonal_heuristic(map, (y, x)));
        }
        heuristic_map.push(heuristic_line);
    }

    // let mut buffer = String::new();
    // for line in &heuristic_map{
    //     for item in line{
    //         buffer += &format!("{:4}", item);
    //     }
    //     buffer += "\n";
    // }

    heuristic_map
}

fn get_diagonal_heuristic(map: &Vec<Vec<u32>>, start: (usize, usize)) -> u32 {
    let mut yx = start;
    let mut acc = 0u32;
    loop {
        if yx.0 < map.len() - 1 {
            yx.0 += 1;
            acc += map[yx.0][yx.1];
        }
        if yx.1 < map[0].len() - 1 {
            yx.1 += 1;
            acc += map[yx.0][yx.1];
        }

        if !(yx.0 < map.len() - 1 && yx.1 < map[0].len() - 1) {
            break;
        }
    }
    acc
}
