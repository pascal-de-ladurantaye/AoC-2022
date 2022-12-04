#![allow(dead_code)]
#![allow(unused_variables)]

use itertools::{self, Itertools};

struct Group {
    elf1_sack: RuckSack,
    elf2_sack: RuckSack,
    elf3_sack: RuckSack,
}

impl Group {
    fn score(&self) -> usize {
        let mut score: usize = 0;
        for (i, item) in self.elf1_sack.all.into_iter().enumerate() {
            if item && item == self.elf2_sack.all[i] && item == self.elf3_sack.all[i] {
                score = i + 1;
                break;
            }
        }
        score
    }
}
struct RuckSack {
    compartment1: [bool; 52],
    compartment2: [bool; 52],
    all: [bool; 52],
}

fn build_ruck_sack(content: &str) -> RuckSack {
    let content = content.chars();
    let mut compartment1 = [false; 52];
    let mut compartment2 = [false; 52];
    let mut all = [false; 52];

    let items_per_compartment = content.clone().count() / 2;

    for (i, item) in content.enumerate() {
        let mut char_value = item as usize - 65;
        if item.is_lowercase() {
            char_value -= 32;
        } else {
            char_value += 26;
        }
        all[char_value] = true;
        if i < items_per_compartment {
            compartment1[char_value] = true;
        } else {
            compartment2[char_value] = true;
        }
    }

    RuckSack {
        compartment1,
        compartment2,
        all,
    }
}

impl RuckSack {
    fn score(&self) -> usize {
        let mut score: usize = 0;
        for (i, item) in self.compartment1.into_iter().enumerate() {
            if item && item == self.compartment2[i] {
                score = i + 1;
                break;
            }
        }
        score
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| build_ruck_sack(line).score())
            .sum::<usize>() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .chunks(3)
            .into_iter()
            .map(|mut chunk| {
                Group {
                    elf1_sack: build_ruck_sack(chunk.next().unwrap()),
                    elf2_sack: build_ruck_sack(chunk.next().unwrap()),
                    elf3_sack: build_ruck_sack(chunk.next().unwrap()),
                }
                .score()
            })
            .sum::<usize>() as u32,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
