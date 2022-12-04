use std::io::{Error};

const INVALID_OWN_SHAPE_ERROR: &str = "Invalid own shape character!";
const INVALID_OTHER_SHAPE_ERROR: &str = "Invalid other shape character!";

// Return the points for the shape I choose.
pub fn get_shape_points(own_shape: char) -> Result<u32, Error> {
    match own_shape {
        'X' => Ok(1), // Rock
        'Y' => Ok(2), // Paper
        'Z' => Ok(3), // Scissors
        _ => panic!("{}", INVALID_OWN_SHAPE_ERROR),
    }
}

// Return the points for the outcome of the game.
pub fn get_win_points(other_shape: char, own_shape: char) -> Result<u32, Error> {
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
pub fn get_char_to_draw(other_shape: char) -> Result<char, Error> {
    match other_shape {
        'A' => Ok('X'), // Rock - Rock
        'B' => Ok('Y'), // Paper - Paper
        'C' => Ok('Z'), // Scissors - Scissors
        _ => panic!("{}", INVALID_OTHER_SHAPE_ERROR),
    }
}

// Given `other_shape` return the shape to make the round a win.
pub fn get_char_to_win(other_shape: char) -> Result<char, Error> {
    match other_shape {
        'A' => Ok('Y'), // Rock - Paper
        'B' => Ok('Z'), // Paper - Scissors
        'C' => Ok('X'), // Scissors - Rock
        _ => panic!("{}", INVALID_OTHER_SHAPE_ERROR),
    }
}

// Given `other_shape` return the shape to make the round a loose.
pub fn get_char_to_loose(other_shape: char) -> Result<char, Error> {
    match other_shape {
        'A' => Ok('Z'), // Rock - Scissors
        'B' => Ok('X'), // Paper - Rock
        'C' => Ok('Y'), // Scissors - Paper
        _ => panic!("{}", INVALID_OTHER_SHAPE_ERROR),
    }
}
