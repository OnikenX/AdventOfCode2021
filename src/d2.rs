use std::{fs::read_to_string, panic};

pub fn p1(input : String) {
    let mut depth = 0;
    let mut horizontal = 0;

    let get_num = |value: &str| value.parse::<i32>().expect("cant convert token to int");

    for line in input.lines() {
        let tokens = line.split(" ").collect::<Vec<_>>();
        if tokens.len() != 2 {
            panic!("wrong size of tokens.");
        }

        match tokens[0] {
            "forward" => {
                horizontal += get_num(tokens[1]);
            }
            "down" => {
                depth += get_num(tokens[1]);
            }
            "up" => {
                depth -= get_num(tokens[1]);
            }
            _ => {
                panic!("Undefined '{}' token", tokens[0]);
            }
        }
    }
    println!("Final value: {}", depth * horizontal);
}

pub fn p2(input : String) {
    let mut depth = 0;
    let mut aim = 0;
    let mut horizontal = 0;

    let get_num = |value: &str| value.parse::<i32>().expect("cant convert token to int");

    for line in input.lines() {
        let tokens = line.split(" ").collect::<Vec<_>>();
        if tokens.len() != 2 {
            panic!("wrong size of tokens.");
        }

        match tokens[0] {
            "forward" => {
                let actual_horizontal = get_num(tokens[1]);
                horizontal += actual_horizontal;
                depth += actual_horizontal * aim;
            }
            "down" => {
                aim += get_num(tokens[1]);
            }
            "up" => {
                aim -= get_num(tokens[1]);
            }
            _ => {
                panic!("Undefined '{}' token", tokens[0]);
            }
        }
    }
    println!("Final value: {}", depth * horizontal);
}
