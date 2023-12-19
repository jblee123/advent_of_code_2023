pub mod day14;

fn main() {
    let input = aoc2023_utils::get_input("inputs/day14.txt");
    let result = day14::get_cycled_summary_from_input(&input, 1000000000);
    println!("{}", result);
}
