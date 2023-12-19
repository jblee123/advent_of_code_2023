pub mod day15;

fn main() {
    let input = aoc2023_utils::get_input("inputs/day15.txt");
    let result = day15::parse_and_sum_step_hashes(&input);
    println!("{}", result);
}
