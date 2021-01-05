enum Maybe<A> {
    Just(A),
    Nothing,
}

use std::fmt::Display;
use std::fmt::Formatter;

impl<A: Display> Display for Maybe<A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Maybe::Just(a) => write!(f, "{}", a),
            _ => write!(f, ""),
        }
    }
}

fn main() {
    let x = Maybe::Just("Poo");
    let y = Maybe::Nothing::<i32>;
    println!("Hello, {}, {}, world!", x, y);
}

/*
pub struct List<A> {
    head: Link<A>,
}

enum Link<A> {
    Empty,
    More(Box<Node<A>>),
}

struct Node<A> {
    elem: A,
    next: Link<A>,
}

impl<A> List<A> {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
}
*/
