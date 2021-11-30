use std::time;

mod day1;

fn main() {
    let start = time::Instant::now();

    day1::run();

    println!("--------------\nExecution took {:?}", start.elapsed());
}
