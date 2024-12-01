advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lhs: Vec<i32> = Vec::new();
    let mut rhs: Vec<i32> = Vec::new();

    // Build left and right hand side lists
    for id in input.split("\n") {
        let line: Vec<String> = id.split_whitespace().map(str::to_string).collect();
        lhs.push(line.get(0)?.parse().unwrap());
        rhs.push(line.get(1)?.parse().unwrap());
    }

    // Sort lists
    lhs.sort();
    rhs.sort();

    // Calculate differences
    let mut diffs: Vec<i32> = Vec::new();
    for (index, lhs_elem) in lhs.iter_mut().enumerate() {
        let diff: i32 = (*lhs_elem - rhs.get(index).unwrap()).abs();
        diffs.push(diff);
    }

    // Sum and get answer
    let sum: u32 = match i32::try_into(diffs.iter().sum()) {
        Ok(u32) => u32,
        Err(e) => 0,
    };
    return Some(sum);
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
