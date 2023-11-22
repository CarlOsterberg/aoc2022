use aoc2022;


#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use aoc2022::day12::*;

    use super::*;
    
    #[test]
    fn test_tokenizing()
    {
        let string = aoc2022::read_to_string("tests/data/day12a.txt").unwrap();
        let mut lex: logos::Lexer<'_, Token> = <Token as logos::Logos>::lexer(&string);
    
        assert_eq!(lex.next(), Some(Ok(Token::Start)));
        assert_eq!(lex.next(), Some(Ok(Token::Letter)));
        assert_eq!(lex.slice(), "a");
        assert_eq!(lex.next(), Some(Ok(Token::Letter)));
        assert_eq!(lex.slice(), "b");
        assert_eq!(lex.next(), Some(Ok(Token::Letter)));
        assert_eq!(lex.slice(), "q");
        assert_eq!(lex.next(), Some(Ok(Token::Letter)));
        assert_eq!(lex.slice(), "p");
        assert_eq!(lex.next(), Some(Ok(Token::Letter)));
        assert_eq!(lex.slice(), "o");
        assert_eq!(lex.next(), Some(Ok(Token::Letter)));
        assert_eq!(lex.slice(), "n");
        assert_eq!(lex.next(), Some(Ok(Token::Letter)));
        assert_eq!(lex.slice(), "m");
        assert_eq!(lex.next(), Some(Ok(Token::NewLine)));

        assert_eq!(lex.next(), Some(Ok(Token::Letter)));
        assert_eq!(lex.slice(), "a");
        assert_eq!(lex.next(), Some(Ok(Token::Letter)));
        assert_eq!(lex.slice(), "b");
        assert_eq!(lex.next(), Some(Ok(Token::Letter)));
        assert_eq!(lex.slice(), "c");
        assert_eq!(lex.next(), Some(Ok(Token::Letter)));
        assert_eq!(lex.slice(), "r");
        assert_eq!(lex.next(), Some(Ok(Token::Letter)));
        assert_eq!(lex.slice(), "y");
        assert_eq!(lex.next(), Some(Ok(Token::Letter)));
        assert_eq!(lex.slice(), "x");
        assert_eq!(lex.next(), Some(Ok(Token::Letter)));
        assert_eq!(lex.slice(), "x");
        assert_eq!(lex.next(), Some(Ok(Token::Letter)));
        assert_eq!(lex.slice(), "l");
        assert_eq!(lex.next(), Some(Ok(Token::NewLine)));

        assert_eq!(lex.next(), Some(Ok(Token::Letter)));
        assert_eq!(lex.slice(), "a");
        assert_eq!(lex.next(), Some(Ok(Token::Letter)));
        assert_eq!(lex.slice(), "c");
        assert_eq!(lex.next(), Some(Ok(Token::Letter)));
        assert_eq!(lex.slice(), "c");
        assert_eq!(lex.next(), Some(Ok(Token::Letter)));
        assert_eq!(lex.slice(), "s");
        assert_eq!(lex.next(), Some(Ok(Token::Letter)));
        assert_eq!(lex.slice(), "z");
        assert_eq!(lex.next(), Some(Ok(Token::End)));
        assert_eq!(lex.next(), Some(Ok(Token::Letter)));
        assert_eq!(lex.slice(), "x");
        assert_eq!(lex.next(), Some(Ok(Token::Letter)));
        assert_eq!(lex.slice(), "k");
        assert_eq!(lex.next(), Some(Ok(Token::NewLine)));

        assert_eq!(lex.next(), Some(Ok(Token::Letter)));
        assert_eq!(lex.slice(), "a");
        assert_eq!(lex.next(), Some(Ok(Token::Letter)));
        assert_eq!(lex.slice(), "c");
        assert_eq!(lex.next(), Some(Ok(Token::Letter)));
        assert_eq!(lex.slice(), "c");
        assert_eq!(lex.next(), Some(Ok(Token::Letter)));
        assert_eq!(lex.slice(), "t");
        assert_eq!(lex.next(), Some(Ok(Token::Letter)));
        assert_eq!(lex.slice(), "u");
        assert_eq!(lex.next(), Some(Ok(Token::Letter)));
        assert_eq!(lex.slice(), "v");
        assert_eq!(lex.next(), Some(Ok(Token::Letter)));
        assert_eq!(lex.slice(), "w");
        assert_eq!(lex.next(), Some(Ok(Token::Letter)));
        assert_eq!(lex.slice(), "j");
        assert_eq!(lex.next(), Some(Ok(Token::NewLine)));

        assert_eq!(lex.next(), Some(Ok(Token::Letter)));
        assert_eq!(lex.slice(), "a");
        assert_eq!(lex.next(), Some(Ok(Token::Letter)));
        assert_eq!(lex.slice(), "b");
        assert_eq!(lex.next(), Some(Ok(Token::Letter)));
        assert_eq!(lex.slice(), "d");
        assert_eq!(lex.next(), Some(Ok(Token::Letter)));
        assert_eq!(lex.slice(), "e");
        assert_eq!(lex.next(), Some(Ok(Token::Letter)));
        assert_eq!(lex.slice(), "f");
        assert_eq!(lex.next(), Some(Ok(Token::Letter)));
        assert_eq!(lex.slice(), "g");
        assert_eq!(lex.next(), Some(Ok(Token::Letter)));
        assert_eq!(lex.slice(), "h");
        assert_eq!(lex.next(), Some(Ok(Token::Letter)));
        assert_eq!(lex.slice(), "i");
        assert_eq!(lex.next(), None);

    }

    #[test]
    fn test_create_create_height_map()
    {
        let string = aoc2022::read_to_string("tests/data/day12a.txt").unwrap();
        let mut lex: logos::Lexer<'_, Token> = <Token as logos::Logos>::lexer(&string);
    
        let height_map = create_height_map(&mut lex);

        assert_eq!(height_map,
        vec![
            vec![ 0, 1, 2,17,16,15,14,13],
            vec![ 1, 2, 3,18,25,24,24,12],
            vec![ 1, 3, 3,19,26,27,24,11],
            vec![ 1, 3, 3,20,21,22,23,10],
            vec![ 1, 2, 4, 5, 6, 7, 8, 9],
        ]
        );
    }

    #[test]
    fn test_find_start_position_example()
    {
        let string = aoc2022::read_to_string("tests/data/day12a.txt").unwrap();
        let mut lex: logos::Lexer<'_, Token> = <Token as logos::Logos>::lexer(&string);
    
        let height_map = create_height_map(&mut lex);
        let result = find_start_position(&height_map);

        assert_eq!(result, Some((0,0)));
        assert_eq!(height_map[0][0], 0);
    }

    #[test]
    fn test_find_start_position_first_task()
    {
        let string = aoc2022::read_to_string("tests/data/day12b.txt").unwrap();
        let mut lex: logos::Lexer<'_, Token> = <Token as logos::Logos>::lexer(&string);
    
        let height_map = create_height_map(&mut lex);
        let result = find_start_position(&height_map);

        assert_eq!(result, Some((20,0)));
        assert_eq!(height_map[20][0], 0);
    }

    #[test]
    fn test_get_legal_moves_example()
    {
        let string = aoc2022::read_to_string("tests/data/day12a.txt").unwrap();
        let mut lex: logos::Lexer<'_, Token> = <Token as logos::Logos>::lexer(&string);
    
        let height_map = create_height_map(&mut lex);
        let position = find_start_position(&height_map).unwrap();

        let path = Path::new(height_map, position.0, position.1, HashSet::new());
        assert_eq!(path.get_legal_moves(),
            vec![Position{x: 0,y: 1},Position {x: 1,y: 0}]
        );
    }

    #[test]
    fn test_get_legal_moves_first_task()
    {
        let string = aoc2022::read_to_string("tests/data/day12b.txt").unwrap();
        let mut lex: logos::Lexer<'_, Token> = <Token as logos::Logos>::lexer(&string);
    
        let height_map = create_height_map(&mut lex);
        let position = find_start_position(&height_map).unwrap();

        let path = Path::new(height_map, position.0,position.1,HashSet::new());
        assert_eq!(path.get_legal_moves(),
            vec![Position{x: 19,y:0},Position{x:21,y:0}]
        );
    }

    #[test]
    fn test_dfs_example()
    {
        let string = aoc2022::read_to_string("tests/data/day12a.txt").unwrap();
        let mut lex: logos::Lexer<'_, Token> = <Token as logos::Logos>::lexer(&string);
    
        let height_map = create_height_map(&mut lex);
        let position = find_start_position(&height_map).unwrap();

        let path = Path::new(height_map, position.0,position.1, HashSet::new());
        let best_path = path.dfs();
        assert!(best_path.is_some());
        assert_eq!(best_path.unwrap().len() - 1, 31);
    }

    // #[test]
    // fn test_dfs_first_task()
    // {
    //     let string = aoc2022::read_to_string("tests/data/day12b.txt").unwrap();
    //     let mut lex: logos::Lexer<'_, Token> = <Token as logos::Logos>::lexer(&string);
    
    //     let height_map = create_height_map(&mut lex);
    //     let position = find_start_position(&height_map).unwrap();

    //     let path = Path::new(height_map, position.0, position.1, HashSet::new());
    //     let best_path = path.dfs();
    //     assert!(best_path.is_some());
    //     assert_eq!(best_path.unwrap().len() - 1, 31);
    // }


}
