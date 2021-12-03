use std::{char};

fn head(lines: &Vec<&str>) {
    for i in 0..5 {
        println!("{}", lines[i]);
    }
}

pub fn p1(input: String) {
    //     let input = "00100
    // 11110
    // 10110
    // 10111
    // 10101
    // 01111
    // 00111
    // 11100
    // 10000
    // 11001
    // 00010
    // 01010"
    //         .to_string();

    let lines = input.lines().collect::<Vec<_>>();
    println!("lines.len(): {}", lines.len());
    head(&lines);
    let one_zero = "10".chars().collect::<Vec<_>>();
    let one = one_zero[0];
    let zero = one_zero[1];
    let mut digits = vec![0; lines[0].len()];
    for line in &lines {
        if line.is_ascii() {
            for (char, digit) in line.chars().zip(digits.iter_mut()) {
                if char == one {
                    *digit += 1
                } else if char != zero {
                    panic!("Invalid input, needs to be `1` or `0`.");
                }
            }
        } else {
            panic!("Invalid input, needs to be only ascii chars.");
        }
    }
    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    digits.reverse();
    let digits = digits;
    for (pos, digit) in digits.iter().enumerate() {
        if *digit > (&lines.len() - *digit) {
            gamma_rate |= 2u32.pow(pos as u32);
        } else {
            epsilon_rate |= 2u32.pow(pos as u32);
        }
    }

    println!(
        "gamma_rate: {0:b}/{0}; epsilon_rate: {1:b}/{1}; answer: {2}",
        gamma_rate,
        epsilon_rate,
        gamma_rate * epsilon_rate
    );
}

pub fn p2(input: String) {
    let lines = input.lines().collect::<Vec<_>>();
    head(&lines);
    let mut oxygen_bins = vec![];
    let mut co2_bins = vec![];
    let one_zero = "10".chars().collect::<Vec<_>>();
    let one = one_zero[0];
    let zero = one_zero[1];
    let line_size = lines[0].len();
    let mut digits = vec![0; line_size];
    for line in &lines {
        if !line.is_ascii() {
            panic!("Invalid input, needs to be only ascii chars.");
        }
        for char in line.chars() {
            if char != one && char != zero {
                panic!("Invalid input, needs to be `1` or `0`. Got: {}", char);
            }
        }
    }

    let mut count_ones;
    for char_pos in 0..line_size {
        if char_pos == 0 {
            count_ones = get_char_count_at_char_pos(&lines, char_pos, &one);
            if count_ones > (lines.len() - count_ones) {
                fill_list(&lines, &mut oxygen_bins, &one);
                fill_list(&lines, &mut co2_bins, &zero);
            } else {
                fill_list(&lines, &mut oxygen_bins, &zero);
                fill_list(&lines, &mut co2_bins, &one);
            }
        } else {
            //caso ambos tenham acabado da stop
            if oxygen_bins.len() == 1 && co2_bins.len() == 1{
                break;
            }
            //para o oxygen
            if oxygen_bins.len() > 1 {
                count_ones = get_char_count_at_char_pos(&oxygen_bins, char_pos, &one);
                if count_ones < (oxygen_bins.len() - count_ones) {
                    remove_all_with_char_at_char_pos(&mut oxygen_bins, char_pos, &one);
                } else {
                    remove_all_with_char_at_char_pos(&mut oxygen_bins, char_pos, &zero);
                }
            }

            //para o co2
            if co2_bins.len() > 1 {
                count_ones = get_char_count_at_char_pos(&co2_bins, char_pos, &one);
                if count_ones < (co2_bins.len() - count_ones) {
                    remove_all_with_char_at_char_pos(&mut co2_bins, char_pos, &zero);
                } else {
                    remove_all_with_char_at_char_pos(&mut co2_bins, char_pos, &one);
                }
            }
        }
    }
    let bin_to_u32 = |vector : &Vec<&str>|{{
        let mut number = 0;
        let mut chars = vector[0].chars().collect::<Vec<_>>();
        chars.reverse();
        for (pos, char) in  chars.iter().enumerate(){
            if *char == one {
                number |= 2u32.pow(pos as u32);
            }
        }
        number
    }};
    let oxygen = bin_to_u32(&oxygen_bins);
    let co2 = bin_to_u32(&co2_bins);

    println!("oxygen: {0:b}/{0}; co2: {1:b}/{1}; final: {2}", oxygen, co2, oxygen*co2);

}

fn remove_all_with_char_at_char_pos(vec: &mut Vec<&str>, char_pos: usize, char: &char) {
    let mut to_delete = vec![];
    for (pos, bin) in vec.iter().enumerate() {
        let chars = bin.chars().collect::<Vec<_>>();
        if chars[char_pos] == *char {
            to_delete.push(pos);
        }
    }
    while let Some(pos) = to_delete.pop() {
        vec.remove(pos);
    }
}

fn get_char_count_at_char_pos(lines: &[&str], char_pos: usize, one: &char) -> usize {
    let mut count_ones = 0usize;
    for line in lines {
        let chars = line.chars().collect::<Vec<_>>();
        let char = &chars[char_pos];
        if *char == *one {
            count_ones += 1;
        }
    }
    count_ones
}

// fils list `to_save`, with the lines in `lines` where the first char is `to_get`
fn fill_list<'a>(lines: &Vec<&'a str>, to_save: &mut Vec<&'a str>, to_get: &char) {
    for line in lines {
        if line.chars().collect::<Vec<_>>()[0] == *to_get {
            to_save.push(line);
        }
    }
}
