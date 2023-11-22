use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[token("addx")]
    Addx,

    #[token("noop")]
    Noop,

    #[token("-")]
    Negative,

    #[regex(r"[0-9]+")]
    Number,
}

pub fn cathode_ray_tube(lex: &mut logos::Lexer<'_, Token>) -> Vec<isize>
{
    let mut vec:Vec<isize> = vec![1];
    let mut register:isize = 1;

    loop {
        match lex.next() {
            Some(Ok(Token::Addx))=> {
                match lex.next() {
                    Some(Ok(Token::Negative)) => {
                        match lex.next() {
                            Some(Ok(Token::Number)) => {
                                vec.push(register);
                                vec.push(register);
                                register -= lex.slice().parse::<isize>().unwrap();
                            },
                            _ => panic!(),
                        }
                    },
                    Some(Ok(Token::Number)) => {
                        vec.push(register);
                        vec.push(register);
                        register += lex.slice().parse::<isize>().unwrap();
                    },
                    _ => panic!(),
                }
            },
            Some(Ok(Token::Noop)) => vec.push(register),
            None => return vec,
            _=> panic!(),
        }
    }
}