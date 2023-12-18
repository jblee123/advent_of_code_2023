pub mod day12;

fn main() {
    let input = aoc2023_utils::get_input("inputs/day12.txt");
    let result = day12::get_sum_of_num_good_configs_unfolded(&input);
    println!("{}", result);
}
