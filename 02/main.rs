use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    // The input file containing the strategy guide.
    let input_file = File::open("./02/input.txt").expect("Failed to open `input.txt`!");
    let buffered = BufReader::new(input_file);

    let mut total_score_strategy_1: u32 = 0;
    let mut total_score_strategy_2: u32 = 0;

    for line in buffered.lines() {
        let line = line?;
        let other_shape = line.chars().nth(0).expect("Failed to opponent shape!");
        let strategy_shape = line.chars().nth(2).expect("Failed to get strategy shape!");

        // Strategy 1
        let score_strategy_1: u32 = day_02_lib::get_shape_points(strategy_shape)?
            + day_02_lib::get_win_points(other_shape, strategy_shape)?;
        total_score_strategy_1 += score_strategy_1;

        // Strategy 2
        let my_shape: char;
        match strategy_shape {
            'X' => my_shape = day_02_lib::get_char_to_loose(other_shape)?,
            'Y' => my_shape = day_02_lib::get_char_to_draw(other_shape)?,
            'Z' => my_shape = day_02_lib::get_char_to_win(other_shape)?,
            _ => panic!("Unknown strategy '{}'", strategy_shape),
        }
        let score_strategy_2: u32 = day_02_lib::get_shape_points(my_shape)?
            + day_02_lib::get_win_points(other_shape, my_shape)?;
        total_score_strategy_2 += score_strategy_2;
    }
    println!("total score (strategy 1): {}", total_score_strategy_1);
    println!("total score (strategy 2): {}", total_score_strategy_2);
    Ok(())
}
