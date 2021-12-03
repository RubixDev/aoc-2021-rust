use std::time;

mod day1;
mod day2;
mod day3;

fn main() {
    let start = time::Instant::now();

    day1::run();
    day2::run();
    day3::run();

    println!("--------------\nExecution took {:?}", start.elapsed());
}
