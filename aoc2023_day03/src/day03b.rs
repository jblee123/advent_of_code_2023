pub mod day03;

fn main() {
    let input = aoc2023_utils::get_input("inputs/day03.txt");
    let result = day03::get_sum_of_gear_ratios(&input);
    println!("{}", result);
}
