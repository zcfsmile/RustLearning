#![allow(dead_code)]
fn main() {

}

struct Queue {
    older: Vec<char>,
    younger: Vec<char>
}

impl Queue {
    fn new() -> Self {
        Queue {
            older: Vec::new(),
            younger: Vec::new()
        }
    }

    fn push(self: &mut Self, c: char) {
        self.younger.push(c);
    }

    fn pop(&mut self) -> Option<char> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }
            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }
        self.older.pop()
    }
}

