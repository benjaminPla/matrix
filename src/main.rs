use rand::distributions::Alphanumeric;
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use std::io::{self, Write};
extern crate termsize;

#[derive(Debug)]
struct Char {
    row: u16,
    col: u16,
    ch: char,
}

impl Char {
    fn fall(&mut self, rows: u16, mut rng: ThreadRng) {
        let ch: char = rng.sample(Alphanumeric).into();
        self.ch = ch;
        match self.row == rows {
            true => self.row = 1,
            false => self.row += 1,
        }
    }
}

fn main() {
    print!("{}[2J", 27 as char);

    let mut rng = thread_rng();
    let term_size = termsize::get().unwrap();

    let mut chars: Vec<Char> = Vec::new();

    loop {
        let random_col: u16 = rng.gen_range(0..term_size.cols);
        let ch: char = rng.sample(Alphanumeric).into();
        chars.push(Char {
            row: 0,
            col: random_col,
            ch,
        });

        for ch in chars.iter_mut() {
            ch.fall(term_size.rows, rng.clone());
            print!("\x1B[32m\x1B[{};{}H{}\x1B[0m", ch.row, ch.col, ch.ch);
        }

        io::stdout().flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
