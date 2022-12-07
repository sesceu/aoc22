use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    // The input file containing the strategy guide.
    let input_file = File::open("./03/input.txt").expect("Failed to open `input.txt`!");
    let buffered = BufReader::new(input_file);

    let mut score : u32 = 0;
    for line in buffered.lines() {
        let line = line?;
        let compartments = day_03_lib::split_inventory(line)?;
        let same_chars = day_03_lib::find_same_chars(compartments.0, compartments.1);
        score += day_03_lib::get_char_score(same_chars);
    }

    println!("total score: {score}");

    Ok(())
}
