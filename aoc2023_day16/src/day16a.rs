pub mod day16;

fn main() {
    let input = aoc2023_utils::get_input("inputs/day16.txt");
    let result = day16::get_num_energized_from_input(&input);
    println!("{}", result);
}
