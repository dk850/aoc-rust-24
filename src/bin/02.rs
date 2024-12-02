use std::process::exit;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut record_decreasing: bool = false;
    let mut safe_count: u32 = 0;

    // Split line for each report and loop over each level in the report
    for record in input.split("\n") {
        let rec_vec: Vec<&str> = record.split(" ").collect(); // easier to deal with as a vector
        println!("Current record: {:?}", rec_vec);

        for (i, curr_str) in rec_vec.iter().enumerate() {
            let curr_level: i32 = curr_str.parse().unwrap();
            // CHECKS
            // Break at the end
            if i == rec_vec.len() - 1 {
                println!(
                    "Exhausted each level in this report. This: {} Max: {}",
                    i,
                    rec_vec.len()
                );
                break;
            }

            // Set record direction when we are at the beginning
            let next_str: &&str = rec_vec.get(i + 1).unwrap();
            let next_level: i32 = next_str.parse().unwrap();
            if i == 0 {
                if curr_level > next_level {
                    println!("First bigger than next so list is decreasing");
                    record_decreasing = true;
                } else if curr_level < next_level {
                    println!("First smaller than next so list is increasing");
                    record_decreasing = false;
                } else {
                    println!("First 2 elements are the same. Unsafe");
                    break;
                }
            }

            // Calculate difference and ensure it is within boundaries
            let diff: i32 = curr_level - next_level;
            if diff == 0 {
                println!("No difference between levels. Unsafe");
                break;
            } else if record_decreasing && diff < 0 {
                println!("Supposed to be decreasing but difference is negative (increasing level). Unsafe");
                break;
            } else if record_decreasing && diff > 3 {
                println!("Record is decreasing at a rate greater than 3. Unsafe");
                break;
            } else if !record_decreasing && diff > 0 {
                println!("Supposed to be increasing but difference is positive (decreasing level). Unsafe");
                break;
            } else if !record_decreasing && diff < -3 {
                println!("Record is increasing at a rate greater than 3. Unsafe");
                break;
            } else {
                if i == rec_vec.len() - 2 {
                    println!("Final check is safe. Good record");
                    safe_count += 1;
                }
            }
        }
        println!("\n");
    }
    println!("All records checked - safe count: {}", safe_count);

    Some(safe_count)
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
