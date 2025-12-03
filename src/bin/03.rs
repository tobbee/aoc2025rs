advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    biggest_joltage(input, 2)
}

pub fn part_two(input: &str) -> Option<u64> {
    biggest_joltage(input, 12)
}

fn biggest_joltage(input: &str, n: i32) -> Option<u64> {
    let lines = input.trim().lines();
    let mut sum_joltage: u64 = 0;
    for line in lines {
        let batteries = line.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<u32>>();
        let mut last_index = -1;
        let mut new_index = -1;
        let mut digits = vec![-1; n as usize];
        for i in 0..n {
            for j in last_index+1..(batteries.len() as i32) -(n - i - 1) {
                if batteries[j as usize] as i32 > digits[i as usize] {
                    digits[i as usize] = batteries[j as usize] as i32;
                    new_index = j;
                }
            }
            last_index = new_index;
        }
        let zoltage = number_from_digits(&digits);
        sum_joltage += zoltage;
    }
    Some(sum_joltage)
}

fn number_from_digits(digits: &Vec<i32>) -> u64 {
    let mut number: u64 = 0;
    let mut multiplier: u64 = 1;
    for i in (0..digits.len()).rev() {
        number += digits[i] as u64 * multiplier;
        multiplier *= 10;
    }
    number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
