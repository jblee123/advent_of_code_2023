use std::collections::HashMap;

type Board = Vec<Vec<u8>>;

#[allow(dead_code)]
fn print_board(board: &Board) {
    board.iter().for_each(|line| {
        let line_str = String::from_utf8(line.clone()).unwrap();
        println!("{line_str}");
    });
}

fn parse_input(s: &str) -> Board {
    s.lines().map(|line| line.as_bytes().to_vec()).collect()
}

fn get_summary(board: &Board) -> u32 {
    let mut load = 0;
    let num_rows = board.len();
    let num_cols = board[0].len();
    for col in 0..num_cols {
        let mut load_base = num_rows;
        let mut num_stones = 0;
        for row in 0..num_rows {
            let c = board[row][col] as char;
            if c == '#' {
                for stone in 0..num_stones {
                    load += load_base - stone;
                }
                load_base = num_rows - row - 1;
                num_stones = 0;
            } else if c == 'O' {
                num_stones += 1;
            }
        }
        for stone in 0..num_stones {
            load += load_base - stone;
        }
    }

    load as u32
}

pub fn get_summary_from_input(input: &str) -> u32 {
    get_summary(&parse_input(input))
}

fn get_load(board: &Board) -> u32 {
    let mut load = 0;
    let num_rows = board.len();
    let num_cols = board[0].len();
    for col in 0..num_cols {
        for row in 0..num_rows {
            let c = board[row][col] as char;
            if c == 'O' {
                load += num_rows - row;
            }
        }
    }

    load as u32
}

fn roll_to_top(board: &mut Board) {
    let num_rows = board.len();
    let num_cols = board[0].len();
    for col in 0..num_cols {
        let mut start_row = 0;
        let mut num_stones = 0;
        for row in 0..num_rows {
            let c = board[row][col] as char;
            if c == '#' {
                start_row = row + 1;
                num_stones = 0;
            } else if c == 'O' {
                let target_row = start_row + num_stones;
                if target_row != row {
                    board[target_row][col] = 'O' as u8;
                    board[row][col] = '.' as u8;
                }
                num_stones += 1;
            }
        }
    }
}

fn rotate_board_ccw(board: &Board) -> Board {
    let num_rows = board.len();
    let num_cols = board[0].len();

    let mut new_board = (0..num_cols)
        .map(|_| {
            std::iter::repeat(' ' as u8)
                .take(num_rows)
                .collect::<Vec<u8>>()
        })
        .collect::<Board>();

    for row in 0..num_rows {
        for col in 0..num_cols {
            let new_row = col;
            let new_col = num_rows - row - 1;
            new_board[new_row][new_col] = board[row][col];
        }
    }

    new_board
}

fn run_cycle(board: &Board) -> Board {
    let mut board = board.clone();
    (0..4).for_each(|_| {
        roll_to_top(&mut board);
        board = rotate_board_ccw(&board);
    });
    board
}

fn board_to_str(board: &Board) -> String {
    let row_strs = board
        .iter()
        .map(|line| String::from_utf8(line.clone()).unwrap())
        .collect::<Vec<String>>();

    row_strs.join("\n")
}

fn run_n_cycles(board: &Board, num_cycles: u32) -> Board {
    let mut history = HashMap::<String, u32>::new();

    let mut board = board.clone();
    let mut cycles_to_go = num_cycles;
    let mut cyclic_length = 0;

    let hash_str = board_to_str(&board);
    history.insert(hash_str, 0);

    for i in 0..num_cycles {
        board = run_cycle(&board);
        let hash_str = board_to_str(&board);
        cycles_to_go -= 1;
        if let Some(prev_cycle) = history.insert(hash_str, i + 1) {
            cyclic_length = (i + 1) - prev_cycle;
            println!("Found a repeat on idx {i} of cycle {prev_cycle} for a cyclic length of {cyclic_length}");
            break;
        }
    };

    if cycles_to_go > 0 {
        let old_cycles_to_go = cycles_to_go;
        cycles_to_go = cycles_to_go % cyclic_length;
        println!("Skipping from {old_cycles_to_go} cycles to go down to {cycles_to_go} cycles to go");
    }

    (0..cycles_to_go).for_each(|_| {
        board = run_cycle(&board);
    });

    board
}

pub fn get_cycled_summary_from_input(input: &str, num_cycles: u32) -> u32 {
    let board = run_n_cycles(&parse_input(input), num_cycles);
    get_load(&board)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_1: &str = concat!(
        "O....#....\n",
        "O.OO#....#\n",
        ".....##...\n",
        "OO.#O....O\n",
        ".O.....O#.\n",
        "O.#..O.#.#\n",
        "..O..#O..O\n",
        ".......O..\n",
        "#....###..\n",
        "#OO..#....\n",
    );

    const SAMPLE_INPUT_1_ROLLED: &str = concat!(
        "OOOO.#.O..\n",
        "OO..#....#\n",
        "OO..O##..O\n",
        "O..#.OO...\n",
        "........#.\n",
        "..#....#.#\n",
        "..O..#.O.O\n",
        "..O.......\n",
        "#....###..\n",
        "#....#....\n",
    );

    const SAMPLE_INPUT_1_ROTATED_CCW: &str = concat!(
        "##..O.O.OO\n",
        "O....OO...\n",
        "O..O#...O.\n",
        "......#.O.\n",
        "......O.#.\n",
        "##.#O..#.#\n",
        ".#.O...#..\n",
        ".#O.#O....\n",
        ".....#....\n",
        "...O#.O.#.\n",
    );

    #[test]
    fn test_get_summary() {
        assert_eq!(get_summary(&parse_input(SAMPLE_INPUT_1)), 136);
    }

    #[test]
    fn test_get_load() {
        assert_eq!(get_load(&parse_input(SAMPLE_INPUT_1_ROLLED)), 136);
    }

    #[test]
    fn test_roll_to_top() {
        let mut board = parse_input(SAMPLE_INPUT_1);
        roll_to_top(&mut board);
        // println!();
        // print_board(&board);
        assert_eq!(board, parse_input(SAMPLE_INPUT_1_ROLLED));
    }

    #[test]
    fn test_rotate_board_ccw() {
        let board = parse_input(SAMPLE_INPUT_1);
        assert_eq!(
            rotate_board_ccw(&board),
            parse_input(SAMPLE_INPUT_1_ROTATED_CCW)
        );
    }

    #[test]
    fn test_run_n_cycles() {
        println!();
        let board = parse_input(SAMPLE_INPUT_1);
        let board = run_n_cycles(&board, 1000000000);
        let load = get_load(&board);
        assert_eq!(load, 64);
    }
}
