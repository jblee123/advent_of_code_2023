pub mod day15;

fn main() {
    let input = aoc2023_utils::get_input("inputs/day15.txt");
    let result = day15::process_input(&input);
    println!("{}", result);
}
