#[macro_use]
extern crate lazy_static;

use advent_of_code_2022::core::get_lines;
use std::collections::{HashMap, HashSet};

lazy_static! {
    static ref PRIORITY: HashMap<char, u64> = {
        let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

        let map: HashMap<char, u64> =
            alphabet
                .chars()
                .enumerate()
                .fold(HashMap::new(), |mut acc, (idx, el)| {
                    acc.insert(el, idx as u64 + 1);

                    acc
                });

        map
    };
}

#[derive(Debug)]
struct Rucksack {
    compartments: Vec<String>,
    sets: Vec<HashSet<char>>,
    full_set: HashSet<char>,
}

impl From<&String> for Rucksack {
    fn from(value: &String) -> Self {
        let half = value.len() / 2;
        let (lhs, rhs) = value.split_at(half);

        Rucksack::new(vec![String::from(lhs), String::from(rhs)])
    }
}

pub fn intersection_of_sets(sets: &[HashSet<char>]) -> HashSet<char> {
    let intersections: Option<HashSet<char>> = sets.iter().fold(None, |acc, c| match acc {
        None => Some(c.clone()),
        Some(next) => Some(next.intersection(c).copied().collect()),
    });

    intersections.unwrap_or_default()
}

impl Rucksack {
    pub fn new(compartments: Vec<String>) -> Self {
        let sets: Vec<HashSet<char>> = compartments.iter().map(|x| x.chars().collect()).collect();
        let full_set: HashSet<char> =
            compartments
                .iter()
                .fold(HashSet::new(), |mut set, compartment| {
                    compartment.chars().for_each(|c| {
                        set.insert(c);
                    });
                    set
                });

        Self {
            compartments,
            sets,
            full_set,
        }
    }

    pub fn find_common(&self) -> Vec<char> {
        intersection_of_sets(&self.sets).iter().copied().collect()
    }

    pub fn priority_of_common_items(&self) -> u64 {
        self.find_common()
            .iter()
            .map(|c| *(PRIORITY.get(c).unwrap()))
            .sum()
    }
}

fn solve_one(input: &[String]) -> u64 {
    input
        .iter()
        .map(Rucksack::from)
        .map(|ruck| ruck.priority_of_common_items())
        .sum()
}

fn solve_two(input: &[String]) -> u64 {
    input
        .chunks(3)
        .map(|window| {
            window
                .iter()
                .map(Rucksack::from)
                .map(|rucksack| rucksack.full_set)
                .collect::<Vec<HashSet<char>>>()
        })
        .map(|x| intersection_of_sets(&x))
        .map(|intersections| {
            intersections
                .iter()
                .map(|c| *(PRIORITY.get(c).unwrap()))
                .sum::<u64>()
        })
        .sum()
}

fn main() -> std::io::Result<()> {
    let data = get_lines("day-4")?;

    // println!("result: {}", solve_one(&data));
    println!("result: {}", solve_two(&data));
    Ok(())
}

#[cfg(test)]
mod day_3_tests {
    use crate::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_problem_one() {
        let lines: Vec<String> = INPUT.split('\n').map(String::from).collect();
        let result = solve_one(&lines);

        assert_eq!(result, 157)
    }

    #[test]
    fn test_problem_two() {
        let lines: Vec<String> = INPUT.split('\n').map(String::from).collect();
        let result = solve_two(&lines);

        assert_eq!(result, 70)
    }
}
