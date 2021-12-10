use crate::downloader::{download, get_token};

const OPENERS: &str = "([{<";
const CLOSERS: &str = ")]}>";

trait IsChunk {
    fn is_chunk_opener(&self) -> bool;
    fn get_chunk_closer(&self) -> Option<char>;
    fn get_illegal_points(&self) -> usize;
    fn get_completer_points(&self) -> usize;
}
impl IsChunk for char {
    fn is_chunk_opener(&self) -> bool {
        OPENERS.chars().any(|f| f == *self)
    }
    fn get_chunk_closer(&self) -> Option<char> {
        if let Some((pos, _)) = OPENERS.char_indices().find(|f| f.1 == *self) {
            if let Some((_, char)) = CLOSERS
                .char_indices()
                .find(|(this_pos, _)| *this_pos == pos)
            {
                Some(char)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_illegal_points(&self) -> usize {
        match self {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!("Character `{}` not expected.", self),
        }
    }

    fn get_completer_points(&self) -> usize {
        match self {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!("Character `{}` not expected.", self),
        }
    }
}

pub(crate) fn p1() {
    //corrupted lines are those who close with the worng character
    let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"
        .to_string();
    let input = download(2021, 10, &get_token());
    let mut illegal_points = 0usize;
    let mut incomplete_scores = vec![];
    for line in input.lines() {
        let mut corrupeted = false;
        let mut chunks = vec![];
        for character in line.chars() {
            if character.is_chunk_opener() {
                chunks.push(character);
            } else {
                //verifing closers
                if chunks.is_empty() {
                    eprintln!("Closing before opening is not possible.");
                    corrupeted = true;
                    break;
                } else {
                    let last = chunks.last().unwrap();
                    let expected_closer = last.get_chunk_closer().unwrap();
                    if expected_closer == character {
                        chunks.pop();
                    } else {
                        eprintln!(
                            "Expected `{}`, but found `{}` instead.",
                            expected_closer, character
                        );
                        corrupeted = true;
                        illegal_points += character.get_illegal_points();
                        break;
                    }
                }
            }
        }
        if !chunks.is_empty() && !corrupeted{
            // eprintln!("Incomplete chunk.");
            let mut completer_points = 0u128;
            while let Some(char) = chunks.last() {
                completer_points *= 5;
                completer_points += char.get_chunk_closer().unwrap().get_completer_points() as u128;
                chunks.pop();
            }
            println!(
                "For the incompleted line `{}` got {} points.",
                line, completer_points
            );
            incomplete_scores.push(completer_points);
        }
    }
    incomplete_scores.sort();
    println!("Illegal points: {}", illegal_points);
    println!("Incomplete points: {}", incomplete_scores[incomplete_scores.len()/2]);
}
