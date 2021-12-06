use std::{os::unix::prelude::FileExt, fs::read_to_string};

use crate::downloader::{self, get_token};

// tempo interno de 3
// depois de um dia seria 2
// depois de um dia seria 1
// depois de um dia seria 0
// depois de um dia seria 6 e cria um novo laternfish com o counter a 8
//fish 1 - 5 / fish 2 - 7

pub fn solver(days : i32){
    let input = 
    downloader::download(2021, 6, &get_token());
    //0/1/2/3/4/5/6/7/8
    let mut state = [0usize; 9];
    for item in input.split(',') {
        let x = item.trim();
        let index = x.parse::<i32>().unwrap();
        state[index as usize] += 1;
    }
    let mut prev = state[8];
    let mut actual = 0;
    for day in 1..=days{
        for index in (0..state.len()-1).rev(){
            actual = state[index];
            state[index] = prev;
            prev=actual;
        }
        state[6] += actual;
        state[8] = actual;
    }
    println!("{:?} - {}", state, state.iter().sum::<usize>());
}
