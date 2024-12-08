use std::collections::HashSet;

use advent_of_code::GridPos;

advent_of_code::solution!(6);

#[derive(Debug)]
enum GuardDir {
    Up,
    Down,
    Left,
    Right,
}

fn is_position_valid(grid: &[Vec<String>], pos: &GridPos) -> bool {
    grid[pos.y][pos.x] != "#"
}

fn check_guard_unique_places(grid: &[Vec<String>], start_pos: GridPos) -> HashSet<GridPos> {
    let mut on_map: bool = true;
    let mut guard_dir = GuardDir::Up;
    let max_y = grid.len() - 1;
    let max_x = grid.first().unwrap_or(&Vec::new()).len() - 1;
    let mut unique_spaces_visited: HashSet<GridPos> = HashSet::new();
    let mut guard_position = start_pos;
    unique_spaces_visited.insert(guard_position);
    let mut old_unique_spaces_count = 0;
    let mut loop_check_count = 0;
    while on_map {
        // Check the position in front of the guard
        match guard_dir {
            GuardDir::Up => {
                // If out of bounds we are DONE
                if Option::is_none(&guard_position.y.checked_sub(1)) {
                    on_map = false;
                } else {
                    let new_pos = GridPos {
                        x: guard_position.x,
                        y: guard_position.y - 1,
                    };

                    // If position valid, move. If not, rotate
                    if is_position_valid(grid, &new_pos) {
                        guard_position = new_pos;
                        unique_spaces_visited.insert(new_pos);
                    } else {
                        guard_dir = GuardDir::Right;
                    }
                }
            }

            GuardDir::Down => {
                // If out of bounds we are DONE
                if guard_position.y + 1 > max_y {
                    on_map = false;
                } else {
                    let new_pos = GridPos {
                        x: guard_position.x,
                        y: guard_position.y + 1,
                    };

                    // If position valid, move. If not, rotate
                    if is_position_valid(grid, &new_pos) {
                        guard_position = new_pos;
                        unique_spaces_visited.insert(new_pos);
                    } else {
                        guard_dir = GuardDir::Left;
                    }
                }
            }

            GuardDir::Left => {
                // If out of bounds we are DONE
                if Option::is_none(&guard_position.x.checked_sub(1)) {
                    on_map = false;
                } else {
                    let new_pos = GridPos {
                        x: guard_position.x - 1,
                        y: guard_position.y,
                    };

                    // If position valid, move. If not, rotate
                    if is_position_valid(grid, &new_pos) {
                        guard_position = new_pos;
                        unique_spaces_visited.insert(new_pos);
                    } else {
                        guard_dir = GuardDir::Up;
                    }
                }
            }

            GuardDir::Right => {
                // If out of bounds we are DONE
                if guard_position.x + 1 > max_x {
                    on_map = false;
                } else {
                    let new_pos = GridPos {
                        x: guard_position.x + 1,
                        y: guard_position.y,
                    };

                    // If position valid, move. If not, rotate
                    if is_position_valid(grid, &new_pos) {
                        guard_position = new_pos;
                        unique_spaces_visited.insert(new_pos);
                    } else {
                        guard_dir = GuardDir::Down;
                    }
                }
            }
        }

        // Check here in case we are in an infinite loop
        if old_unique_spaces_count == unique_spaces_visited.len() {
            loop_check_count += 1;
            if loop_check_count == 200 {
                println!("LOOPED WITH 5 SPECS GONE TO");
                return HashSet::new();
            }
        } else {
            loop_check_count = 0;
            old_unique_spaces_count = unique_spaces_visited.len();
        }
    }
    unique_spaces_visited
}

pub fn part_one(input: &str) -> Option<u32> {
    let input_grid = advent_of_code::parse_into_grid(input);
    advent_of_code::print_2d_vec(&input_grid);

    // Find initial position of the guard
    let mut guard_position = advent_of_code::GridPos::default();
    let mut guard_pos_set: bool = false;
    for (y, row) in input_grid.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if col == "^" {
                guard_position.x = x;
                guard_position.y = y;
                guard_pos_set = true;
                break;
            }
        }
        if guard_pos_set {
            break;
        }
    }
    println!("Guard starting position: {:?}", guard_position);

    let mut on_map: bool = true;
    let mut guard_dir = GuardDir::Up;
    let max_y = input_grid.len() - 1;
    let max_x = input_grid.first().unwrap_or(&Vec::new()).len() - 1;
    let mut unique_spaces_visited: HashSet<GridPos> = HashSet::new();
    unique_spaces_visited.insert(guard_position);
    while on_map {
        println!("Guard at {:?}", guard_position);

        // Check the position in front of the guard
        match guard_dir {
            GuardDir::Up => {
                // If out of bounds we are DONE
                if Option::is_none(&guard_position.y.checked_sub(1)) {
                    on_map = false;
                } else {
                    let new_pos = GridPos {
                        x: guard_position.x,
                        y: guard_position.y - 1,
                    };

                    // If position valid, move. If not, rotate
                    if is_position_valid(&input_grid, &new_pos) {
                        guard_position = new_pos;
                        unique_spaces_visited.insert(new_pos);
                    } else {
                        guard_dir = GuardDir::Right;
                    }
                }
            }

            GuardDir::Down => {
                // If out of bounds we are DONE
                if guard_position.y + 1 > max_y {
                    on_map = false;
                } else {
                    let new_pos = GridPos {
                        x: guard_position.x,
                        y: guard_position.y + 1,
                    };

                    // If position valid, move. If not, rotate
                    if is_position_valid(&input_grid, &new_pos) {
                        guard_position = new_pos;
                        unique_spaces_visited.insert(new_pos);
                    } else {
                        guard_dir = GuardDir::Left;
                    }
                }
            }

            GuardDir::Left => {
                // If out of bounds we are DONE
                if Option::is_none(&guard_position.x.checked_sub(1)) {
                    on_map = false;
                } else {
                    let new_pos = GridPos {
                        x: guard_position.x - 1,
                        y: guard_position.y,
                    };

                    // If position valid, move. If not, rotate
                    if is_position_valid(&input_grid, &new_pos) {
                        guard_position = new_pos;
                        unique_spaces_visited.insert(new_pos);
                    } else {
                        guard_dir = GuardDir::Up;
                    }
                }
            }

            GuardDir::Right => {
                // If out of bounds we are DONE
                if guard_position.x + 1 > max_x {
                    on_map = false;
                } else {
                    let new_pos = GridPos {
                        x: guard_position.x + 1,
                        y: guard_position.y,
                    };

                    // If position valid, move. If not, rotate
                    if is_position_valid(&input_grid, &new_pos) {
                        guard_position = new_pos;
                        unique_spaces_visited.insert(new_pos);
                    } else {
                        guard_dir = GuardDir::Down;
                    }
                }
            }
        }
    }
    println!("Guard visited {} places", unique_spaces_visited.len());
    Some(unique_spaces_visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input_grid = advent_of_code::parse_into_grid(input);
    advent_of_code::print_2d_vec(&input_grid);

    // Find initial position of the guard
    let mut guard_position = advent_of_code::GridPos::default();
    let mut guard_pos_set: bool = false;
    for (y, row) in input_grid.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if col == "^" {
                guard_position.x = x;
                guard_position.y = y;
                guard_pos_set = true;
                break;
            }
        }
        if guard_pos_set {
            break;
        }
    }
    println!("Guard starting position: {:?}", guard_position);
    let original_unique_places = check_guard_unique_places(&input_grid, guard_position);
    let mut loop_count: u32 = 0;
    println!("STARTING TO ADD OBSTACLEZ");
    for visit in original_unique_places {
        let mut new_grid = input_grid.clone();

        // Try and place an obstacle here, see if the guard gets stuck
        new_grid[visit.y][visit.x] = "#".to_string();
        if check_guard_unique_places(&new_grid, guard_position).is_empty() {
            println!("Length of unique places = 0 meaning guard got stuck in a loop");
            loop_count += 1;
        }
    }
    Some(loop_count)
    // DAY 2 LOGIC
    //

    // Loop over places guard visits, add an obstacle
    // do the main loop again (functionise) to see if he gets stuck
    // what define as stuck? no more unique spaces visited after ~30 iterations? and not off map obviously
    // try lower (10) and go higher (100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
