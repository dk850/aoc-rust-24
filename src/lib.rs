pub mod template;

// Use this file to add helper functions and additional modules.
pub fn print_2d_vec(input: &[Vec<String>]) {
    for line in input.iter() {
        println!("{:?}", line);
    }
}
