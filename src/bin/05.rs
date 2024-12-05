use std::collections::HashMap;

advent_of_code::solution!(5);

fn check_update(update_list: &[i32], rules: &HashMap<i32, Vec<i32>>) -> bool {
    let mut is_valid = true;
    for (i, page) in update_list.iter().enumerate() {
        // Loop over each page rule if any
        // println!("Looking for rule {}:{:?}", page, rules.get(page).unwrap());
        for rule in rules.get(page).unwrap() {
            // Each rule page must be of a greater index than our current index
            if let Some(at_pos) = update_list.iter().position(|&r| r == *rule) {
                // println!("Checking rule {}:{:?}", page, rule);
                if i > at_pos {
                    // println!("NOT VALID FALSE");
                    is_valid = false;
                }
            }
        }
    }
    is_valid
}

fn custom_bubble_sort(unsorted_list: &[i32], rules: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut swapped = true;
    let mut sorted_list: Vec<i32> = unsorted_list.to_vec();

    // implement bubble sort using the rules as comparator
    while swapped {
        swapped = false;
        for i in 0..sorted_list.len() {
            // println!("Checking letter {}", sorted_list[i]);
            for rule in rules.get(&sorted_list[i]).unwrap() {
                if let Some(at_pos) = sorted_list.iter().position(|&r| r == *rule) {
                    if i > at_pos {
                        // println!("Swapping {}:{} with {}:{}", i, rule, at_pos, sorted_list[i]);
                        sorted_list.swap(i, at_pos);
                        swapped = true;
                        break;
                    }
                }
                // D.K- Maybe these conditional ifs should be in a while loop somewhere?
            }
        }
        if check_update(&sorted_list, rules) {
            swapped = false;
            // println!();
        }
    }

    // return arr;

    sorted_list
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut running_total: u32 = 0;

    // Parse input into data structures
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();
    let mut on_rules: bool = true;
    for line in input.split("\n") {
        // start parsing updates instead of rules if we have hit a newline
        if line.is_empty() {
            on_rules = false;
            continue;
        }

        if on_rules {
            // D.K- I spent too much time trying to collapse these 2 lines how split str into i32
            let (key_s, value_s) = line.trim().split_once("|").unwrap();
            let (key, value): (i32, i32) = (key_s.parse().unwrap(), value_s.parse().unwrap());
            rules
                .entry(key)
                .and_modify(|r_list| r_list.push(value))
                .or_insert(vec![value]);
        } else {
            updates.push(
                line.split(",")
                    .map(|x| -> i32 { x.parse().unwrap() })
                    .collect::<Vec<i32>>(),
            );
        }
    }

    // Loop over pages in updates and check for validity against the rules
    for update in updates {
        let mut has_failed = false;
        for (i, page) in update.iter().enumerate() {
            // Loop over each page rule if any
            for rule in rules.entry(*page).or_default() {
                // Each rule page must be of a greater index than our current index
                // println!("we@{}       {:?} is at index {:?}", i, rule, update.iter().position(|&r| r == *rule));
                if let Some(atpos) = update.iter().position(|&r| r == *rule) {
                    if i > atpos {
                        has_failed = true;
                    }
                }
                // D.K- Maybe these conditional ifs should be in a while loop somewhere?
                if has_failed {
                    break;
                }
            }
            if has_failed {
                break;
            }
        }
        if has_failed {
            // println!();
            continue;
        }
        // If we reach here the update is valid so get the answer for the day
        // println!("Rule passed. Midpoint: {}", *update.get(update.len()/2).unwrap() as u32);
        running_total += *update.get(update.len() / 2).unwrap() as u32;
        // println!();
    }
    // println!("FINAL: {}", running_total);
    Some(running_total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut running_total: u32 = 0;
    let mut to_fix: Vec<Vec<i32>> = Vec::new();

    // Parse input into data structures
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();
    let mut on_rules: bool = true;
    for line in input.split("\n") {
        // start parsing updates instead of rules if we have hit a newline
        if line.is_empty() {
            on_rules = false;
            continue;
        }

        if on_rules {
            // D.K- I spent too much time trying to collapse these 2 lines how split str into i32
            let (key_s, value_s) = line.trim().split_once("|").unwrap();
            let (key, value): (i32, i32) = (key_s.parse().unwrap(), value_s.parse().unwrap());
            rules
                .entry(key)
                .and_modify(|r_list| r_list.push(value))
                .or_insert(vec![value]);
        } else {
            updates.push(
                line.split(",")
                    .map(|x| -> i32 { x.parse().unwrap() })
                    .collect::<Vec<i32>>(),
            );
        }
    }

    // Loop over pages in updates and check for validity against the rules
    for update in updates {
        let mut has_failed = false;
        for (i, page) in update.iter().enumerate() {
            // Loop over each page rule if any
            for rule in rules.entry(*page).or_default() {
                // Each rule page must be of a greater index than our current index
                if let Some(atpos) = update.iter().position(|&r| r == *rule) {
                    if i > atpos {
                        has_failed = true;
                        to_fix.push(update.clone());
                    }
                }
                // D.K- Maybe these conditional ifs should be in a while loop somewhere?
                if has_failed {
                    break;
                }
            }
            if has_failed {
                break;
            }
        }
        if has_failed {
            continue;
        }
    }
    // println!("To Fix: {}", to_fix.len());

    //
    // PART 2 MAIN BODY BELOW
    //
    for broken_update in to_fix {
        let fixed_update = custom_bubble_sort(&broken_update, &rules);
        running_total += fixed_update[fixed_update.len() / 2] as u32;
    }
    Some(running_total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
