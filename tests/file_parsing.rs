use aoc2022;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_str_to_filepath()
    {
        let res = aoc2022::open_file("tests/data/day7a.txt");
        assert!(res.is_ok());
    }

    #[test]
    fn parse_str_to_logical_filepath()
    {
        let res = aoc2022::open_file("tests/data/../data/day7a.txt");
        assert!(res.is_ok());
    }
}
