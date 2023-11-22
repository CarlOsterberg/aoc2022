use std::{collections::HashMap, usize};
use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[token("$ cd")]
    Cd,

    #[token("$ ls")]
    Ls,

    #[regex(r"[0-9]+")]
    Size,

    #[regex(r"[a-zA-Z|\\/]+(\.[a-zA-Z]+){0,1}")]
    Name,

    #[regex(r"dir")]
    Dir,

    #[token("..")]
    Back,
}

pub fn get_dirs_as_hashmap(lex: &mut logos::Lexer<'_, Token>, dir_name: String) -> HashMap<String, usize>
{
    let mut size:usize = 0;
    let mut directories:HashMap<String, usize> = HashMap::new();
    let mut children: Vec<String> = Vec::new();
    loop {
        match lex.next() {
            Some(Ok(Token::Cd)) =>
            {
                match lex.next() {
                    Some(Ok(Token::Name)) =>{
                        for (key, value) in get_dirs_as_hashmap(lex, lex.slice().to_owned()).into_iter()
                        {
                            if directories.contains_key(&key) {
                                let unique_name = "_".to_string() + &key;
                                directories.insert(unique_name, value);
                            }
                            else {
                                directories.insert(key, value);   
                            }
                        }
                    },
                    Some(Ok(Token::Back)) => {
                        for child in children {
                            match directories.get(&child) {
                                Some(value) => size += value,
                                None => panic!(),
                            }
                        }

                        directories.insert(dir_name, size);
                        return directories;
                    },
                    _ => panic!()
                }
            },
            Some(Ok(Token::Ls)) =>
            {
                ()
            },
            Some(Ok(Token::Size)) =>
            {
                size += lex.slice().parse::<usize>().unwrap();
                lex.next();
            }
            Some(Ok(Token::Dir)) =>
            {
                match lex.next() {
                    Some(Ok(Token::Name)) => 
                    {
                        children.push(lex.slice().to_owned());
                    },
                    _ => panic!()
                }
            }
            None => {
                if dir_name == "TOP_OF_ZE_RECURSION" {
                    return directories;
                }
                for child in children {
                    match directories.get(&child) {
                        Some(value) => size += value,
                        None => panic!(),
                    }
                }

                if directories.contains_key(&dir_name) {
                    let unique_name = "_".to_string() + &dir_name;
                    directories.insert(unique_name, size);
                }
                else {
                    directories.insert(dir_name, size);   
                }

                return directories;
            }
            _ => panic!(),
        }
    }
}    

pub fn sum_of_dirs_less_then_cond(map: HashMap<String, usize>, condition: usize) -> usize
{
    let mut sum: usize = 0;
    for (_, value) in map.into_iter() {
        if value <= condition {
            sum += value;
        }
    }
    sum
}