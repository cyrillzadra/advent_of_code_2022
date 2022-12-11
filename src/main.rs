use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    day1_part1();
    day1_part2();

    day2_part1();
    day2_part2();

    day3_part1();
}

/*https://adventofcode.com/2022/day/3 */

fn day3_part1() {
    if let Ok(lines) = read_lines("src/day3.txt") {
        let sum: i32 = lines.map(|line| xxx(line.unwrap())).sum();
        println!("Result Day3 Part1 {}", sum);
        assert_eq!(sum, 7980);
    }
}

fn xxx(line: String) -> i32 {
    let mut characters: HashMap<char, i32> = HashMap::new();
    let chars = 'a'..='z';
    for (i, c) in chars.enumerate() {
        characters.insert(c, i as i32 + 1);
    }
    let chars = 'A'..='Z';
    for (i, c) in chars.enumerate() {
        characters.insert(c, 26 + i as i32 + 1);
    }

    let (part1, part2) = line.split_at(line.len() / 2);
    let mut map1 = HashMap::new();
    for s in part1.chars() {
        map1.entry(s).and_modify(|v| *v += 1).or_insert(1);
    }
    for (_i, s) in part2.chars().enumerate() {
        if map1.contains_key(&s) {
            match characters.get(&s) {
                Some(&number) => {
                    return number
                }
                _ => 0,
            };
        }
    }
    0
}

fn day2_part1() {
    if let Ok(lines) = read_lines("src/day2.txt") {
        let score: i32 = lines.map(|l| score_part1(l.unwrap())).sum();
        println!("Result Day2 Part1 {}", score);
        assert_eq!(score, 12740);
    }
}

fn day2_part2() {
    if let Ok(lines) = read_lines("src/day2.txt") {
        let score: i32 = lines.map(|l| score_part2(l.unwrap())).sum();
        println!("Result Day2 Part2 {}", score);
        assert_eq!(score, 11980);
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
    let res = calories[calories.len() - 1];
    println!("Result Day1 Part1 {}", res);
    assert_eq!(res, 69836);
}

fn day1_part2() {
    let r = read_to_vec();
    let res = r[r.len() - 1] + r[r.len() - 2] + r[r.len() - 3];
    println!("Result Day1 Part2 {}", res);
    assert_eq!(res, 207968);

}

fn read_to_vec() -> Vec<i32> {
    let mut result = Vec::new();
    let mut elf = 0;
    if let Ok(lines) = read_lines("src/day1.txt") {
        for line in lines {
            if let Ok(calories) = line {
                if calories.chars().count() == 0 {
                    result.push(elf);
                    elf = 0;
                } else {
                    elf = elf + calories.parse::<i32>().unwrap();
                }
            }
        }
    }
    result.sort();
    result
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}