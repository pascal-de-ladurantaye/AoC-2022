pub fn part_one(input: &str) -> Option<u32> {
    let mut max_calories: u32 = 0;
    let mut current_elf_calories: u32 = 0;
    for line in input.lines().chain([""]) {
        if line.is_empty() {
            if current_elf_calories > max_calories {
                max_calories = current_elf_calories;
            }
            current_elf_calories = 0;
            continue;
        }
        let meal_calories = line.parse::<u32>().unwrap();
        current_elf_calories += meal_calories;
    }
    Some(max_calories)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut top_3_calories: [u32; 3] = [0, 0, 0];
    let mut minimum_value_index: usize = 2;
    let mut current_elf_calories: u32 = 0;

    for line in input.lines().chain([""]) {
        if line.is_empty() {
            if current_elf_calories > top_3_calories[minimum_value_index] {
                top_3_calories[minimum_value_index] = current_elf_calories;
            }
            current_elf_calories = 0;
            minimum_value_index = find_minimum_value_index(&top_3_calories);
            continue;
        }
        let meal_calories = line.parse::<u32>().unwrap();
        current_elf_calories += meal_calories;
    }

    Some(top_3_calories.iter().sum())
}

fn find_minimum_value_index(arr: &[u32; 3]) -> usize {
    let mut minimum_value_index: usize = 0;
    for i in 1..arr.len() {
        if arr[i] < arr[minimum_value_index] {
            minimum_value_index = i;
        }
    }
    minimum_value_index
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(30));
    }
}
