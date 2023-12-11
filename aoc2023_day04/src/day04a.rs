pub mod day04;

fn main() {
    let input = aoc2023_utils::get_input("inputs/day04.txt");
    let result = day04::get_points_for_cards(&input);
    println!("{}", result);
}
