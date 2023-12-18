pub mod day13;

fn main() {
    let input = aoc2023_utils::get_input("inputs/day13.txt");
    let result = day13::get_smudged_summary_val(&input);
    println!("{}", result);
}
