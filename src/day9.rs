use std::collections::HashMap;

use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[token("R")]
    Right,

    #[token("U")]
    Up,

    #[token("L")]
    Left,

    #[token("D")]
    Down,

    #[regex(r"[0-9]+")]
    Steps,
}

pub struct Grid
{
    head: Pos,
    tail: Pos,
    visited: HashMap<Pos, usize>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Pos
{
    x: i128,
    y: i128,
}

impl Pos
{
    pub fn new(x: i128, y:i128) -> Pos
    {
        Pos {x, y}
    }
}

impl Grid
{
    pub fn new(x: i128, y: i128) -> Grid
    {
        let mut grid = Grid {
            head: Pos::new(x, y),
            tail: Pos::new(x, y),
            visited: HashMap::new(),
        };
        grid.increment(x, y);
        grid
    }

    pub fn increment(&mut self, x: i128, y: i128)
    {
        self.visited.insert(Pos::new(x, y), 1);
    }

    pub fn move_head(&mut self, direction: Token, steps: usize)
    {
        match direction {
            Token::Right =>
            for _ in 0..steps {
                self.head.y += 1;
                self.move_tail();
            },
            Token::Up => 
            for _ in 0..steps {
                self.head.x -= 1;
                self.move_tail();
            },
            Token::Left => 
            for _ in 0..steps {
                self.head.y -= 1;
                self.move_tail();
            },
            Token::Down =>
            for _ in 0..steps {
                self.head.x += 1;
                self.move_tail();
            },
            _ => panic!(),
        }
    }

    pub fn move_tail(&mut self)
    {
        let distance = ((self.head.x as f64 - self.tail.x as f64 ).powf(2.0) + (self.head.y as f64 - self.tail.y as f64).powf(2.0)).sqrt();
        if distance < 2.0 {
            return;
        }
        let anotha_one = if distance > 2.0
        {
            true
        }
        else {
            false
        };

        
        if self.head.y - self.tail.y > 1 {
            self.tail.y += 1;
            if anotha_one
            {
                self.tail.x = self.head.x;
            }
        }
        if self.head.y - self.tail.y < -1 {
            self.tail.y -= 1;
            if anotha_one
            {
                self.tail.x = self.head.x;
            }
        }
        if self.head.x - self.tail.x > 1 {
            self.tail.x += 1;
            if anotha_one
            {
                self.tail.y = self.head.y;
            }
        }
        if self.head.x - self.tail.x < -1 {
            self.tail.x -= 1;
            if anotha_one
            {
                self.tail.y = self.head.y;
            }
        }   
        
        self.increment(self.tail.y, self.tail.x);
    }

    pub fn head_pos(&self) -> &Pos
    {
        &self.head
    }

    pub fn tail_pos(&self) -> &Pos
    {
        &self.tail
    }

    pub fn perform_moves(&mut self, lex: &mut logos::Lexer<'_, Token>)
    {
        loop {
            match lex.next() {
                Some(Ok(direction)) => {
                    match lex.next() {
                        Some(Ok(Token::Steps)) =>
                        {
                            self.move_head(direction, lex.slice().parse::<usize>().unwrap())
                        }
                        _ => panic!()
                    }
                },
                None => return,
                _ => panic!()
            }
            
        }
    }

    pub fn count_unique_visits(&self) -> usize
    {
        self.visited.len()
    }
}
