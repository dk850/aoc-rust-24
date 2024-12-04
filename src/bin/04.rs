use core::str;

use advent_of_code::template::aoc_cli::check;

advent_of_code::solution!(4);

struct GridPos {
    x: usize,
    y: usize,
}

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

pub fn check_letter(grid: &Vec<Vec<String>>, letter: String, at_pos: &GridPos) -> bool {
    println!(
        "Checking letter {} at pos {}, {}",
        letter, at_pos.x, at_pos.y
    );
    grid[at_pos.y][at_pos.x] == letter
}

pub fn check_word(
    grid: &Vec<Vec<String>>,
    word: String,
    at_pos: &GridPos,
    direction: GridDir,
) -> bool {
    match direction {
        GridDir::Up => {
            println!("Checking up");
            for letter in 0..word.len() {
                let check_pos = GridPos {
                    x: at_pos.x,
                    y: at_pos.y - letter,
                };
                let abc = (word.as_bytes()[letter] as char).to_string();
                println!("LETTER {} of {} is {}", letter, word, abc);
                if !check_letter(grid, abc, &check_pos) {
                    return false;
                }
            }
        }
        _ => todo!(),
    }
    false
}

pub fn find_word_in_grid(grid: &Vec<Vec<String>>, word: String, at_pos: &GridPos) -> i32 {
    // LOGIC : Check the word can fit in the direction, then for each letter, move an extra space and see if its there
    let mut found_count: u32;
    let word_len = word.len();
    let max_y = grid.len();
    let max_x = grid.first().unwrap_or(&Vec::new()).len();

    // Look up
    match at_pos.y.checked_sub(word_len - 1) {
        None => println!("{}, {} nofit", at_pos.x, at_pos.y),
        _ => {
            println!("{}, {} fit", at_pos.x, at_pos.y);
            check_word(grid, word, at_pos, GridDir::Up);
        }
    }

    // Look down

    // Look left

    // Look right

    // Look up-left diagonal

    // Look up-right diagonal

    // Look down-left diagonal

    // Look down-right diagonal
    0
}

pub fn part_one(input: &str) -> Option<u32> {
    let input_grid = parse_into_grid(input);
    println!("Input Wordsearch:");
    advent_of_code::print_2d_vec(&input_grid);

    // Iterate over grid to find every occurrence of X (starting letter of search term)
    let mut x_pos: Vec<GridPos> = Vec::new(); // [1, 2], [0, 0]...
    for (y, row) in input_grid.iter().enumerate() {
        for (x, letter) in row.iter().enumerate() {
            if letter == "X" {
                x_pos.push(GridPos { x, y });
            }
        }
    }

    for xxx in x_pos.iter() {
        find_word_in_grid(&input_grid, "soop".to_string(), xxx);
    }

    // LOGIC:
    // get positions of the first letter of the word
    // pass to a function
    //  find_word_in_grid(grid, word, start_pos)
    //  which then passes it to
    //    look up(grid, letter, start_pos)  | down, left, right, ul, ur, dl, dr
    //    if up true for X, M, A, S then +1. Go other dirs

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
