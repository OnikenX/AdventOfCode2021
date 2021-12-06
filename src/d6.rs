use std::os::unix::prelude::FileExt;

use crate::downloader::{self, get_token};

// tempo interno de 3
// depois de um dia seria 2
// depois de um dia seria 1
// depois de um dia seria 0
// depois de um dia seria 6 e cria um novo laternfish com o counter a 8
//fish 1 - 5 / fish 2 - 7

pub fn p1() {
    // optimized();
    added_new_fish(14);
    return;
    // let input = downloader::download(2021, 6, &get_token());
    let input = "3,4,3,1,2".to_string();

    println!("{}", input);
    let mut lanternfish = vec![];
    for x in input.split(",") {
        let x = x.trim();
        lanternfish.push(x.parse::<i32>().unwrap());
    }
    let days = 18;

    for day in 1..=days {
        let mut to_add = 0;
        for fish in &mut lanternfish {
            if *fish == 0 {
                to_add += 1;
                *fish = 6;
            } else {
                *fish -= 1;
            }
        }
        while to_add != 0 {
            lanternfish.push(8);
            to_add -= 1;
        }
        println!("day: {}; size: {}; {}%", &day, lanternfish.len(), (day as f32/days as f32)*100.0f32);
    }
    println!("{}", lanternfish.len());
}


pub fn optimized(){
    let input = "3,4,3,1,2".to_string();
    println!("{}", input);
    let mut lanternfish = vec![];
    for x in input.split(",") {
        let x = x.trim();
        lanternfish.push(x.parse::<i32>().unwrap());
    }
    let days  = 256;
    let n_lanternfish = 0u128;
    for fish in &mut lanternfish{
        *fish = *fish-days;
        println!("{} -> {}",(*fish), ((*fish)/6).abs());
        
    }
    

    //3,4,3,1,2
    // depois de 2
    //3-2=1,4-2=2,3-2=1,1-2=-1,2-2=0
    //1,2,1,-1,0
    //1,2,1,6-1=5,1,2,8

    //depois de 18
    //3-18, 4-18,

    let x = 26984457539u128;
}


fn added_new_fish(days_remaning : i32) -> u128{
    //after the criation of a new one, after 14 days it creates an pair
    //these new ones create new ones in doubles
    let mut y = 0;
    let x = 1;
    for day in 1..=days_remaning{
        y = if day <= 8{
             0
        }else if day > 8 && day <= 14{
            1
        }else{
            x/6 + 1
        };
    }
    println!("{:?}", y);
    return y;
}