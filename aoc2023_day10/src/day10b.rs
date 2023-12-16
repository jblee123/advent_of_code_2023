pub mod day10;

fn main() {
    let input = aoc2023_utils::get_input("inputs/day10.txt");
    let result = day10::get_num_enclosing_tiles_from_input(&input);
    println!("{}", result);
}
