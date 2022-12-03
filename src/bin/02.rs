#![allow(unused_variables)]

/*
0 0 -> 0 Draw
0 1 -> -1 Loss
0 2 -> -2 Win
1 0 -> 1 Win
1 1 -> 0 Draw
1 2 -> -1 Loss
2 0 -> 2 Loss
2 1 -> 1 Win
2 2 -> 0 Draw
*/
pub fn part_one(input: &str) -> Option<u32> {
    let mut score: u32 = 0;
    for round in input.lines() {
        if let Some((elf, me)) = round.split_once(' ') {
            let elf = (elf.chars().next().unwrap() as i32) - 65;
            let me = (me.chars().next().unwrap() as i32) - 88;

            score += (me as u32) + 1;
            let result = elf - me;
            match result {
                0 => {
                    // Draw
                    score += 3
                }
                -1 | 2 => {
                    // Win
                    score += 6
                }
                _ => {
                    // Loss
                }
            }
        }
    }
    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut score: u32 = 0;
    for round in input.lines() {
        if let Some((elf, result)) = round.split_once(' ') {
            let elf = (elf.chars().next().unwrap() as u32) - 65;
            let result = (result.chars().next().unwrap() as u32) - 88;

            score += (result as u32) * 3;
            match result {
                0 => {
                    // Lose
                    score += (elf + 2) % 3 + 1
                }
                1 => {
                    // Draw
                    score += elf + 1
                }
                _ => {
                    // Win
                    score += (elf + 1) % 3 + 1
                }
            }
        }
    }
    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
