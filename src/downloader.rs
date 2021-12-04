use std::fs::read_to_string;

use curl::easy::Easy;

pub fn get_token() -> String {
    let token = read_to_string("token");
    if token.is_err() {
        println!("Don't have file `token` in the current directory ({})\n
        Using throw away account.", std::env::current_dir().unwrap().as_os_str().to_str().unwrap());
        "53616c7465645f5fd4bd15796f57e4ba457e3529d96ef6e4fd492d193f0bfdc7edb8f7d7c6f293e1b3d6c242c27a4c5c".to_string()
    }else{
        token.unwrap()
    }
    
    
}

pub fn download(year: i32, day: i32, token: &str) -> String {
    let mut data = Vec::<u8>::new();
    let mut handle = Easy::new();
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    handle.cookie(&format!("session={}", token)).unwrap();
    handle.url(&url).unwrap();
    {
        let mut transfer = handle.transfer();
        transfer
            .write_function(|new_data| {
                data.extend(new_data);
                Ok(new_data.len())
            })
            .expect("Can't define write function.");
        transfer.perform().expect("Can't make http request.");
    }
    let input = String::from_utf8(data).unwrap();
    if input.contains("login") {
        panic!("Bad login token.");
    }
    input
}


pub fn head(lines: &Vec<&str>, size : usize) {
    for i in 0..size {
        println!("{}", lines[i]);
    }
}