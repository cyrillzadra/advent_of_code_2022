use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const INPUT_DAY1 : &str = "src/day1.txt";
const INPUT_DAY2 : &str = "src/day2.txt";
const INPUT_DAY3 : &str = "src/day3.txt";
const INPUT_DAY4 : &str = "src/day4.txt";

struct Day4 {

}


impl Day4 {

    /*https://adventofcode.com/2022/day/4 */

    fn part1() {
        if let Ok(lines) = read_lines(INPUT_DAY4) {
            // split line into 2 groups with comma as separator.
            let sum : i32 = lines.map(|l| Self::compare_ranges_and_check_if_overlaps(  l.unwrap().split(',').collect()) ).sum();
            println!("Result Day4 Part1 {}", sum);
            assert_eq!(sum, 644);
        }
    }

    fn part2() {
        if let Ok(lines) = read_lines(INPUT_DAY4) {
            let sum : i32 = lines.map(|l| Self::compare_ranges_and_sum_pairs_that_are_overlapping( l.unwrap().split(',').collect() ) ).sum();
            println!("Result Day4 Part2 {}", sum);
            assert_eq!(sum, 926);
        }
    }

    fn compare_ranges_and_sum_pairs_that_are_overlapping(v: Vec<&str>) -> i32 {
        let (start1, end1) = Self::split_range(v.get(0).unwrap());
        let (start2, end2) = Self::split_range(v.get(1).unwrap());
        let overlaps = Self::check_if_pair_overlaps(start1, end1, start2, end2);
        
        Self::bool2int(overlaps)
    } 

    fn split_range(range: &str) -> (i32, i32) {
        let v: Vec<&str> = range.split('-').collect();
        let start = v.get(0).unwrap().parse::<i32>().unwrap();
        let end = v.get(1).unwrap().parse::<i32>().unwrap();
        (start, end)
    }

    fn compare_ranges_and_check_if_overlaps(v: Vec<&str>) -> i32 {
        let (start1, end1) = Self::split_range(v.get(0).unwrap());
        let (start2, end2) = Self::split_range(v.get(1).unwrap());
        let overlaps = Self::check_if_pair_fully_contain_other(start1, end1, start2, end2);
       
        Self::bool2int(overlaps)
    }

    fn bool2int(b: bool) -> i32 {
        if b {
            return 1;
        }
        0
    }

    fn check_if_pair_overlaps(start1: i32, end1: i32, start2: i32, end2: i32) -> bool {
        start1 <= start2 && start2 <= end1
            || start1 <= end2 && end2 <= end1
            || start2 <= start1 && start1 <= end2
            || start2 <= end1 && end1 <= end2
    }

    fn check_if_pair_fully_contain_other(start1: i32, end1: i32, start2: i32, end2: i32) -> bool {
        if start1 <= start2 && end1 >= end2 {
            return true;
        }
        if start2 <= start1 && end2 >= end1 {
            return true;
        }
        false
    }


}

fn main() {
    day1_part1();
    day1_part2();

    day2_part1();
    day2_part2();

    day3_part1();
    day3_part2();

    Day4::part1();
    Day4::part2();
}


/*https://adventofcode.com/2022/day/3 */

fn day3_part2() {
    if let Ok(lines) = read_lines(INPUT_DAY3) {
        let v: Vec<String> = lines.map(|l| l.unwrap()).collect();
        let sum: i32 = v.chunks(3).map(|c| c.to_vec()).map(|c| priorities2(c)).sum();
        println!("Result Day3 Part2 {}", sum);
        assert_eq!(sum, 2881);
    }
} 


fn day3_part1() {
    if let Ok(lines) = read_lines(INPUT_DAY3) {
        let sum: i32 = lines.map(|line| priorities(line.unwrap())).sum();
        println!("Result Day3 Part1 {}", sum);
        assert_eq!(sum, 7980);
    }
}

fn priorities2(lines: Vec<String>) -> i32 {
    let mut characters: HashMap<char, i32> = HashMap::new();
    let chars = 'a'..='z';
    for (i, c) in chars.enumerate() {
        characters.insert(c, i as i32 + 1);
    }
    let chars = 'A'..='Z';
    for (i, c) in chars.enumerate() {
        characters.insert(c, 26 + i as i32 + 1);
    }

    let mut map1 = HashMap::new();
    for s in lines.get(0).unwrap().chars() {
        map1.entry(s).and_modify(|v| *v += 1).or_insert(1);
    }

    let mut map2 = HashMap::new();
    for s in lines.get(1).unwrap().chars() {
        map2.entry(s).and_modify(|v| *v += 1).or_insert(1);
    }

    for s in lines.get(2).unwrap().chars() {
        if map1.contains_key(&s) && map2.contains_key(&s) {
            match characters.get(&s) {
                Some(&number) => {
                    return number;
                }
                _ => 0,
            };
        }
    }
    0
}


fn priorities(line: String) -> i32 {
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
                    return number;
                }
                _ => 0,
            };
        }
    }
    0
}

fn day2_part1() {
    if let Ok(lines) = read_lines(INPUT_DAY2) {
        let score: i32 = lines.map(|l| score_part1(l.unwrap())).sum();
        println!("Result Day2 Part1 {}", score);
        assert_eq!(score, 12740);
    }
}

fn day2_part2() {
    if let Ok(lines) = read_lines(INPUT_DAY2) {
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
    if let Ok(lines) = read_lines(INPUT_DAY1) {
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