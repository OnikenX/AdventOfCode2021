use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    collections::{HashMap, HashSet},
    hash,
    path::Path,
    rc::Rc,
    vec,
};

use crate::downloader::{download, get_token};
type Points = HashSet<String>;
type Point = Rc<RefCell<Node>>;
type Map = HashMap<String, Points>;

const END: &str = "end";
const START: &str = "start";

trait MapFeactures {
    fn add_connections(&mut self, points: &Vec<&str>);
}

#[derive(Debug, Clone)]
struct Paths {
    pub start: Point,
}

impl Paths {
    fn new() -> Paths {
        let paths = Paths {
            start: Rc::new(RefCell::new(Node::new(START, None))),
        };
        (*paths.start).borrow_mut().self_ref = Some(paths.start.clone());
        paths
    }
}

#[derive(Debug, Clone)]
struct Node {
    pub name: String,
    pub nexts: Vec<Point>,
    pub self_ref: Option<Point>,
    pub prev_ref: Option<Point>,
}

impl Node {
    fn new(name: &str, prev: Option<Point>) -> Node {
        Node {
            name: name.to_string(),
            nexts: vec![],
            self_ref: None,
            prev_ref: prev,
        }
    }

    fn new_point(name: &str, prev: Option<Point>) -> Point {
        let rc_node = Rc::new(RefCell::new(Node::new(name, prev)));
        (*rc_node).borrow_mut().self_ref = Some(rc_node.clone());
        rc_node
    }
}

impl MapFeactures for Map {
    fn add_connections(&mut self, points: &Vec<&str>) {
        let mut add_direction = |from: &str, to: &str| {
            let set;
            if let Some(set_tmp) = self.get_mut(from) {
                set = set_tmp;
            } else {
                self.insert(from.to_string(), Points::new());
                set = self.get_mut(from).unwrap();
            }
            set.insert(to.to_string());
        };
        add_direction(points[0], points[1]);
        add_direction(points[1], points[0]);
    }
}

pub fn p1(){
    solution(true);
}pub fn p2(){
    solution(false);
}

fn solution(part1 : bool) {
    let input = "\
start-A
start-b
A-c
A-b
b-d
A-end
b-end
"
    .to_string();
    let input = download(2021, 12, &get_token());
    let mut map = Map::new();
    for line in input.lines() {
        let points = line.split('-').collect::<Vec<_>>();
        map.add_connections(&points);
    }
    find_paths(&map, part1);
}

fn find_paths(map: &Map, part1 : bool) {
    let start = map.get(START).unwrap();
    let paths: Paths = Paths::new();
    for ligacao in start {
        let next = Node::new_point(ligacao, Some(paths.start.clone()));
        (*paths.start).borrow_mut().nexts.push(next.clone());
        expand_path(map, next, part1);
    }
    println!("Possible paths: {}", verify_lines_completed(paths.start));
}

fn is_small_cave(name: &str) -> bool {
    if name.to_lowercase() == name {
        true
    } else {
        false
    }
}

fn expand_path(map: &Map, point: Point, passed_twice_small_cave: bool) {
    let this_name = point.as_ref().borrow().name.clone();
    if this_name == END {
        return;
    }
    let hashmap = map.get(&this_name).unwrap().clone();
    for cave in hashmap {
        let mut passed_twice_small_cave = passed_twice_small_cave;
        if cave == START {
            continue;
        }
        if is_small_cave(&cave) {
            if cave_has_visited_before(&cave, (*point).borrow().prev_ref.clone().unwrap()) {
                if passed_twice_small_cave {
                    continue;
                } else {
                    passed_twice_small_cave = true;
                }
            }
        }
        let next = Node::new_point(&cave, Some(point.clone()));
        (*point).borrow_mut().nexts.push(next.clone());
        if cave != END {
            expand_path(map, next, passed_twice_small_cave);
        }
    }
}

fn verify_lines_completed(point: Point) -> usize {
    if point.as_ref().borrow().name == END {
        // println!("{}", sequence_string(point.clone()));
        return 1;
    }
    let ref_nexts = &point.as_ref().borrow().nexts;
    if ref_nexts.is_empty() {
        // println!("{}", sequence_string(point.clone()));
        return 0;
    }
    let mut counter = 0;
    for path in ref_nexts {
        counter += verify_lines_completed(path.clone());
    }
    counter
}

fn sequence_string(point: Rc<RefCell<Node>>) -> String {
    if point.as_ref().borrow().name == START {
        String::from(START)
    } else {
        sequence_string(point.as_ref().borrow().prev_ref.clone().unwrap())
            + " -> "
            + &point.as_ref().borrow().name.clone()
    }
}

fn cave_has_visited_before(cave: &str, prev_point: Point) -> bool {
    let prev_name = prev_point.as_ref().borrow().name.clone();
    if prev_name == cave {
        return true;
    }
    if prev_name == START {
        return false;
    }
    return cave_has_visited_before(cave, prev_point.as_ref().borrow().prev_ref.clone().unwrap());
}
