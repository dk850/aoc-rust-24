use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut results: Vec<i32> = Vec::new();

    // Create regex and search
    let instruction_re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let numbers_re = Regex::new(r"\d{1,3}").unwrap();
    let instructions: Vec<&str> = instruction_re
        .find_iter(input)
        .map(|r| r.as_str())
        .collect();

    for instruction in instructions.iter() {
        // Parse the digits to multiply
        let operands: Vec<i32> = numbers_re
            .find_iter(instruction)
            .map(|r| r.as_str().parse::<i32>().unwrap())
            .collect();
        let result: i32 = operands[0] * operands[1];
        results.push(result);
    }

    let total: u32 = i32::try_into(results.iter().sum()).unwrap();
    println!("Sum of results: {}", total);
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut results: Vec<i32> = Vec::new();

    // Default our work command to true
    let mut is_mul_enabled = true;

    // Create regex and search
    let instruction_re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)").unwrap();
    let numbers_re = Regex::new(r"\d{1,3}").unwrap();
    let instructions: Vec<&str> = instruction_re
        .find_iter(input)
        .map(|r| r.as_str())
        .collect();

    for instruction in instructions.iter() {
        // Apply any do or don't commands
        if *instruction == "do()" {
            is_mul_enabled = true;
            continue;
        } else if *instruction == "don't()" {
            is_mul_enabled = false;
            continue;
        }

        // Only do work if mul is enabled
        if is_mul_enabled {
            // Parse the digits to multiply
            let operands: Vec<i32> = numbers_re
                .find_iter(instruction)
                .map(|r| r.as_str().parse::<i32>().unwrap())
                .collect();
            let result: i32 = operands[0] * operands[1];
            results.push(result);
        }
    }
    let total: u32 = i32::try_into(results.iter().sum()).unwrap();
    println!("Sum of results: {}", total);
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
