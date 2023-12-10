pub mod day02;

fn main() {
    let input = aoc2023_utils::get_input("inputs/day02.txt");
    let result = day02::get_sum_of_possible_game_ids(&input, 12, 13, 14);
    println!("{}", result);
}
