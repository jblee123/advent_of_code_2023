pub mod day06;

fn main() {
    let input = aoc2023_utils::get_input("inputs/day06.txt");
    let result = day06::get_product_of_ways_to_win(&input);
    println!("{}", result);
}
