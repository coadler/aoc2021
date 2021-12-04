#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref INPUT: Vec<&'static str> = include_str!("day_three").lines().collect();
}

fn main() {
    println!("part 1 output is {}", part1());
}

fn part1() -> i64 {
    let len = INPUT.len();
    let mut num_ones: [usize; 12] = [0; 12];

    for line in INPUT.iter() {
        for (i, char) in line.char_indices() {
            if char == '1' {
                num_ones[i] += 1
            }
        }
    }

    let mut gamma_bits: [u8; 12] = [0; 12];
    let mut epsilon_bits: [u8; 12] = [0; 12];

    for (i, ones) in num_ones.iter().enumerate() {
        if (len - ones) < *ones {
            gamma_bits[i] = 1
        }
    }

    for (i, e) in gamma_bits.iter().enumerate() {
        if *e == 0 {
            epsilon_bits[i] = 1
        }
    }

    (from_bits(gamma_bits) * from_bits(epsilon_bits))
        .try_into()
        .unwrap()
}

fn part2() -> i64 {
    for line in INPUT.iter() {}

    0
}

fn from_bits<const N: usize>(bits: [u8; N]) -> u64 {
    let mut out = 0;
    for bit in bits {
        out <<= 1;
        out ^= bit as u64;
    }

    out
}
