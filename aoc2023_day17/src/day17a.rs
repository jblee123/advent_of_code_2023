pub mod day17;

fn main() {
    let input = aoc2023_utils::get_input("inputs/day17.txt");
    let result = day17::get_min_disipation_from_input(&input);
    println!("{}", result);
}
