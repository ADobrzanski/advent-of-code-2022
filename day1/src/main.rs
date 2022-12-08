fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("Could not read from \"./input.txt\"");

    let calories_per_elf = input
        .clone()
        .split_terminator("\n\n")
        .map(|elf| elf
            .lines()
            .map(|calorie_string| calorie_string.parse::<i32>())
            .try_fold(0, |sum, calories| calories.map(|c| c + sum)))
        .collect::<Result<Vec<i32>, _>>()
        .unwrap();

    match calories_per_elf.iter().max() {
        Some(max_calories) => println!("The most calories an elf carries is {max_calories} calories"),
        None => println!("There were no elves?! Verify your input!")
    };

    let mut calories_per_elf_part2 = calories_per_elf.clone();
    calories_per_elf_part2.sort();
    calories_per_elf_part2.reverse();

    let sum_of_top3_elves = calories_per_elf_part2.get(..3).map(|top3|top3.iter().sum::<i32>());

    match sum_of_top3_elves {
        Some(sum_top_3) => println!("Top 3 thickest elves carry together {sum_top_3} calories"),
        None => println!("Not even 3 evles in here?! Verify your input!")
    };
}


