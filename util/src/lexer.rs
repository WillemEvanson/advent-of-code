use std::str::Chars;

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    initial_len: usize,
    chars: Chars<'a>,
}

impl<'a> Lexer<'a> {
    #[inline]
    pub fn new(input: &'a str) -> Self {
        Self {
            initial_len: input.len(),
            chars: input.chars(),
        }
    }

    #[inline]
    pub fn eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    #[inline]
    pub fn consumed(&self) -> usize {
        self.initial_len - self.chars.as_str().len()
    }

    #[inline]
    pub fn peek(&self) -> char {
        self.chars.clone().next().unwrap_or('\0')
    }

    #[inline]
    pub fn bump(&mut self) -> char {
        self.chars.next().unwrap_or('\0')
    }

    #[inline]
    pub fn at(&self, c: char) -> bool {
        self.peek() == c
    }

    #[inline]
    pub fn eat(&mut self, c: char) -> bool {
        if self.at(c) {
            self.bump();
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn eat_while<F: FnMut(char) -> bool>(&mut self, mut f: F) {
        while f(self.peek()) {
            self.bump();
        }
    }
}
