pub mod day07;

fn main() {
    let input = aoc2023_utils::get_input("inputs/day07.txt");
    let result = day07::get_winnings(&input);
    println!("{}", result);
}
