use std::{fs::read_to_string, collections::VecDeque};

pub fn read_file() -> String {
    read_to_string("d1").expect("there is no file")
}

pub fn p1() {
    let d1 = read_file();
    let mut last_line: i32 = -1;
    let mut current_line: i32;
    let mut increase_counter: usize = 0;
    for line in d1.lines() {
        current_line = line.parse::<i32>().unwrap();
        if last_line != -1 {
            if current_line > last_line {
                increase_counter += 1;
            }
        }
        last_line = current_line;
    }
    println!("Number of lines increased:\n{}", increase_counter);
}

#[derive(Clone, Debug)]
struct Group {
    pub valor: i32,
    pub counter: i32,
}

impl Group {
    pub fn new() -> Group {
        Group {
            valor: 0,
            counter: 0,
        }
    }
    pub fn increment(&mut self, amount: i32) -> bool {
        if self.counter == 3 {
            false
        } else {
            self.valor += amount;
            self.counter += 1;
            true
        }
    }
}

pub fn p2() {
    let d1 = read_file();
    let mut groups = VecDeque::<Group>::new();
    groups.reserve(4);
    let mut increase_counter: usize = 0;
    let mut last_group_value = -1;
    let compare_values = |actual_group : &Group, increase_counter : &mut usize, last_group_value : &mut i32|
    {if *last_group_value != -1 {
        if *last_group_value < actual_group.valor {
            *increase_counter += 1;
        }
    }};
    for line in d1.lines() {
        let current_line = line.parse::<i32>().unwrap();
      
        groups.push_back(Group::new());
        let mut remove = false;
        for actual_group in &mut groups {
            if !actual_group.increment(current_line) {
                compare_values(actual_group,&mut increase_counter, &mut last_group_value);
                last_group_value = actual_group.valor;
                remove  = true;
            };
        }

        if remove {
            let _ = groups.pop_front();
        }
    }

    if let Some(group) = groups.get(0){
        if group.counter == 3 {
        compare_values(group,&mut increase_counter,&mut last_group_value);}
    }

    println!("Number of lines increased: {}", increase_counter);
}
