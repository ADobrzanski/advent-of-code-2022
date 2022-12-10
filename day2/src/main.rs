use std::fs;

const LOSS: i32 = 0;
const TIE: i32 = 3;
const WIN: i32 = 6;

const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSORS: i32 = 3;

fn get_round_score_part1<'s, 'a, 'b>(column_1: &'a char, column_2: &'b char) -> Result<i32, &'s str> {
    match (column_1, column_2) {
        ('A', 'X') => Ok(TIE + ROCK),
        ('A', 'Y') => Ok(WIN + PAPER),
        ('A', 'Z') => Ok(LOSS + SCISSORS),

        ('B', 'X') => Ok(LOSS + ROCK),
        ('B', 'Y') => Ok(TIE + PAPER),
        ('B', 'Z') => Ok(WIN + SCISSORS),

        ('C', 'X') => Ok(WIN + ROCK),
        ('C', 'Y') => Ok(LOSS + PAPER),
        ('C', 'Z') => Ok(TIE + SCISSORS),
        _ => Err("Unknown round description {_.0} {_.1}")
    }
}

fn get_round_score_part2<'s, 'a, 'b>(column_1: &'a char, column_2: &'b char) -> Result<i32, &'s str> {
    match (column_1, column_2) {
        ('A', 'X') => Ok(LOSS + SCISSORS),
        ('A', 'Y') => Ok(TIE + ROCK),
        ('A', 'Z') => Ok(WIN + PAPER),

        ('B', 'X') => Ok(LOSS + ROCK),
        ('B', 'Y') => Ok(TIE + PAPER),
        ('B', 'Z') => Ok(WIN + SCISSORS),

        ('C', 'X') => Ok(LOSS + PAPER),
        ('C', 'Y') => Ok(TIE + SCISSORS),
        ('C', 'Z') => Ok(WIN + ROCK),
        _ => Err("Unknown round description {_.0} {_.1}")
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Reading file failed!");

    let rounds = input
        .lines()
        .map(|input_row| {
            let col1 = input_row.chars().nth(0).ok_or("Failure reading first column")?;
            let col2 = input_row.chars().nth(2).ok_or("Failure reading second column")?;
            Ok::<(char, char), &str>((col1, col2))
        })
        .collect::<Result<Vec<(char, char)>, _>>().unwrap();

    let result_part1 = rounds
        .iter()
        .try_fold(0, |total_score, columns| {
            get_round_score_part1(&columns.0, &columns.1).map(|x| x + total_score)
        });

    match result_part1 {
       Ok(value) => println!("Following (part1) strategy guide results in score of: {value}"),
       Err(value) => println!("{value}"),
    };

    let result_part2 = rounds
        .iter()
        .try_fold(0, |total_score, columns| {
            get_round_score_part2(&columns.0, &columns.1).map(|x| x + total_score)
        });

    match result_part2 {
       Ok(value) => println!("Following (part2) strategy guide results in score of: {value}"),
       Err(value) => println!("{value}"),
    };
}
