pub mod day01;

fn main() {
    let input = aoc2023_utils::get_input("inputs/day01.txt");
    let result = day01::sum_calibration_values2(&input);
    println!("{}", result);
}
