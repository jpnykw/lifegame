use std::{thread, time};
use std::time::Duration;
use std::io::prelude::*;
use std::fs::File;
use std::env;

mod generate;
mod update;
mod print;
mod count;

fn main() {
    // lifegame
    let mut stage = generate::new(90, 30);

    // file IO
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let mut file = File::open(&args[1]).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        let mut token = 0;
        let mut dx = String::new();
        let mut dy = String::new();
        let mut text = String::new();

        for c in data.chars() {
            if c == '\n' {
                match token {
                    0 => dx = text,
                    1 => dy = text,
                    _ => {
                        let mut x = 0;
                        for mode in text.to_string().chars() {
                            stage[token - 2 + dy.parse::<usize>().unwrap()][x + dx.parse::<usize>().unwrap()] = if mode == '1' { true } else { false };
                            x += 1;
                        }
                    }
                }

                token += 1;
                text = String::new();
                // println!("--");
            } else {
                text = format!("{}{}", text, c);
            }
        }
    }

    // start
    loop {
        print::new(&stage);
        stage = update::run(&stage);
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 5));
    }
}

