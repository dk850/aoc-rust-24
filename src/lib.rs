pub mod template;

#[derive(Debug, PartialEq, Clone, Default, Hash, Eq, Copy)]
pub struct GridPos {
    pub x: usize,
    pub y: usize,
}

// Use this file to add helper functions and additional modules.
pub fn print_2d_vec(input: &[Vec<String>]) {
    for line in input.iter() {
        println!("{:?}", line);
    }
}

pub fn parse_into_grid(input: &str) -> Vec<Vec<String>> {
    let mut output: Vec<Vec<String>> = Vec::new();

    // Build grid
    for line in input.lines() {
        output.push(line.chars().map(String::from).collect());
    }
    output
}
