pub mod day08;

fn main() {
    let input = aoc2023_utils::get_input("inputs/day08.txt");
    let result = day08::get_ghost_traversal_steps(&input);
    println!("{}", result);
}
