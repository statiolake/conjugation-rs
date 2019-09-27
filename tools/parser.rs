use std::collections::HashMap;
use std::env;
use std::fs;
use std::iter::Peekable;

#[derive(Debug)]
pub struct CType {
    name: String,
    forms: HashMap<String, Vec<String>>,
}

struct Parser<P> {
    state: P,
}

impl<I: Iterator<Item = char>> Parser<Peekable<I>> {
    pub fn new(state: Peekable<I>) -> Parser<Peekable<I>> {
        Parser { state }
    }

    fn eat(&mut self, ch: char) {
        assert_eq!(self.state.next(), Some(ch))
    }

    fn skip(&mut self) {
        self.state.next().expect("unexpected end of file");
    }

    fn peek(&mut self) -> char {
        *self.state.peek().expect("unexpected end of file")
    }

    fn skip_whitespace(&mut self) {
        while let Some(true) = self.state.peek().map(|c| c.is_whitespace()) {
            self.skip();
        }
    }

    fn is_finished(&mut self) -> bool {
        self.skip_whitespace();
        self.state.peek().is_none()
    }

    pub fn parse_word(&mut self) -> String {
        let mut word = String::new();
        self.skip_whitespace();
        loop {
            let ch = self.peek();
            if ch == ')' || ch.is_whitespace() {
                return word;
            } else {
                self.skip();
                word.push(ch);
            }
        }
    }

    pub fn parse_cform(&mut self) -> (String, String) {
        self.skip_whitespace();
        self.eat('(');

        let name = self.parse_word();
        let form = self.parse_word();
        let _ = self.parse_word();

        self.skip_whitespace();
        if self.peek() != ')' {
            let _ = self.parse_word();
        }

        self.skip_whitespace();
        self.eat(')');

        (name, form)
    }

    pub fn parse_ctype(&mut self) -> CType {
        self.skip_whitespace();
        self.eat('(');
        let name = self.parse_word();
        eprintln!("reading {}", name);
        let mut forms = HashMap::new();
        loop {
            self.skip_whitespace();
            match self.peek() {
                ')' => break,
                '(' => {
                    let (name, form) = self.parse_cform();
                    forms.entry(name).or_insert_with(|| Vec::new()).push(form);
                }
                _ => panic!("unexpected word occurence"),
            }
        }
        self.skip_whitespace();
        self.eat(')');
        CType { name, forms }
    }
}

pub fn print_match(ctypes: &[CType]) {
    println!("match #matcher# {{");
    for ctype in ctypes {
        for (name, cforms) in &ctype.forms {
            println!(
                "    Conjugation::#{ctype}#(#{ctype}#::#{cform}#) => &{form:?} as &[&'static str],",
                ctype = ctype.name,
                cform = name,
                form = cforms
            );
        }
    }
    println!("}}")
}

fn main() {
    let file_name = env::args().nth(1).unwrap();
    let content = fs::read_to_string(file_name).unwrap();
    let mut parser = Parser::new(content.chars().peekable());

    let mut ctypes = Vec::new();
    while !parser.is_finished() {
        ctypes.push(parser.parse_ctype());
    }

    print_match(&ctypes);
}
