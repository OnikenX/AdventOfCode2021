use core::panic;
use std::{ops::RangeBounds, vec};

use crate::downloader::{download, get_token};

// existem vários números com vários segmentos

// 0:      1:      2:      3:      4:
// aaaa    ....    aaaa    aaaa    ....
// b    c  .    c  .    c  .    c  b    c
// b    c  .    c  .    c  .    c  b    c
// ....    ....    dddd    dddd    dddd
// e    f  .    f  e    .  .    f  .    f
// e    f  .    f  e    .  .    f  .    f
// gggg    ....    gggg    gggg    ....

//  5:      6:      7:      8:      9:
// aaaa    aaaa    aaaa    aaaa    aaaa
// b    .  b    .  .    c  b    c  b    c
// b    .  b    .  .    c  b    c  b    c
// dddd    dddd    ....    dddd    dddd
// .    f  e    f  .    f  e    f  .    f
// .    f  e    f  .    f  e    f  .    f
// gggg    gggg    ....    gggg    gggg

//valor-n segmentos-segmentos:
//0 - 6 - abcefg
//1 - 2 - cf
//2 - 5 - acdeg
//3 - 5 - acdfg
//4 - 4 - bcdf
//5 - 5 - abdfg
//6 - 6 - abdefg
//7 - 3 - acf
//8 - 7 - abcdefg
//9 - 6 - abcdfg

//únicos n de segmentos:
//1 - 2 - cf
//4 - 4 - bcdf
//7 - 3 - acf
//8 - 7 - abcdefg

trait RemoveLetter {
    fn remove_letters(&mut self, pattern: String);
}

impl RemoveLetter for String {
    fn remove_letters(&mut self, pattern: String) {
        for char in pattern.chars() {
            while let Some(index) = self.find(char) {
                self.remove(index);
            }
        }
    }
}

fn get_index_for_length(length: usize, maps: &Vec<&str>) -> Vec<usize> {
    let mut index = vec![];
    for i in 0..maps.len() {
        if maps[i].len() == length {
            index.push(i);
        }
    }
    index
}

fn get_comuns(maps: &Vec<&str>, indexes: &Vec<usize>) -> Vec<char> {
    if indexes.len() < 2 {
        panic!("Need at least 2 indexes");
    }
    let mut chars = maps[indexes[0]].chars().collect::<Vec<_>>();

    for i in 1..indexes.len() {
        let mut comuns = vec![];
        chars.iter().for_each(|x| {
            if maps[indexes[i]].contains(*x) {
                comuns.push(*x);
            }
        });
        chars = comuns;
    }
    chars
}

fn mapping_correct(maps: &Vec<&str>) -> [char; 7] {
    //comuns cfb
    //1 tem o cf comum com o 478
    //4 tem o b comum com 8
    //4 tem o d comum com 8
    //7 tem o a comum com o 8

    //a:0,b:1,c:2,d:3,e:4,f:5,g:6
    let mut mapping = ['x'; 7];

    //todos os possiveis
    let mut map_index = [
        maps.len(),                       //0
        get_index_for_length(2, maps)[0], //1
        maps.len(),                       //2
        maps.len(),                       //3
        get_index_for_length(4, maps)[0], //4
        maps.len(),                       //5
        maps.len(),                       //6
        get_index_for_length(3, maps)[0], //7
        get_index_for_length(7, maps)[0], //8
        maps.len(),                       //9
    ];

    // com 5 segmentos existe o 2, o 3 e o 5,
    let indexes_com_5 = get_index_for_length(5, maps);
    let comuns = get_comuns(maps, &indexes_com_5);
    let comuns = String::from_iter(comuns);
    //o 4 tem o d unico
    mapping[3] = get_comuns(&vec![maps[map_index[4]], &comuns], &vec![0, 1])[0];
    //o 7 tem o a unico
    mapping[0] = get_comuns(&vec![maps[map_index[7]], &comuns], &vec![0, 1])[0];

    //sendo que o `d` e o `a` ja foi descoberto pode se descobrir o b que é algo que o 4 tem mas o 7 nao tem
    let mut quatro_e_sete = String::from(maps[map_index[4]]) + maps[map_index[7]];
    //remover os que estao no um para tirar o cf
    maps[map_index[1]].chars().for_each(|x| {
        quatro_e_sete.remove_letters(x.to_string());
    });
    //remover o d
    while let Some(index) = quatro_e_sete.find(mapping[3]) {
        quatro_e_sete.remove(index);
    }
    //remover o a
    while let Some(index) = quatro_e_sete.find(mapping[0]) {
        quatro_e_sete.remove(index);
    }
    //b
    mapping[1] = vec![quatro_e_sete.chars().next().unwrap()][0];

    //sabendo o b podemos descobrir qual é o 5 dos numeros com 5 segmentos(2,3,5)
    for i in indexes_com_5 {
        if maps[i].contains(mapping[1]) {
            map_index[5] = i;
        }
    }
    //sabendo agora o 5, posso retirar o a,b,d  e ficar só com o f e g
    let mut letters_5 = String::from(maps[map_index[5]]);
    letters_5.remove_letters(String::from_iter([mapping[0], mapping[1], mapping[3]]));
    let comuns = get_comuns(&vec![&letters_5, maps[map_index[1]]], &vec![0, 1]);
    //f
    mapping[5] = comuns[0];
    //remove se o que esta f e fica se com o g
    letters_5.remove_letters(String::from_iter([comuns[0]]));
    //g
    mapping[6] = letters_5.chars().next().unwrap();

    //agora para descobrir o c basta buscar as letras do 1 e remover o f
    let mut letters_1 = String::from(maps[map_index[1]]);
    letters_1.remove_letters(String::from_iter([mapping[5]]));
    //c
    mapping[2] = letters_1.chars().next().unwrap();

    //para descobrir o ultimo, e
    let mut todos = String::from("abcdefg");
    todos.remove_letters(String::from_iter(mapping));
    //e
    mapping[4] = todos.chars().next().unwrap();

    mapping
}

pub(crate) fn p1() {
    solution(true);
}
pub(crate) fn p2() {
    solution(false);
}

fn solution(part1: bool) {
    let input =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
            .to_string();
    // let input =
    //     "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"
    //         .to_string();
    let input = download(2021, 8, &get_token());
    // let mut map :Vec<(Vec<i32>, Vec<i32>)>= vec![];
    let lines = input.lines().collect::<Vec<_>>();
    let mut counter = 0u64;
    for i in 0..lines.len() {
        let line = lines[i].trim();
        let mut splited = line.split(" | ");
        let mut temp = vec![];
        
        add_to_vec(&mut temp, splited.next().unwrap());
        let mut outputs = vec![];
        add_to_vec(&mut outputs, splited.next().unwrap());
        let mut digits = ['0'; 4];
        let mapping = mapping_correct(&temp);
        for (pos,output) in outputs.iter().enumerate() {
            if part1 {
                if output.len() == 2 || output.len() == 4 || output.len() == 3 || output.len() == 7
                {
                    //unique_sizes_counter
                    counter += 1;
                }
            } else {
                digits[pos] = output_to_number(output, &mapping);
            }
        }
        counter += String::from_iter(digits).parse::<u64>().unwrap();
    }
    if part1 {
        println!("unique_sizes_counter: {}", counter);
    } else {
        println!("sum: {}", counter);
    }
}

fn output_to_number(output: &str, mapping: &[char; 7]) -> char{
    let mut final_number = 0;
    let a = 'a' as u8;
    let mut output_chars = output.chars().collect::<Vec<_>>();
    for i_out in 0..output_chars.len() {
        let (i_map, _) = mapping
            .iter()
            .enumerate()
            .find(|(_, char)| **char == output_chars[i_out])
            .unwrap();
        //coverts from the index of the map to a correct char
        output_chars[i_out] = (a + (i_map as u8)) as char;
    }
    output_chars.sort();
    let output = String::from_iter(output_chars);
    match output.as_str(){
        "abcefg" => '0',
        "cf"=> '1',
        "acdeg" => '2',
        "acdfg" => '3',
        "bcdf" => '4',
        "abdfg" => '5',
        "abdefg" => '6',
        "acf" => '7',
        "abcdefg" => '8',
        "abcdfg" => '9',
        _ => panic!("Cant find match!")
    }
}

fn add_to_vec<'a>(vec: &mut Vec<&'a str>, line: &'a str) {
    *vec = line.split(' ').collect::<Vec<_>>(); /*for_each(|x| vec.push(x.parse::<i32>().unwrap()))*/
}
