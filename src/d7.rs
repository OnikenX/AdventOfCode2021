use crate::downloader::{self, get_token};

pub fn p1(){
    solution(true);
}
pub fn p2(){
    solution(false);
}


pub fn solution(is_p1: bool) {
    // let input = "16,1,2,0,4,2,7,1,2,14".to_string();
    let input = downloader::download(2021, 7, &get_token());
    let mut values = vec![];
    input.split(',').for_each(|x| {
        values.push(x.trim().parse::<i32>().unwrap());
    });

    values.sort();

    let calculate_cost =|values: &Vec<i32>,to_move: i32|{
        if is_p1 {
            calculate_cost_p1(&values, to_move)
        } else {
             calculate_cost_p2(&values, to_move)
        }
    };
    
    let mut lower = *values.iter().min().unwrap();
    let mut higher = *values.iter().max().unwrap();

    let mut center = values[values.len() / 2];
    let mut center_custo = calculate_cost(&values, center);
    // println!("{}", center_custo);
    let mut lower_cost = calculate_cost(&values, center - 1);
    let mut higher_cost = calculate_cost(&values, center + 1);
    println! {"lower:{}->{}; center:{}->{}; higher:{}->{}",lower, lower_cost, center, center_custo, higher, higher_cost};

    while lower_cost < center_custo || higher_cost < center_custo {
        if lower_cost < higher_cost {
            higher = center - 1;
        } else if lower_cost > higher_cost {
            lower = center + 1;
        }
        center = ((higher - lower) / 2) + lower;
        lower_cost = calculate_cost(&values, center - 1);
        higher_cost = calculate_cost(&values, center + 1);
        center_custo = calculate_cost(&values, center);
        println! {"lower:{}->{}; center:{}->{}; higher:{}->{}",lower, lower_cost, center, center_custo, higher, higher_cost};
    }
}

// fn higher_half(higher, lower : i32);

fn calculate_cost_p1(values: &Vec<i32>, to_move: i32) -> u64 {
    let mut cost = 0;
    for value in values {
        cost += (value - to_move).abs() as u64;
    }
    cost
}

//with factorial
fn calculate_cost_p2(values: &Vec<i32>, to_move: i32) -> u64 {
    let mut cost = 0;
    for value in values {
        cost += ((value - to_move).abs() as u64).factorial();
    }
    cost
}

trait factorial {
    fn factorial(&self) -> Self;
}

impl factorial for u64 {
    fn factorial(&self) -> u64 {
        let mut fact = 0u64;
        for i in (1..=*self).rev() {
            fact += i;
        }
        fact
    }
}
