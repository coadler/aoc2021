#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref INPUT: Vec<i32> = include_str!("day_one")
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();
}

fn main() {
    let count_part1 = INPUT
        .windows(2)
        .fold(0, |count, v| if v[0] < v[1] { count + 1 } else { count });

    println!("part 1 total is {}", count_part1);

    let count_part2 = INPUT
        .windows(3)
        .map(|i| [i[0], i[1], i[2]])
        .collect::<Vec<_>>()
        .windows(2)
        .fold(0, |count, v| {
            if v[0].iter().sum::<i32>() < v[1].iter().sum::<i32>() {
                count + 1
            } else {
                count
            }
        });

    println!("part 2 total is {}", count_part2);
}
