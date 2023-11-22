use aoc2022;


#[cfg(test)]
mod tests {
    use aoc2022::day9::*;

    use super::*;
    
    #[test]
    fn test_moves()
    {
        let mut grid = Grid::new(4, 0);
        assert_eq!(grid.head_pos(), &Pos::new(4, 0));
        assert_eq!(grid.tail_pos(), &Pos::new(4, 0));
        grid.move_head(Token::Up, 1);
        assert_eq!(grid.head_pos(), &Pos::new(3, 0));
        assert_eq!(grid.tail_pos(), &Pos::new(4, 0));
        grid.move_head(Token::Up, 1);
        assert_eq!(grid.head_pos(), &Pos::new(2, 0));
        assert_eq!(grid.tail_pos(), &Pos::new(3, 0));
        grid.move_head(Token::Right, 1);
        assert_eq!(grid.head_pos(), &Pos::new(2, 1));
        assert_eq!(grid.tail_pos(), &Pos::new(3, 0));
        grid.move_head(Token::Right, 1);
        assert_eq!(grid.head_pos(), &Pos::new(2, 2));
        assert_eq!(grid.tail_pos(), &Pos::new(2, 1));
    }

    #[test]
    fn test_tokenizing()
    {
        let string = aoc2022::read_to_string("tests/data/day9a.txt").unwrap();
        let mut lex: logos::Lexer<'_, Token> = <Token as logos::Logos>::lexer(&string);
    
        assert_eq!(lex.next(), Some(Ok(Token::Right)));
        assert_eq!(lex.next(), Some(Ok(Token::Steps)));
        assert_eq!(lex.slice(), "4");
        assert_eq!(lex.next(), Some(Ok(Token::Up)));
        assert_eq!(lex.next(), Some(Ok(Token::Steps)));
        assert_eq!(lex.slice(), "4");
        assert_eq!(lex.next(), Some(Ok(Token::Left)));
        assert_eq!(lex.next(), Some(Ok(Token::Steps)));
        assert_eq!(lex.slice(), "3");
        assert_eq!(lex.next(), Some(Ok(Token::Down)));
        assert_eq!(lex.next(), Some(Ok(Token::Steps)));
        assert_eq!(lex.slice(), "1");
        assert_eq!(lex.next(), Some(Ok(Token::Right)));
        assert_eq!(lex.next(), Some(Ok(Token::Steps)));
        assert_eq!(lex.slice(), "4");
        assert_eq!(lex.next(), Some(Ok(Token::Down)));
        assert_eq!(lex.next(), Some(Ok(Token::Steps)));
        assert_eq!(lex.slice(), "1");
        assert_eq!(lex.next(), Some(Ok(Token::Left)));
        assert_eq!(lex.next(), Some(Ok(Token::Steps)));
        assert_eq!(lex.slice(), "5");
        assert_eq!(lex.next(), Some(Ok(Token::Right)));
        assert_eq!(lex.next(), Some(Ok(Token::Steps)));
        assert_eq!(lex.slice(), "2");
    }

    #[test]
    fn test_perform_moves()
    {
        let string = aoc2022::read_to_string("tests/data/day9a.txt").unwrap();
        let mut lex: logos::Lexer<'_, Token> = <Token as logos::Logos>::lexer(&string);
    
        let mut grid = Grid::new(4, 0);
        grid.perform_moves(&mut lex);
        assert_eq!(grid.head_pos(), &Pos::new(2, 2));
        assert_eq!(grid.tail_pos(), &Pos::new(2, 1));
    }

    #[test]
    fn test_first_example()
    {
        let string = aoc2022::read_to_string("tests/data/day9a.txt").unwrap();
        let mut lex: logos::Lexer<'_, Token> = <Token as logos::Logos>::lexer(&string);
    
        let mut grid = Grid::new(4, 0);
        grid.perform_moves(&mut lex);
        assert_eq!(grid.count_unique_visits(), 13);
    }

    #[test]
    fn test_first_task()
    {
        let string = aoc2022::read_to_string("tests/data/day9b.txt").unwrap();
        let mut lex: logos::Lexer<'_, Token> = <Token as logos::Logos>::lexer(&string);
    
        let mut grid = Grid::new(0, 0);
        grid.perform_moves(&mut lex);
        assert_eq!(grid.count_unique_visits(), 6269);
    }
}
