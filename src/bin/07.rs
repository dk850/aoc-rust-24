advent_of_code::solution!(7);

fn recurse_operator(result: i64, num_list: Vec<i64>, part_2: bool) -> bool {
    // Base case only 1 number left in the list should match the result
    if num_list.len() == 1 {
        // println!(
        //     "Hit base case with result {} and num_list {:?}",
        //     result, num_list
        // );
        result == *num_list.first().unwrap()
    } else {
        // Try division (multiplication)
        if result % num_list.last().unwrap() == 0 {
            // if result mod num_list == 0 then the division here is valid
            let new_result = result / num_list.last().unwrap();
            let mut new_num = num_list.clone();
            new_num.pop();
            // println!("new after division: {}    {:?}", new_result, new_num);
            if recurse_operator(new_result, new_num, part_2) {
                return true;
            }
        }

        // Try concatenation if on part 2
        if part_2 {
            // have to do some funky power of 10 maths here in case concatting > 1 digit
            let mut next_10 = 1;
            while next_10 <= *num_list.last().unwrap() {
                next_10 *= 10; // figure out how many digits we have (1=1, 10=2, )
            }
            let new_result = result - num_list.last().unwrap(); // 156: 15 || 6 == 156-6 = 150. with 1 digit
            if new_result % next_10 == 0 {
                // now we can concat if the last digit matches (156-6) is 150 [6-6]
                let mut new_num = num_list.clone();
                new_num.pop();
                // println!("new after concat: {}    {:?}", new_result / np10, new_num);
                if recurse_operator(new_result / next_10, new_num, part_2) {
                    // we have taken a power of 10 (or multiple 100 1000) off by concatting we just take off the matching digit
                    return true;
                }
            }
        }

        // Try subtraction (addition)
        if result - num_list.last().unwrap() > 0 {
            let new_result = result - num_list.last().unwrap();
            let mut new_num = num_list;
            new_num.pop();
            // println!("new after subtraction: {}    {:?}", new_result, new_num);
            if recurse_operator(new_result, new_num, part_2) {
                return true;
            }
        }
        false
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut final_result: u64 = 0;
    for line in input.split("\n") {
        let (result_s, num_list_s) = line.split_once(":").unwrap();
        let result: i64 = result_s.parse().unwrap();
        let num_list: Vec<i64> = num_list_s
            .trim()
            .split(" ")
            .map(|x| x.parse().unwrap())
            .collect();
        // println!("Result: {}  REST: {:?}", result, num_list);
        if recurse_operator(result, num_list, false) {
            final_result += result as u64;
        }
        // println!();
    }
    // println!("{}", final_result);
    Some(final_result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut final_result: u64 = 0;
    for line in input.split("\n") {
        let (result_s, num_list_s) = line.split_once(":").unwrap();
        let result: i64 = result_s.parse().unwrap();
        let num_list: Vec<i64> = num_list_s
            .trim()
            .split(" ")
            .map(|x| x.parse().unwrap())
            .collect();
        if recurse_operator(result, num_list, true) {
            final_result += result as u64;
        }
    }
    // println!("{}", final_result);
    Some(final_result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3969));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
