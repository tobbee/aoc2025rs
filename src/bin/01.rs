advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.lines();
    let wrap = 100;
    let mut position: i32 = 50;
    let mut nr_zeroes: u64 = 0;
    for line in lines {
        let first_char = line.chars().next()?;
        assert!(first_char == 'L' || first_char == 'R', "First character must be L or R");
        let steps: i32 = line[1..].parse().ok()?;
        if first_char == 'L' {
            position = (position - steps) % wrap;
        } else {
            position = (position + steps) % wrap;
        }
        if position < 0 {
            position += wrap;
        }
        if position == 0 {
            nr_zeroes += 1;
        }
        // Process the direction and number here
    }
    Some(nr_zeroes)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.lines();
    let wrap = 100;
    let mut position: i32 = 50;
    let mut nr_zeroes: u64 = 0;
    for line in lines {
        let first_char = line.chars().next()?;
        assert!(first_char == 'L' || first_char == 'R', "First character must be L or R");
        let mut steps: i32 = line[1..].parse().ok()?;
        let nr_wraps = steps / wrap;
        nr_zeroes += nr_wraps as u64;
        steps = steps % wrap;
        if steps == 0 {
            continue;
        }
        if first_char == 'L' {
            if position <= steps  && position != 0{
                nr_zeroes += 1;
            }
            position = (position - steps + wrap) % wrap;
        } else {
            if position + steps >= wrap {
                nr_zeroes += 1;
            }
            position = (position + steps) % wrap;
        }
    }
    Some(nr_zeroes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
