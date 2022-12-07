use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let input_file = File::open("./04/input.txt").expect("Failed to open `input.txt`!");
    let buffered = BufReader::new(input_file);

    let mut sum_contained: u32 = 0;
    let mut sum_overlap: u32 = 0;
    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();

    for line in buffered.lines() {
        let line = line?;
        if !re.is_match(&line) {
            panic!("line '{}' doesn't match regex!", line);
        }
        let caps = re.captures(&line).unwrap();
        let start1: u32 = caps
            .get(1)
            .expect("Failed to extract start1")
            .as_str()
            .parse::<u32>()
            .unwrap();
        let end1: u32 = caps
            .get(2)
            .expect("Failed to extract end1")
            .as_str()
            .parse::<u32>()
            .unwrap();
        let start2: u32 = caps
            .get(3)
            .expect("Failed to extract start2")
            .as_str()
            .parse::<u32>()
            .unwrap();
        let end2: u32 = caps
            .get(4)
            .expect("Failed to extract end2")
            .as_str()
            .parse::<u32>()
            .unwrap();

        sum_contained += day_lib::contains(start1, end1, start2, end2) as u32;
        sum_overlap += day_lib::overlaps(start1, end1, start2, end2) as u32;
    }

    println!("total sum_contained: {sum_contained}");
    println!("total sum_overlap: {sum_overlap}");

    Ok(())
}
