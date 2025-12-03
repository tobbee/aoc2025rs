advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum_invalids = 0;
    let ranges = split_to_ranges(input);
    for r in ranges {
        let (start, end) = r;
        for n in start..=end {
            let n_str = n.to_string();
            let n_len = n_str.len();
            if n_len % 2 != 0 {
                continue;
            }
            let half_str = &n_str[0..n_str.len()/2];
            let full_str = half_str.repeat(2);
            if full_str == n_str {
                sum_invalids += n;
            }
        }
    }
    Some(sum_invalids as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum_invalids = 0;
    let ranges = split_to_ranges(input);
    for r in ranges {
        let (start, end) = r;
        for n in start..=end {
            let n_str = n.to_string();
            let n_len = n_str.len();
            for i in 1..=n_len/2 {
                if n_len % i != 0 {
                    continue;
                }
                let segment = &n_str[0..i];
                let repeated = segment.repeat(n_len / i);
                if repeated == n_str {
                    sum_invalids += n;
                    break;
                }
            }
        }
    }
    Some(sum_invalids as u64)
}

fn split_to_ranges(s: &str) -> Vec<(u64, u64)> {
    let t = s.trim();
    t.split(',')
        .map(|r| {
            let mut bounds = r.split('-').map(|n| n.parse::<u64>().unwrap());
            (
                bounds.next().unwrap(),
                bounds.next().unwrap(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
