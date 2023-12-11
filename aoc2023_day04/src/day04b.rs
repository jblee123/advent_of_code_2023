pub mod day04;

fn main() {
    let input = aoc2023_utils::get_input("inputs/day04.txt");
    let result = day04::get_num_cards_after_rewinning(&input);
    println!("{}", result);
}
