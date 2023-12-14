pub mod day05;

fn main() {
    let input = aoc2023_utils::get_input("inputs/day05.txt");
    let result = day05::get_lowest_loc_for_seed_ranges(&input);
    println!("{}", result);
}
