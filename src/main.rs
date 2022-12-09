use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    day1_part1();
    day1_part2();

    day2_part1();
    day2_part2()
}

fn day2_part1() {
    if let Ok(lines) = read_lines("src/day2.txt") {
        let score: i32 = lines.map(|l| score_part1(l.unwrap())).sum();
        println!("Result Day2 Part1 {}", score)
    }
}

fn day2_part2() {
    if let Ok(lines) = read_lines("src/day2.txt") {
        let score: i32 = lines.map(|l| score_part2(l.unwrap())).sum();
        println!("Result Day2 Part2 {}", score)
    }
}


fn score_part2(s: String) -> i32 {
    match &*s {
        "A Y" => 1 + 3,
        "A X" => 3 + 0,
        "A Z" => 2 + 6,
        "B Y" => 2 + 3,
        "B X" => 1 + 0,
        "B Z" => 3 + 6,
        "C Y" => 3 + 3,
        "C X" => 2 + 0,
        "C Z" => 1 + 6,
        _ => 0
    }
}

fn score_part1(s: String) -> i32 {
    match &*s {
        "A Y" => 2 + 6,
        "A X" => 1 + 3,
        "A Z" => 3 + 0,
        "B Y" => 2 + 3,
        "B X" => 1 + 0,
        "B Z" => 3 + 6,
        "C Y" => 2 + 0,
        "C X" => 1 + 6,
        "C Z" => 3 + 3,
        _ => 0
    }
}

fn day1_part1() {
    let calories = read_to_vec();
    println!("Result Day1 Part1 {}", calories[calories.len() - 1]);
}

fn day1_part2() {
    let r = read_to_vec();
    println!("Result Day1 Part2 {}, ", r[r.len() - 1] + r[r.len() - 2] + r[r.len() - 3]);
}

fn read_to_vec() -> Vec<i32> {
    let mut r = Vec::new();
    let mut elf = 0;
    if let Ok(lines) = read_lines("src/day1.txt") {
        for line in lines {
            if let Ok(calories) = line {
                if calories.chars().count() == 0 {
                    r.push(elf);
                    elf = 0;
                } else {
                    elf = elf + calories.parse::<i32>().unwrap();
                }
            }
        }
    }
    r.sort();
    r
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}