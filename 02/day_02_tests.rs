#[cfg(test)]
mod tests {
    use std::io::Error;

    #[test]
    fn test_get_shape_points() -> Result<(), Error> {
        let mut points: u32 = day_02_lib::get_shape_points('X')?;
        assert_eq!(points, 1);

        points = day_02_lib::get_shape_points('Y')?;
        assert_eq!(points, 2);

        points = day_02_lib::get_shape_points('Z')?;
        assert_eq!(points, 3);

        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_get_shape_points_when_not_a_shape() {
        let _result = day_02_lib::get_shape_points('A');
    }

    #[test]
    fn test_get_char_to_draw() -> Result<(), Error> {
        let mut shape: char = day_02_lib::get_char_to_draw('A')?;
        assert_eq!(shape, 'X');

        shape = day_02_lib::get_char_to_draw('B')?;
        assert_eq!(shape, 'Y');

        shape = day_02_lib::get_char_to_draw('C')?;
        assert_eq!(shape, 'Z');

        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_get_char_to_draw_when_not_a_shape() {
        let _result = day_02_lib::get_char_to_draw('X');
    }

    #[test]
    fn test_get_char_to_win() -> Result<(), Error> {
        let mut shape: char = day_02_lib::get_char_to_win('A')?;
        assert_eq!(shape, 'Y');

        shape = day_02_lib::get_char_to_win('B')?;
        assert_eq!(shape, 'Z');

        shape = day_02_lib::get_char_to_win('C')?;
        assert_eq!(shape, 'X');

        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_get_char_to_win_when_not_a_shape() {
        let _result = day_02_lib::get_char_to_win('X');
    }

    #[test]
    fn test_get_char_to_loose() -> Result<(), Error> {
        let mut shape: char = day_02_lib::get_char_to_loose('A')?;
        assert_eq!(shape, 'Z');

        shape = day_02_lib::get_char_to_loose('B')?;
        assert_eq!(shape, 'X');

        shape = day_02_lib::get_char_to_loose('C')?;
        assert_eq!(shape, 'Y');

        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_get_char_to_loose_when_not_a_shape() {
        let _result = day_02_lib::get_char_to_loose('X');
    }
}
