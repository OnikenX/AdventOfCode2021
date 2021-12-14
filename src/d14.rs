use std::{collections::HashMap, hash::Hash, thread::current};

use crate::downloader::{download, get_token};

pub(crate) fn p1() {
    solution(10);
}

pub(crate) fn p2() {
    solution(40);
}

fn solution(steps : usize) {
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

    let mut pair_insertion_rules = vec![];
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let pair_insertion_rule = line.split(" -> ").collect::<Vec<_>>();
        if pair_insertion_rule.len() == 2 {
            pair_insertion_rules.push((
                pair_insertion_rule[0].to_string(),
                pair_insertion_rule[1].chars().next().unwrap(),
            ));
        } else {
            polymer_template = line.to_string();
        }
    }
    //rules: (inicial pair , result first pair, resulting second pair)
    let mut rules = HashMap::new();
    for rule in &pair_insertion_rules {
        let mut compiled = rule.0.clone();
        compiled.insert(1, rule.1.clone());
        let child1 = compiled[0..=1].to_string();
        let child2 = compiled[1..=2].to_string();
        rules.insert(rule.0.clone(), (child1, child2));
    }

    //all pairs possibles
    let mut pairs_counter = vec![];
    pair_insertion_rules.iter().map(|f| f.0.clone()).for_each(|s| {
        pairs_counter.push((s.clone(), 0usize));
    });
    //to used in the start of each step
    let mut empty_pairs_counter = pairs_counter.clone();
    //all chars possible
    let mut chars_counter = HashMap::new();

    for pair in pairs_counter.iter().map(|f| f.0.clone()) {
        chars_counter.insert(pair[0..1].chars().next().unwrap(), 0usize);
        chars_counter.insert(pair[1..2].chars().next().unwrap(), 0usize);
    }
    //read all chars to get its number
    let mut init_pair = polymer_template.len() - 2;
    for char in polymer_template.chars() {
        *chars_counter.get_mut(&char).unwrap() += 1;
    }

    //read inicial string to get its pairs
    loop {
        let pair = polymer_template[init_pair..(init_pair + 2)].to_string();
        // polymer_template.insert(init_pair + 1, *pair_insertion_rules.get(&pair).unwrap());
        pairs_counter.iter_mut().find(|f| f.0 == pair).unwrap().1 += 1;
        if init_pair == 0 {
            break;
        } else {
            init_pair -= 1;
        }
    }
    
    for _ in 1..=steps {
        let mut current_pairs_couter = empty_pairs_counter.clone();
        for pair in pairs_counter.iter_mut() {
            if pair.1 == 0 {
                continue;
            }
            let pairs_to_add = rules.iter().find(|f| *f.0 == pair.0).unwrap().1;
            let to_add = pair.1.clone();
            current_pairs_couter.iter_mut().find(|f| f.0 == pairs_to_add.0).unwrap().1 += to_add;
            current_pairs_couter.iter_mut().find(|f| f.0 == pairs_to_add.1).unwrap().1 += to_add;
            let char_c = pair_insertion_rules.iter().find(|f| f.0 == pair.0).unwrap().1;
            let c_count = chars_counter.get_mut(&char_c).unwrap();
            *c_count += to_add;
        }
        pairs_counter = current_pairs_couter;
    }
    println!(
        "response: {}",
        chars_counter.iter().map(|f| f.1).max().unwrap()
            - chars_counter.iter().map(|f| f.1).min().unwrap()
    );
}
