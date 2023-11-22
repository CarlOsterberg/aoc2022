use aoc2022;


#[cfg(test)]
mod tests {
    use aoc2022::day7::{*, self};

    use super::*;

    #[test]
    fn parse_to_tokens()
    {
        let string = aoc2022::read_to_string("tests/data/day7a.txt").unwrap();
        let mut lex: logos::Lexer<'_, Token> = <Token as logos::Logos>::lexer(&string);

        assert_eq!(lex.next(), Some(Ok(Token::Cd)));
        assert_eq!(lex.next(), Some(Ok(Token::Name)));
        assert_eq!(lex.slice(), "/");
        

        assert_eq!(lex.next(), Some(Ok(Token::Ls)));
        assert_eq!(lex.next(), Some(Ok(Token::Dir)));
        assert_eq!(lex.next(), Some(Ok(Token::Name)));
        assert_eq!(lex.slice(), "a");
        assert_eq!(lex.next(), Some(Ok(Token::Size)));
        assert_eq!(lex.slice(), "14848514");
        assert_eq!(lex.next(), Some(Ok(Token::Name)));
        assert_eq!(lex.slice(), "b.txt");
        assert_eq!(lex.next(), Some(Ok(Token::Size)));
        assert_eq!(lex.slice(), "8504156");
        assert_eq!(lex.next(), Some(Ok(Token::Name)));
        assert_eq!(lex.slice(), "c.dat");
        assert_eq!(lex.next(), Some(Ok(Token::Dir)));
        assert_eq!(lex.next(), Some(Ok(Token::Name)));
        assert_eq!(lex.slice(), "d");

        assert_eq!(lex.next(), Some(Ok(Token::Cd)));
        assert_eq!(lex.next(), Some(Ok(Token::Name)));
        assert_eq!(lex.slice(), "a");

        assert_eq!(lex.next(), Some(Ok(Token::Ls)));
        assert_eq!(lex.next(), Some(Ok(Token::Dir)));
        assert_eq!(lex.next(), Some(Ok(Token::Name)));
        assert_eq!(lex.slice(), "e");
        assert_eq!(lex.next(), Some(Ok(Token::Size)));
        assert_eq!(lex.slice(), "29116");
        assert_eq!(lex.next(), Some(Ok(Token::Name)));
        assert_eq!(lex.slice(), "f");
        assert_eq!(lex.next(), Some(Ok(Token::Size)));
        assert_eq!(lex.slice(), "2557");
        assert_eq!(lex.next(), Some(Ok(Token::Name)));
        assert_eq!(lex.slice(), "g");
        assert_eq!(lex.next(), Some(Ok(Token::Size)));
        assert_eq!(lex.slice(), "62596");
        assert_eq!(lex.next(), Some(Ok(Token::Name)));
        assert_eq!(lex.slice(), "h.lst");

        assert_eq!(lex.next(), Some(Ok(Token::Cd)));
        assert_eq!(lex.next(), Some(Ok(Token::Name)));
        assert_eq!(lex.slice(), "e");

        assert_eq!(lex.next(), Some(Ok(Token::Ls)));
        assert_eq!(lex.next(), Some(Ok(Token::Size)));
        assert_eq!(lex.slice(), "584");
        assert_eq!(lex.next(), Some(Ok(Token::Name)));
        assert_eq!(lex.slice(), "i");

        assert_eq!(lex.next(), Some(Ok(Token::Cd)));
        assert_eq!(lex.next(), Some(Ok(Token::Back)));
    
        assert_eq!(lex.next(), Some(Ok(Token::Cd)));
        assert_eq!(lex.next(), Some(Ok(Token::Back)));

        assert_eq!(lex.next(), Some(Ok(Token::Cd)));
        assert_eq!(lex.next(), Some(Ok(Token::Name)));
        assert_eq!(lex.slice(), "d");

        assert_eq!(lex.next(), Some(Ok(Token::Ls)));
        assert_eq!(lex.next(), Some(Ok(Token::Size)));
        assert_eq!(lex.slice(), "4060174");
        assert_eq!(lex.next(), Some(Ok(Token::Name)));
        assert_eq!(lex.slice(), "j");
        assert_eq!(lex.next(), Some(Ok(Token::Size)));
        assert_eq!(lex.slice(), "8033020");
        assert_eq!(lex.next(), Some(Ok(Token::Name)));
        assert_eq!(lex.slice(), "d.log");
        assert_eq!(lex.next(), Some(Ok(Token::Size)));
        assert_eq!(lex.slice(), "5626152");
        assert_eq!(lex.next(), Some(Ok(Token::Name)));
        assert_eq!(lex.slice(), "d.ext");
        assert_eq!(lex.next(), Some(Ok(Token::Size)));
        assert_eq!(lex.slice(), "7214296");
        assert_eq!(lex.next(), Some(Ok(Token::Name)));
        assert_eq!(lex.slice(), "k");

        assert_eq!(lex.next(), None);

    }

    #[test]
    fn check_first_task_example()
    {
        let string = aoc2022::read_to_string("tests/data/day7a.txt").unwrap();
        let mut lex: logos::Lexer<'_, Token> = <Token as logos::Logos>::lexer(&string);

        let dirs = day7::get_dirs_as_hashmap(&mut lex, "TOP_OF_ZE_RECURSION".to_owned());

        assert_eq!(day7::sum_of_dirs_less_then_cond(dirs, 100000), 95437);
    }

    #[test]
    fn first_task()
    {
        let string = aoc2022::read_to_string("tests/data/day7b.txt").unwrap();
        let mut lex: logos::Lexer<'_, Token> = <Token as logos::Logos>::lexer(&string);

        let dirs = day7::get_dirs_as_hashmap(&mut lex, "TOP_OF_ZE_RECURSION".to_owned());

        assert_eq!(day7::sum_of_dirs_less_then_cond(dirs, 100000), 1407786);
    }


}
