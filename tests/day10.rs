use aoc2022;


#[cfg(test)]
mod tests {
    use aoc2022::day10::*;

    use super::*;
    
    #[test]
    fn test_tokenizing()
    {
        let string = aoc2022::read_to_string("tests/data/day10a.txt").unwrap();
        let mut lex: logos::Lexer<'_, Token> = <Token as logos::Logos>::lexer(&string);
    
        assert_eq!(lex.next(), Some(Ok(Token::Addx)));
        assert_eq!(lex.next(), Some(Ok(Token::Number)));
        assert_eq!(lex.slice(), "15");

        assert_eq!(lex.next(), Some(Ok(Token::Addx)));
        assert_eq!(lex.next(), Some(Ok(Token::Negative)));
        assert_eq!(lex.next(), Some(Ok(Token::Number)));
        assert_eq!(lex.slice(), "11");

        assert_eq!(lex.next(), Some(Ok(Token::Addx)));
        assert_eq!(lex.next(), Some(Ok(Token::Number)));
        assert_eq!(lex.slice(), "6");

        assert_eq!(lex.next(), Some(Ok(Token::Addx)));
        assert_eq!(lex.next(), Some(Ok(Token::Negative)));
        assert_eq!(lex.next(), Some(Ok(Token::Number)));
        assert_eq!(lex.slice(), "3");

        assert_eq!(lex.next(), Some(Ok(Token::Addx)));
        assert_eq!(lex.next(), Some(Ok(Token::Number)));
        assert_eq!(lex.slice(), "5");

        assert_eq!(lex.next(), Some(Ok(Token::Addx)));
        assert_eq!(lex.next(), Some(Ok(Token::Negative)));
        assert_eq!(lex.next(), Some(Ok(Token::Number)));
        assert_eq!(lex.slice(), "1");

        assert_eq!(lex.next(), Some(Ok(Token::Addx)));
        assert_eq!(lex.next(), Some(Ok(Token::Negative)));
        assert_eq!(lex.next(), Some(Ok(Token::Number)));
        assert_eq!(lex.slice(), "8");

        assert_eq!(lex.next(), Some(Ok(Token::Addx)));
        assert_eq!(lex.next(), Some(Ok(Token::Number)));
        assert_eq!(lex.slice(), "13");

        assert_eq!(lex.next(), Some(Ok(Token::Addx)));
        assert_eq!(lex.next(), Some(Ok(Token::Number)));
        assert_eq!(lex.slice(), "4");

        assert_eq!(lex.next(), Some(Ok(Token::Noop)));

    }

    #[test]
    fn test_cathode_ray_tube()
    {
        let string = aoc2022::read_to_string("tests/data/day10a.txt").unwrap();
        let mut lex: logos::Lexer<'_, Token> = <Token as logos::Logos>::lexer(&string);

        let vec = cathode_ray_tube(&mut lex);

        assert_eq!(vec[20], 21);
        assert_eq!(vec[60], 19);
        assert_eq!(vec[100], 18);
        assert_eq!(vec[140], 21);
        assert_eq!(vec[180], 16);
        assert_eq!(vec[220], 18);

        let mut accum:isize = 0;
        for value in (20..=220).step_by(40) {
            accum += vec[value] * value as isize;
        };

        assert_eq!(accum, 13140);
    }

    #[test]
    fn test_first_task()
    {
        let string = aoc2022::read_to_string("tests/data/day10b.txt").unwrap();
        let mut lex: logos::Lexer<'_, Token> = <Token as logos::Logos>::lexer(&string);

        let vec = cathode_ray_tube(&mut lex);

        let mut accum:isize = 0;
        for value in (20..=220).step_by(40) {
            accum += vec[value] * value as isize;
        };

        assert_eq!(accum, 14920);
    }

    #[test]
    fn test_draw_sprite()
    {
        let string = aoc2022::read_to_string("tests/data/day10a.txt").unwrap();
        let mut lex: logos::Lexer<'_, Token> = <Token as logos::Logos>::lexer(&string);

        let vec = cathode_ray_tube(&mut lex);

        let mut row:usize = 0;

        for cycle in 1..=240 {
            if (cycle-row-1) as isize >= (vec[cycle]-1) && (cycle-row-1) as isize <= (vec[cycle]+1)
            {
                print!("#");
            }
            else {
                print!(".");
            };
            if cycle % 40 == 0 {
                row+=40;
                println!();
            }
        };
    }

    #[test]
    fn test_draw_letters()
    {
        let string = aoc2022::read_to_string("tests/data/day10c.txt").unwrap();
        let mut lex: logos::Lexer<'_, Token> = <Token as logos::Logos>::lexer(&string);

        let vec = cathode_ray_tube(&mut lex);

        let mut row:usize = 0;

        for cycle in 1..=240 {
            if (cycle-row-1) as isize >= (vec[cycle]-1) && (cycle-row-1) as isize <= (vec[cycle]+1)
            {
                print!("#");
            }
            else {
                print!(".");
            };
            if cycle % 40 == 0 {
                row+=40;
                println!();
            }
        };
    }

}
