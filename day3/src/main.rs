#![feature(array_chunks)]
use std::fs;

fn get_common_char(strings: Vec<&str>) -> Option<char> {
    let first_string = &strings[0];
    let other_strings = &strings[1..];

    let common_char: String = other_strings.iter().fold(
        first_string.to_string(),
        |common_char, next_string| common_char.chars()
            .filter(|&x| next_string.contains(x))
            .collect::<String>()
   );

    common_char.chars().nth(0)
}

fn get_priority<'a>(item_type: char) -> Result<u32, &'a str> {
    match item_type {
       'a'..='z' => Ok(item_type as u32 - 96),
       'A'..='Z' => Ok(item_type as u32 - 38),
       _ => Err("Illegal item type character")
    }
}

fn solve_part_1<'a>(input: &String) -> Result<u32, &'a str> {
    input.lines().try_fold(0u32, |priority_sum, rucksack| {
        let rucksack_midpoint = rucksack.len().checked_div(2)
            .ok_or("Rucksack compartments not evenly sized")?;

        let (compartment_a, compartment_b) = rucksack.split_at(rucksack_midpoint);

        let duplicated_type = get_common_char([compartment_a, compartment_b].to_vec())
            .ok_or("No duplicated item")?;

        get_priority(duplicated_type).map(|priority_value| priority_value + priority_sum)
    })
}

fn solve_part_2<'a>(input: &String) -> Result<u32, &'a str> {
    input.lines().collect::<Vec<&str>>().array_chunks::<3>()
    .try_fold(0, |acc, group| {
        let common_char = get_common_char(group.to_vec()).ok_or("No common item type")?;
        get_priority(common_char).map(|priority| priority + acc)
    })
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Reading from file failed!");

    match solve_part_1(&input) {
        Ok(priority_sum) => println!("Sum of mixed item types priorities: {priority_sum}!"),
        Err(err_msg) => println!("{err_msg}"),
    };

    match solve_part_2(&input) {
        Ok(priority_sum) => println!("Sum of badges: {priority_sum}!"),
        Err(err_msg) => println!("{err_msg}"),
    };
}
