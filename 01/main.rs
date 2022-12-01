use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::vec::Vec;

fn main() -> Result<(), Error> {
    // The number of Elves from which to sum up the carried calories.
    let ns = [1, 3];

    // The input file containing the calories per Elf.
    let input_file = File::open("./01/input.txt").expect("Failed to open `input.txt`!");

    let buffered = BufReader::new(input_file);

    // Carries the total amount of calories carried by each Elf.
    let mut max_calories = Vec::new();

    let mut calories_of_current_elf: u32 = 0;
    for line in buffered.lines() {
        let str_calories = line?;
        // Are we at the end of the current Elf's inventory?
        if str_calories == "" {
            max_calories.push(calories_of_current_elf);
            calories_of_current_elf = 0;
            continue;
        }
        // Turn the string calories into an integer.
        let calories: u32 = str_calories
            .trim()
            .parse()
            .expect("Index entered was not a number");

        // Add the current calories to the current Elf's inventory.
        calories_of_current_elf += calories;
    }

    // Sort the carried calories in decending order.
    max_calories.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());

    for n in ns {
        // Sum up the top n carried calories.
        let top_n_sum: u32 = max_calories[0..n].iter().sum();
        println!("top_{}_sum: {}", n, top_n_sum);
    }
    Ok(())
}
