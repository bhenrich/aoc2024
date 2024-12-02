use std::fs;

fn main() {
    println!("Advent of Code 2024 - by yunii");
    println!("Please enter the day number: ");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input: i16 = input.trim().parse().unwrap();

    match input {
        1 => day1::run(),
        /*
        2 => day2::run(),
        3 => day3::run(),
        4 => day4::run(),
        5 => day5::run(),
        6 => day6::run(),
        7 => day7::run(),
        8 => day8::run(),
        9 => day9::run(),
        10 => day10::run(),
        11 => day11::run(),
        12 => day12::run(),
        13 => day13::run(),
        14 => day14::run(),
        15 => day15::run(),
        16 => day16::run(),
        17 => day17::run(),
        18 => day18::run(),
        19 => day19::run(),
        20 => day20::run(),
        21 => day21::run(),
        22 => day22::run(),
        23 => day23::run(),
        24 => day24::run(),
        25 => day25::run(),
        */
        _ => println!("Day not found!"),
    }
}

fn read_input(day: i16) -> String {
    let path = format!("src/day{}/input.txt", day);
    fs::read_to_string(path).unwrap()
}
/*
mod sample {
    use super::*;

    pub fn run() {
        let input = read_input(1);

        // Code here
    }
}
*/
mod day1 {
    use super::*;

    pub fn run() {
        let input = read_input(1);

        // PART 1

        let mut left: Vec<i32> = Vec::new();
        let mut right: Vec<i32> = Vec::new();
        let mut distances: Vec<i32> = Vec::new();

        for line in input.lines() {
            let mut parts = line.split_whitespace();
            left.push(parts.next().unwrap().parse().unwrap());
            right.push(parts.next().unwrap().parse().unwrap());
        }

        let mut left_sorted = left.clone();
        left_sorted.sort();

        let mut right_sorted = right.clone();
        right_sorted.sort();

        for i in 0..left.len() {
            distances.push((left_sorted[i] - right_sorted[i]).abs());
        }

        let distance = distances.iter().sum::<i32>();

        println!("P1 Distance: {}", distance);

        // PART 2

        let mut scores: Vec<i64> = Vec::new();

        for i in 0..left.len() {
            let mut count: i32 = 0;

            for j in 0..right.len() {
                if left_sorted[i] == right_sorted[j] {
                    count = count + 1;
                }

                scores.push((left_sorted[i] * count).into());
            }
        }

        let score: i64 = scores.iter().sum();

        println!("P2 Score: {}", score);
    }
}