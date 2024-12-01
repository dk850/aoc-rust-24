advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lhs: Vec<i32> = Vec::new();
    let mut rhs: Vec<i32> = Vec::new();

    // Build left and right hand side lists
    for id in input.split("\n") {
        let line: Vec<String> = id.split_whitespace().map(str::to_string).collect();
        lhs.push(line.first()?.parse().unwrap());
        rhs.push(line.last()?.parse().unwrap());
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
    let sum: u32 = i32::try_into(diffs.iter().sum()).unwrap();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lhs: Vec<u32> = Vec::new();
    let mut rhs: Vec<u32> = Vec::new();

    // Build left and right hand side lists
    for id in input.split("\n") {
        let line: Vec<String> = id.split_whitespace().map(str::to_string).collect();
        lhs.push(line.first()?.parse().unwrap());
        rhs.push(line.last()?.parse().unwrap());
    }

    // Count occurances of LHS in RHS and sum as per day logic
    // Might be faster sorting first and somehow telling rust its searching a sorted list when counting occurrences. Didn't bother optimising
    let mut sum_similarity_score: u32 = 0;
    for lhs_elem in lhs.iter_mut() {
        let this_count: u32 = rhs
            .iter()
            .filter(|&val| *val == *lhs_elem)
            .count()
            .try_into()
            .unwrap(); // count occurrences of a filter that returns true for when rhs 'val' is the val of the lhs element
        sum_similarity_score += *lhs_elem * this_count; // day logic
    }
    Some(sum_similarity_score)
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
        assert_eq!(result, Some(31));
    }
}
