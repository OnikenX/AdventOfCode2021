#[derive(Clone, Copy, Debug)]
struct table_win {
    pos: usize,
    score: i32,
}

pub fn p1(input: String) {
    evaluate_which_win(input, true);
}

pub fn p2(input: String) {
    evaluate_which_win(input, false);
}

// descobrir a board que vai ganhar primeiro caso `first` esteja a true, caso contrario procura o ultimo
fn evaluate_which_win(input: String, first: bool) {
    //saves the last biggest score
    let mut wins = vec![];
    // saves current table
    let mut table: Vec<Vec<&str>> = vec![vec![]; 5];
    let mut lines = input.lines();
    let solutions = lines.next().unwrap().split(',').collect::<Vec<_>>();
    let mut table_line = 0;

    while let Some(line) = lines.next() {
        let line_ = line.clone();
        // skip dos espaços em branco
        if line.chars().all(|x| x.is_whitespace()) {
            continue;
        }
        //vai se lendo as tabelas
        table[table_line] = line
            .split(" ")
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>();
        table_line += 1;
        //quando se le uma tabela inteira
        if table_line == 5 {
            //reseta se a tabela
            table_line = 0;
            if let Some((score, pos)) = verify_bingo(&table, &solutions) {
                wins.push(table_win { pos, score: score });
            }
        }
    }
    let mut desired;
    let mut wins_iter = wins.iter();
    desired = wins_iter.next().unwrap();
    while let Some(next) = wins_iter.next() {
        if first {
            if desired.pos > next.pos {
                desired = next;
            }
        } else {
            if desired.pos < next.pos {
                desired = next;
            }
        }
    }

    println!("tables: {:?}", desired);
}

//verificas se a tabela é bingo
fn verify_bingo(table: &Vec<Vec<&str>>, solutions: &Vec<&str>) -> Option<(i32, usize)> {
    //verifies if there is a bingo by column
    let mut checked_numbers = [[false; 5]; 5];
    for (pos, solution) in solutions.iter().enumerate() {
        for (pos_column, table_line) in table.iter().enumerate() {
            for (pos_row, f) in table_line.iter().enumerate() {
                if solution == f {
                    checked_numbers[pos_column][pos_row] = true;
                }
            }
        }

        if check_bingo(&checked_numbers) {
            let mut final_score = 0;
            for i in 0..5 {
                for f in 0..5 {
                    if !checked_numbers[i][f] {
                        let parsed = table[i][f].parse::<i32>().unwrap();
                        final_score += parsed;
                    }
                }
            }
            return Some((final_score * solution.parse::<i32>().unwrap(), pos));
        }
    }

    None
}

fn check_bingo(checked_numbers: &[[bool; 5]; 5]) -> bool {
    for i in 0..5 {
        let mut count_row = 0;
        let mut count_column = 0;
        for f in 0..5 {
            if checked_numbers[i][f] {
                count_row += 1;
            }
            if checked_numbers[f][i] {
                count_column += 1;
            }
            if count_column == 5 || count_row == 5 {
                return true;
            }
        }
    }
    return false;
}
