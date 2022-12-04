use std::fs::File;
use std::io::{BufRead, BufReader, Error};

const INVALID_OWN_SHAPE_ERROR: &str = "Invalid own shape character!";
const INVALID_OTHER_SHAPE_ERROR: &str = "Invalid other shape character!";

// Return the points for the shape I choose.
fn get_shape_points(own_shape: char) -> Result<u32, Error> {
    match own_shape {
        'X' => Ok(1), // Rock
        'Y' => Ok(2), // Paper
        'Z' => Ok(3), // Scissors
        _ => panic!("{}", INVALID_OWN_SHAPE_ERROR),
    }
}

// Return the points for the outcome of the game.
fn get_win_points(other_shape: char, own_shape: char) -> Result<u32, Error> {
    match other_shape {
        'A' => match own_shape {
            'X' => return Ok(3), // Rock - Rock = Draw = 3
            'Y' => return Ok(6), // Rock - Paper = Win = 6
            'Z' => return Ok(0), // Rock - Scissors = Loss = 0
            _ => panic!("{}", INVALID_OWN_SHAPE_ERROR),
        },
        'B' => match own_shape {
            'X' => return Ok(0), // Paper - Rock = Loss = 0
            'Y' => return Ok(3), // Paper - Paper = Draw = 3
            'Z' => return Ok(6), // Paper - Scissors = Win = 6
            _ => panic!("{}", INVALID_OWN_SHAPE_ERROR),
        },
        'C' => match own_shape {
            'X' => return Ok(6), // Scissors - Rock = Win = 6
            'Y' => return Ok(0), // Scissors - Paper = Loss = 0
            'Z' => return Ok(3), // Scissors - Scissors = Draw = 3
            _ => panic!("{}", INVALID_OWN_SHAPE_ERROR),
        },
        _ => panic!("{}", INVALID_OTHER_SHAPE_ERROR),
    }
}

// Given `other_shape` return the shape to make the round a draw.
fn get_char_to_draw(other_shape: char) -> Result<char, Error> {
    match other_shape {
        'A' => Ok('X'), // Rock - Rock
        'B' => Ok('Y'), // Paper - Paper
        'C' => Ok('Z'), // Scissors - Scissors
        _ => panic!("{}", INVALID_OTHER_SHAPE_ERROR),
    }
}

// Given `other_shape` return the shape to make the round a win.
fn get_char_to_win(other_shape: char) -> Result<char, Error> {
    match other_shape {
        'A' => Ok('Y'), // Rock - Paper
        'B' => Ok('Z'), // Paper - Scissors
        'C' => Ok('X'), // Scissors - Rock
        _ => panic!("{}", INVALID_OTHER_SHAPE_ERROR),
    }
}

// Given `other_shape` return the shape to make the round a loose.
fn get_char_to_loose(other_shape: char) -> Result<char, Error> {
    match other_shape {
        'A' => Ok('Z'), // Rock - Scissors
        'B' => Ok('X'), // Paper - Rock
        'C' => Ok('Y'), // Scissors - Paper
        _ => panic!("{}", INVALID_OTHER_SHAPE_ERROR),
    }
}

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
        let score_strategy_1: u32 =
            get_shape_points(strategy_shape)? + get_win_points(other_shape, strategy_shape)?;
        total_score_strategy_1 += score_strategy_1;

        // Strategy 2
        let my_shape: char;
        match strategy_shape {
            'X' => my_shape = get_char_to_loose(other_shape)?,
            'Y' => my_shape = get_char_to_draw(other_shape)?,
            'Z' => my_shape = get_char_to_win(other_shape)?,
            _ => panic!("Unknown strategy '{}'", strategy_shape),
        }
        let score_strategy_2: u32 =
            get_shape_points(my_shape)? + get_win_points(other_shape, my_shape)?;
        total_score_strategy_2 += score_strategy_2;
    }
    println!("total score (strategy 1): {}", total_score_strategy_1);
    println!("total score (strategy 2): {}", total_score_strategy_2);
    Ok(())
}
