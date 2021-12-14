use std::{collections::HashMap, hash::Hash};

use crate::downloader::{download, get_token};

pub(crate) fn p1() {
    solution(true);
}

pub(crate) fn p2() {
    solution(false);
}

fn solution(part1: bool) {
    let input = "\
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
    let input = download(2021, 14, &get_token());
    let mut polymer_template = "".to_string();

    let mut pair_insertion_rules = HashMap::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let pair_insertion_rule = line.split(" -> ").collect::<Vec<_>>();
        if pair_insertion_rule.len() == 2 {
            pair_insertion_rules.insert(
                pair_insertion_rule[0].to_string(),
                pair_insertion_rule[1].chars().next().unwrap(),
            );
        } else {
            polymer_template = line.to_string();
        }
    }
    let steps = 20;
    for step in 1..=steps {
        let mut init_pair = polymer_template.len() - 2;
        loop {
            let pair = polymer_template[init_pair..(init_pair + 2)].to_string();
            // println!("{}", pair);
            polymer_template.insert(init_pair + 1, *pair_insertion_rules.get(&pair).unwrap());
            if init_pair == 0 {
                break;
            } else {
                init_pair -= 1;
            }
        }
        println!("{}, len:{}", step, polymer_template.len());
    }
    //0 - B, 1 - C, 2 - H, 3 -N
    let mut counters = HashMap::new();

    for char in polymer_template.chars() {
        if let Some(counter) = counters.get_mut(&char){
            *counter += 1;
        }else {
            counters.insert(char, 1usize);
        }
    }

    println!(
        "response: {}",
        counters.iter().map(|f| f.1).max().unwrap() - counters.iter().map(|f| f.1).min().unwrap()
    );
}
