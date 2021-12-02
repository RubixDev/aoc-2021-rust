use std::time;

mod day1;
mod day2;

fn main() {
    let start = time::Instant::now();

    day1::run();
    day2::run();

    println!("--------------\nExecution took {:?}", start.elapsed());
}
