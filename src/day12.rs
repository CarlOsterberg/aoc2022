use logos::Logos;

use std::collections::HashSet;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[\t\f]+")]
pub enum Token {
    #[token("S")]
    Start,

    #[token("E")]
    End,

    #[regex(r"[\n]")]
    NewLine,

    #[regex(r"[a-z]")]
    Letter,
}

pub fn create_height_map(lex: &mut logos::Lexer<'_, Token>) -> Vec<Vec<usize>>
{
    let mut r_vec:Vec<Vec<usize>> = Vec::new();
    let mut i_vec:Vec<usize> = Vec::new();
    loop {
        match lex.next() {
            Some(Ok(Token::Start)) => i_vec.push(0),
            Some(Ok(Token::End)) => i_vec.push(27),
            Some(Ok(Token::NewLine)) => {
                r_vec.push(i_vec.clone());
                i_vec.clear();
            },
            Some(Ok(Token::Letter)) => i_vec.push(lex.slice().chars().nth(0).unwrap() as usize - 96),
            None => {
                r_vec.push(i_vec);    
                return r_vec
            },
            _=> panic!(),
        }
    }
}

pub fn find_start_position(height_map: &Vec<Vec<usize>>) -> Option<(usize, usize)>
{
    let mut i = 0;
    let mut j = 0;
    for vec in height_map {
        for pos in vec {
            if pos == &0 {
                return Some((i, j));
            }
            j+=1;
        }
        j = 0;
        i+=1;
    }
    None
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct Position
{
    pub x: usize,
    pub y: usize,
}

#[derive(Clone)]
pub struct Path
{
    height_map: Vec<Vec<usize>>,
    position: Position,
    visited: HashSet<Position>,
}

impl Path {

pub fn len(&self) -> usize
{
    self.visited.len()
}

pub fn new(height_map: Vec<Vec<usize>>, x: usize, y: usize, mut visited: HashSet<Position>) -> Path
{
    let position = Position {x, y};
    visited.insert(position.clone());
    Path { height_map, position, visited }
}

pub fn dfs(self) -> Option<Path>
{
    if self.height_map[self.position.x][self.position.y] == 27
    {
        return Some(self);
    }
    let moves = self.get_legal_moves();
    let mut best_path: Option<Path> = None;
    for position in moves {
        let path_result = Path::new(self.height_map.clone(), position.x, position.y, self.visited.clone()).dfs();
        match path_result.clone() {
            Some(some) => {
                match best_path {
                    Some(ref current_best) => {
                        if current_best.len() > some.len() {
                            best_path = path_result;
                        }
                    },
                    None => {
                        best_path = path_result;
                    }
                }
            },
            None => (),
        }
    }
    best_path
}   

pub fn get_legal_moves(&self) -> Vec<Position>
{
    let mut vec:Vec<Position> = Vec::new();

    // Is down legal?
    if self.position.y > 0 {
        let pos = Position {x: self.position.x, y: self.position.y-1};
        if !self.is_visited(&pos) && self.is_legal_move(&pos){
            vec.push(pos);
        }
    }
    if self.position.y < self.height_map[0].len() - 1 {
        let pos = Position {x: self.position.x, y: self.position.y+1};
        if !self.is_visited(&pos) && self.is_legal_move(&pos){
            vec.push(pos);
        }
    }
    if self.position.x > 0 {
        let pos = Position {x: self.position.x-1, y: self.position.y};
        if !self.is_visited(&pos) && self.is_legal_move(&pos){
            vec.push(pos);
        }
    }
    if self.position.x < self.height_map.len() - 1 {
        let pos = Position{x: self.position.x+1, y: self.position.y};
        if !self.is_visited(&pos) && self.is_legal_move(&pos){
            vec.push(pos);
        }
    }
    vec
}

fn is_legal_move(&self, position: &Position) -> bool
{
    if self.height_map[position.x][position.y] as i32 - 
    self.height_map[self.position.x][self.position.y] as i32 <= 1
    {
        true
    }
    else {
        false
    }
}

fn is_visited(&self, position: &Position) -> bool
{
    for element in &self.visited
    {
        if element.x == position.x && element.y == position.y {
            return true;
        }
    }
    false
}

}
