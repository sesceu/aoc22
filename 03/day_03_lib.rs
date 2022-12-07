pub(crate) use std::io::Error;

pub fn split_inventory(inventory: String) -> Result<(String, String), Error> {
    let len = inventory.chars().count();
    if len % 2 == 1 {
        panic!("{}", "Compartments have different size!");
    }
    Ok((
        String::from(&inventory[..len / 2]),
        String::from(&inventory[len / 2..]),
    ))
}

pub fn find_same_chars(compartment_a: String, compartment_b: String) -> String {
    let mut result: String = "".to_string();
    for c in compartment_a.chars() {
        if compartment_b.contains(c) {
            if !result.contains(c) {
                result += &String::from(c);
            }
        }
    }
    return result;
}

pub fn get_char_score(chars : String) -> u32 {
    let ordinal_small_a = 'a' as u32;
    let ordinal_big_a = 'A' as u32;
    let mut score : u32 = 0;
    for c in chars.chars() {
        let ordinal_c : u32 = c as u32;
        if ordinal_c >= ordinal_small_a {
            score += 1 + ordinal_c - ordinal_small_a;
        } else {
            score += 27 + ordinal_c - ordinal_big_a;
        }
    }
    return score;
}
