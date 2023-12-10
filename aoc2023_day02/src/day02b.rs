pub mod day02;

fn main() {
    let input = aoc2023_utils::get_input("inputs/day02.txt");
    let result = day02::get_sum_of_power_of_min_possible_sets(&input);
    println!("{}", result);
}
