#[cfg(test)]
mod tests {
    use std::io::Error;

    #[test]
    #[should_panic]
    fn test_split_inventory() {
        let _inventory = day_03_lib::split_inventory("aaa".to_string());
    }

    #[test]
    fn test_split_inventory_when_typical() -> Result<(), Error> {
        let inventory = day_03_lib::split_inventory("aaccdd".to_string())?;
        assert_eq!(inventory.0.chars().count(), inventory.1.chars().count());

        Ok(())
    }

    #[test]
    fn test_find_same_chars() -> Result<(), Error> {
        let mut same_chars = day_03_lib::find_same_chars("aa".to_string(), "bb".to_string());
        assert_eq!(same_chars, "");

        let mut compartments = day_03_lib::split_inventory("vJrwpWtwJgWrhcsFMMfFFhFp".to_string())?;
        same_chars = day_03_lib::find_same_chars(compartments.0, compartments.1);
        assert_eq!(same_chars, "p");

        compartments = day_03_lib::split_inventory("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string())?;
        same_chars = day_03_lib::find_same_chars(compartments.0, compartments.1);
        assert_eq!(same_chars, "L");

        compartments = day_03_lib::split_inventory("PmmdzqPrVvPwwTWBwg".to_string())?;
        same_chars = day_03_lib::find_same_chars(compartments.0, compartments.1);
        assert_eq!(same_chars, "P");

        compartments = day_03_lib::split_inventory("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string())?;
        same_chars = day_03_lib::find_same_chars(compartments.0, compartments.1);
        assert_eq!(same_chars, "v");

        compartments = day_03_lib::split_inventory("ttgJtRGJQctTZtZT".to_string())?;
        same_chars = day_03_lib::find_same_chars(compartments.0, compartments.1);
        assert_eq!(same_chars, "t");

        compartments = day_03_lib::split_inventory("CrZsJsPPZsGzwwsLwLmpwMDw".to_string())?;
        same_chars = day_03_lib::find_same_chars(compartments.0, compartments.1);
        assert_eq!(same_chars, "s");

        Ok(())
    }

    #[test]
    fn test_get_char_score() {
        let mut score = day_03_lib::get_char_score("".to_string());
        assert_eq!(score, 0);

        score = day_03_lib::get_char_score("a".to_string());
        assert_eq!(score, 1);

        score = day_03_lib::get_char_score("z".to_string());
        assert_eq!(score, 26);

        score = day_03_lib::get_char_score("A".to_string());
        assert_eq!(score, 27);

        score = day_03_lib::get_char_score("Z".to_string());
        assert_eq!(score, 52);
    }
}
