pub fn run() {
    println!("--- DAY 01 ---");
    let input: Vec<u16> = include_str!("resources/input01.txt")
        .split('\n')
        .map(|num| num.parse::<u16>().unwrap())
        .collect();
    part1(&input);
    println!();
    part2(&input);
    println!();
}

fn part1(input: &Vec<u16>) {
    println!(
        "{}",
        input
            .iter()
            .enumerate()
            .filter(|(index, num)| index != &0 && num > &&input[index - 1])
            .count()
    );
}

fn of_index(input: &Vec<u16>, index: usize) -> u16 {
    if index > input.len() - 3 {
        0
    } else {
        input[index] + input[index + 1] + input[index + 2]
    }
}

fn part2(input: &Vec<u16>) {
    println!(
        "{}",
        input
            .iter()
            .enumerate()
            .filter(|(index, _)| index != &0 && of_index(input, *index) > of_index(input, index - &1))
            .count()
    );
}
