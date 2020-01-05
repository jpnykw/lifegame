use std::{thread, time};
use std::time::Duration;

mod generate;
mod update;
mod print;
mod count;

fn main() {
    let mut stage = generate::new(200, 50);

    // glider
    stage[2][0] = true;
    stage[2][1] = true;
    stage[2][2] = true;
    stage[1][2] = true;
    stage[0][1] = true;

    loop {
        print::new(&stage);
        stage = update::run(&stage);
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 5));
    }
}
