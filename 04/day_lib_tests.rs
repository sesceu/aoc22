#[cfg(test)]
mod tests {

    #[test]
    fn contains() {
        assert_eq!(day_lib::contains(0, 0, 0, 0), true);
        assert_eq!(day_lib::contains(2, 4, 6, 8), false);
        assert_eq!(day_lib::contains(2, 3, 4, 5), false);
        assert_eq!(day_lib::contains(5, 7, 7, 9), false);
        assert_eq!(day_lib::contains(2, 8, 3, 7), true);
        assert_eq!(day_lib::contains(6, 6, 4, 6), true);
        assert_eq!(day_lib::contains(2, 6, 4, 8), false);
    }

    #[test]
    fn overlaps() {
        assert_eq!(day_lib::overlaps(0, 0, 0, 0), true);
        assert_eq!(day_lib::overlaps(2, 4, 6, 8), false);
        assert_eq!(day_lib::overlaps(2, 3, 4, 5), false);
        assert_eq!(day_lib::overlaps(5, 7, 7, 9), true);
        assert_eq!(day_lib::overlaps(2, 8, 3, 7), true);
        assert_eq!(day_lib::overlaps(6, 6, 4, 6), true);
        assert_eq!(day_lib::overlaps(2, 6, 4, 8), true);
    }
}
