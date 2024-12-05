use core::str;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

advent_of_code::solution!(4);

#[derive(Debug, PartialEq, Clone)]
struct GridPos {
    x: usize,
    y: usize,
}

#[derive(Debug, EnumIter)]
enum GridDir {
    Up,
    Down,
    Left,
    Right,
    ULeft,
    URight,
    DLeft,
    DRight,
}

pub fn parse_into_grid(input: &str) -> Vec<Vec<String>> {
    let mut output: Vec<Vec<String>> = Vec::new();

    // Build grid
    for line in input.lines() {
        output.push(line.chars().map(String::from).collect());
    }
    output
}

// Returns the mid-point or Error
fn check_word(
    grid: &[Vec<String>],
    word: &String,
    start_pos: &GridPos,
    direction: &GridDir,
) -> Option<GridPos> {
    let max_y = grid.len() - 1;
    let max_x = grid.first().unwrap_or(&Vec::new()).len() - 1;
    let word_len = word.len() - 1;

    // If word is odd return the mid point instead of start point  [MAS vs XMAS]
    let mut point_modifier = 0;
    if word.len() % 2 == 1 {
        point_modifier = (word.len() - 1) / 2;
    }
    let mut ret_pos = GridPos {
        x: start_pos.x,
        y: start_pos.y,
    };

    // Match on direction, error check, then check each letter matches. For each direction
    match direction {
        GridDir::Up => {
            if Option::is_none(&start_pos.y.checked_sub(word_len)) {
                return None;
            }
            for (letter_pos, &letter_bytes) in word.as_bytes().iter().enumerate() {
                if grid[start_pos.y - letter_pos][start_pos.x] != (letter_bytes as char).to_string()
                {
                    return None;
                }
            }
            ret_pos.y -= point_modifier;
            Some(ret_pos)
        }

        GridDir::Down => {
            if Option::is_none(&start_pos.y.checked_add(word_len))
                || ((start_pos.y + word_len) > max_y)
            {
                return None;
            }
            for (letter_pos, &letter_bytes) in word.as_bytes().iter().enumerate() {
                if grid[start_pos.y + letter_pos][start_pos.x] != (letter_bytes as char).to_string()
                {
                    return None;
                }
            }
            ret_pos.y += point_modifier;
            Some(ret_pos)
        }

        GridDir::Left => {
            if Option::is_none(&start_pos.x.checked_sub(word_len)) {
                return None;
            }
            for (letter_pos, &letter_bytes) in word.as_bytes().iter().enumerate() {
                if grid[start_pos.y][start_pos.x - letter_pos] != (letter_bytes as char).to_string()
                {
                    return None;
                }
            }
            ret_pos.x -= point_modifier;
            Some(ret_pos)
        }

        GridDir::Right => {
            if Option::is_none(&start_pos.x.checked_add(word_len))
                || ((start_pos.x + word_len) > max_x)
            {
                return None;
            }
            for (letter_pos, &letter_bytes) in word.as_bytes().iter().enumerate() {
                if grid[start_pos.y][start_pos.x + letter_pos] != (letter_bytes as char).to_string()
                {
                    return None;
                }
            }
            ret_pos.x += point_modifier;
            Some(ret_pos)
        }

        GridDir::ULeft => {
            if Option::is_none(&start_pos.y.checked_sub(word_len))
                || Option::is_none(&start_pos.x.checked_sub(word_len))
            {
                return None;
            }
            for (letter_pos, &letter_bytes) in word.as_bytes().iter().enumerate() {
                if grid[start_pos.y - letter_pos][start_pos.x - letter_pos]
                    != (letter_bytes as char).to_string()
                {
                    return None;
                }
            }
            ret_pos.y -= point_modifier;
            ret_pos.x -= point_modifier;
            Some(ret_pos)
        }

        GridDir::URight => {
            if Option::is_none(&start_pos.y.checked_sub(word_len))
                || Option::is_none(&start_pos.x.checked_add(word_len))
                || ((start_pos.x + word_len) > max_x)
            {
                return None;
            }
            for (letter_pos, &letter_bytes) in word.as_bytes().iter().enumerate() {
                if grid[start_pos.y - letter_pos][start_pos.x + letter_pos]
                    != (letter_bytes as char).to_string()
                {
                    return None;
                }
            }
            ret_pos.y -= point_modifier;
            ret_pos.x += point_modifier;
            Some(ret_pos)
        }

        GridDir::DLeft => {
            if Option::is_none(&start_pos.y.checked_add(word_len))
                || ((start_pos.y + word_len) > max_y)
                || Option::is_none(&start_pos.x.checked_sub(word_len))
            {
                return None;
            }
            for (letter_pos, &letter_bytes) in word.as_bytes().iter().enumerate() {
                if grid[start_pos.y + letter_pos][start_pos.x - letter_pos]
                    != (letter_bytes as char).to_string()
                {
                    return None;
                }
            }
            ret_pos.y += point_modifier;
            ret_pos.x -= point_modifier;
            Some(ret_pos)
        }

        GridDir::DRight => {
            if Option::is_none(&start_pos.y.checked_add(word_len))
                || ((start_pos.y + word_len) > max_y)
                || Option::is_none(&start_pos.x.checked_add(word_len))
                || ((start_pos.x + word_len) > max_x)
            {
                return None;
            }
            for (letter_pos, &letter_bytes) in word.as_bytes().iter().enumerate() {
                if grid[start_pos.y + letter_pos][start_pos.x + letter_pos]
                    != (letter_bytes as char).to_string()
                {
                    return None;
                }
            }
            ret_pos.y += point_modifier;
            ret_pos.x += point_modifier;
            Some(ret_pos)
        }
    }
}

fn find_word_in_grid(grid: &[Vec<String>], word: &String, at_pos: &GridPos) -> i32 {
    // Check if there is a match in each direction
    let mut found_count: i32 = 0;
    for dir in GridDir::iter() {
        if check_word(grid, word, at_pos, &dir).is_some() {
            found_count += 1;
        }
    }
    found_count
}

fn find_cross_word_in_grid(grid: &[Vec<String>], word: &String, at_pos: &GridPos) -> Vec<GridPos> {
    // Check if there is a match in each diagonal. Return positive match positions for futher filtering
    let mut found_pos: Vec<GridPos> = Vec::new();

    if let Some(ul) = check_word(grid, word, at_pos, &GridDir::ULeft) {
        found_pos.push(ul);
    }

    if let Some(ur) = check_word(grid, word, at_pos, &GridDir::URight) {
        found_pos.push(ur);
    }

    if let Some(dl) = check_word(grid, word, at_pos, &GridDir::DLeft) {
        found_pos.push(dl);
    }

    if let Some(dr) = check_word(grid, word, at_pos, &GridDir::DRight) {
        found_pos.push(dr);
    }
    found_pos
}

pub fn part_one(input: &str) -> Option<u32> {
    let search_term = "XMAS";
    let input_grid = parse_into_grid(input);

    // println!("Input Wordsearch:");
    // advent_of_code::print_2d_vec(&input_grid);

    // Iterate over grid to find every occurrence of X (starting letter of search term)
    let first_letter = search_term.chars().next().unwrap().to_string();
    let mut first_letter_pos: Vec<GridPos> = Vec::new(); // [1, 2], [0, 0]...
    for (y, row) in input_grid.iter().enumerate() {
        for (x, letter) in row.iter().enumerate() {
            if letter == &first_letter {
                first_letter_pos.push(GridPos { x, y });
            }
        }
    }
    // println!("Found {} at {} positions", first_letter, first_letter_pos.len());

    // See if any of the X positions have the full word
    let mut total_matching = 0;
    for p_match_pos in first_letter_pos.iter() {
        total_matching += find_word_in_grid(&input_grid, &search_term.to_string(), p_match_pos);
    }
    Some(total_matching as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    // D.K- a horrible shoehorn of my implementation to solve the answer
    let search_term = "MAS";
    let input_grid = parse_into_grid(input);

    let first_letter = search_term.chars().next().unwrap().to_string();
    let mut first_letter_pos: Vec<GridPos> = Vec::new(); // [1, 2], [0, 0]...
    for (y, row) in input_grid.iter().enumerate() {
        for (x, letter) in row.iter().enumerate() {
            if letter == &first_letter {
                first_letter_pos.push(GridPos { x, y });
            }
        }
    }
    // println!("Found {} at {} positions", first_letter, first_letter_pos.len());

    let mut total_grid_pos: Vec<GridPos> = Vec::new();
    for p_match_pos in first_letter_pos.iter() {
        total_grid_pos.append(
            find_cross_word_in_grid(&input_grid, &search_term.to_string(), p_match_pos).as_mut(),
        ); // add newly found grid pos to our running vector
    }

    // Any duplicate pos in the vector mean theres a valid cross
    let mut matches = 0;
    let iter_pos_clone = total_grid_pos.clone();
    for pos in total_grid_pos {
        if iter_pos_clone.iter().filter(|&n| *n == pos).count() == 2 {
            matches += 1;
        }
    }
    Some((matches / 2) as u32) // each entry will be in the list twice so /2 gives real result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
