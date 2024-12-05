use std::collections::HashMap;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    // LOGIC:
    // parse input as dictionary lookup hashmap until newline RULES
    //  each LHS value has a vec u32 of the RHS
    // after a blank line, add each line to a 2d vector of ints UPDATES
    // for each update, loop over each int
    //  lookup the int in the dict to get the vec of its rules ints
    //  loop over this rules vector and do a find to get the index of the pagenumber in this update
    //  if index of find > current rule pos, break and fail
    //  else add the len()/2 element to a running total to return

    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    let mut onRules: bool = true;
    for line in input.split("\n") {
        // start parsing updates if we have hit a newline
        if line.is_empty() {
            onRules = false;
        }

        if onRules {
            // D.K- I spent too much time trying to collapse these 2 lines
            let (key_s, value_s) = line.split_once("|").unwrap();
            let (key, value): (i32, i32) = (key_s.parse().unwrap(), value_s.parse().unwrap());

            rules
                .entry(key)
                .and_modify(|r_list| r_list.push(value))
                .or_insert(vec![value]);
        } else {
            let update_s = line.split(",").collect::<Vec<&str>>();
            
        }
    }

    None
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
