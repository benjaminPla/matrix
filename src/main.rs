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
    fn fall(&mut self, mut rng: ThreadRng) {
        let ch: char = rng.sample(Alphanumeric).into();
        self.ch = ch;
        self.row += 1;
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
            ch.fall(rng.clone());
            print!("\x1B[{};{}H{}", ch.row, ch.col, ch.ch);
        }

        io::stdout().flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
