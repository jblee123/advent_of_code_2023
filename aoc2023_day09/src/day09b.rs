pub mod day09;

fn main() {
    let input = aoc2023_utils::get_input("inputs/day09.txt");
    let result = day09::sum_extrapolated_back_values(&input);
    println!("{}", result);
}
