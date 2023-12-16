pub mod day11;

fn main() {
    let input = aoc2023_utils::get_input("inputs/day11.txt");
    let result = day11::get_sum_of_galaxy_dists(&input, 1000000);
    println!("{}", result);
}
