use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let input_file = File::open("./03/input2.txt").expect("Failed to open `input.txt`!");
    let buffered = BufReader::new(input_file);

    let mut score : u32 = 0;

    let mut line = buffered.lines().peekable();
    while line.peek().is_some() {
        let inventory_0 = line.next().unwrap().unwrap();
        let inventory_1 = line.next().unwrap().unwrap();
        let inventory_2 = line.next().unwrap().unwrap();

        let same_chars_0_1 = day_03_lib::find_same_chars(inventory_0, inventory_1);
        let same_chars = day_03_lib::find_same_chars(same_chars_0_1, inventory_2);
        score += day_03_lib::get_char_score(same_chars);
    }

    println!("total score: {score}");

    Ok(())
}
