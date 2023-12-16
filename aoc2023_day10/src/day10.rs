#[derive(Debug, PartialEq, Copy, Clone, PartialOrd)]
#[repr(usize)]
enum Pipe {
    None = 0,
    NorthSouth = 1,
    EastWest = 2,
    NorthEast = 3,
    NorthWest = 4,
    SouthEast = 5,
    SouthWest = 6,
}

#[derive(Debug, PartialEq, Copy, Clone, PartialOrd)]
#[repr(usize)]
enum CardinalDir {
    North = 0,
    South = 1,
    East = 2,
    West = 3,
}

#[derive(Debug, PartialEq, Copy, Clone, PartialOrd)]
struct Coord {
    row: usize,
    col: usize,
}

type PipeMap = Vec<Vec<Pipe>>;
type DrawnMap = Vec<Vec<u8>>;

fn symbol_to_pipe(c: char) -> Pipe {
    match c {
        '|' => Pipe::NorthSouth,
        '-' => Pipe::EastWest,
        'L' => Pipe::NorthEast,
        'J' => Pipe::NorthWest,
        '7' => Pipe::SouthWest,
        'F' => Pipe::SouthEast,
        '.' => Pipe::None,
        _ => Pipe::None,
    }
}

fn get_neighbors(pipe_map: &PipeMap, coord: Coord) -> [Pipe; 4] {
    [
        pipe_map[coord.row - 1][coord.col],
        pipe_map[coord.row + 1][coord.col],
        pipe_map[coord.row][coord.col + 1],
        pipe_map[coord.row][coord.col - 1],
    ]
}

fn is_connected_from_north(neighbors: &[Pipe; 4]) -> bool {
    match neighbors[CardinalDir::North as usize] {
        Pipe::NorthSouth | Pipe::SouthEast | Pipe::SouthWest => true,
        _ => false,
    }
}

fn is_connected_from_south(neighbors: &[Pipe; 4]) -> bool {
    match neighbors[CardinalDir::South as usize] {
        Pipe::NorthSouth | Pipe::NorthEast | Pipe::NorthWest => true,
        _ => false,
    }
}

fn is_connected_from_east(neighbors: &[Pipe; 4]) -> bool {
    match neighbors[CardinalDir::East as usize] {
        Pipe::EastWest | Pipe::NorthWest | Pipe::SouthWest => true,
        _ => false,
    }
}

fn is_connected_from_west(neighbors: &[Pipe; 4]) -> bool {
    match neighbors[CardinalDir::West as usize] {
        Pipe::EastWest | Pipe::NorthEast | Pipe::SouthEast => true,
        _ => false,
    }
}

fn connects_north(pipe: Pipe) -> bool {
    match pipe {
        Pipe::NorthSouth | Pipe::NorthEast | Pipe::NorthWest => true,
        _ => false,
    }
}

fn connects_south(pipe: Pipe) -> bool {
    match pipe {
        Pipe::NorthSouth | Pipe::SouthEast | Pipe::SouthWest => true,
        _ => false,
    }
}

fn connects_east(pipe: Pipe) -> bool {
    match pipe {
        Pipe::EastWest | Pipe::NorthEast | Pipe::SouthEast => true,
        _ => false,
    }
}

fn connects_west(pipe: Pipe) -> bool {
    match pipe {
        Pipe::EastWest | Pipe::NorthWest | Pipe::SouthWest => true,
        _ => false,
    }
}

fn derive_pipe(pipe_map: &PipeMap, coord: Coord) -> Pipe {
    let neighbors = get_neighbors(pipe_map, coord);
    let connected_north = is_connected_from_north(&neighbors);
    let connected_south = is_connected_from_south(&neighbors);
    let connected_east = is_connected_from_east(&neighbors);
    let connected_west = is_connected_from_west(&neighbors);

    if connected_north && connected_east {
        Pipe::NorthEast
    } else if connected_north && connected_south {
        Pipe::NorthSouth
    } else if connected_north && connected_west {
        Pipe::NorthWest
    } else if connected_south && connected_east {
        Pipe::SouthEast
    } else if connected_south && connected_west {
        Pipe::SouthWest
    } else if connected_east && connected_west {
        Pipe::EastWest
    } else {
        Pipe::None
    }
}

fn parse_row(row: &str) -> (Vec<Pipe>, Option<usize>) {
    let mut start_idx: Option<usize> = None;
    let mut pipes = Vec::<Pipe>::default();
    pipes.reserve(row.len() + 2);
    pipes.push(Pipe::None);
    for i in 0..row.len() {
        let c = row.as_bytes()[i];
        let pipe = if c == 'S' as u8 {
            start_idx = Some(i + 1);
            Pipe::None
        } else {
            symbol_to_pipe(row.as_bytes()[i] as char)
        };
        pipes.push(pipe);
    }
    pipes.push(Pipe::None);

    (pipes, start_idx)
}

fn parse_input(s: &str) -> (PipeMap, Coord) {
    let mut start_coord = Coord { row: 0, col: 0 };
    let mut pipe_map = PipeMap::default();
    s.lines().enumerate().for_each(|(row_idx, line)| {
        if pipe_map.is_empty() {
            pipe_map.push(vec![Pipe::None; line.len() + 2]);
        }

        let (row, start_col_idx_opt) = parse_row(line);
        if start_col_idx_opt.is_some() {
            start_coord.row = row_idx + 1;
            start_coord.col = start_col_idx_opt.unwrap();
        }

        pipe_map.push(row);
    });
    pipe_map.push(pipe_map[0].clone());

    pipe_map[start_coord.row][start_coord.col] = derive_pipe(&pipe_map, start_coord);

    (pipe_map, start_coord)
}

fn get_next_pipe_coord(pipe_map: &PipeMap, at: Coord, prev: Coord) -> Coord {
    let pipe = pipe_map[at.row][at.col];
    let connected_north = connects_north(pipe);
    let connected_south = connects_south(pipe);
    let connected_east = connects_east(pipe);
    let connected_west = connects_west(pipe);
    let came_from_north = prev.row < at.row;
    let came_from_south = prev.row > at.row;
    let came_from_east = prev.col > at.col;
    let came_from_west = prev.col < at.col;
    if connected_north && !came_from_north {
        Coord {
            row: at.row - 1,
            col: at.col,
        }
    } else if connected_south && !came_from_south {
        Coord {
            row: at.row + 1,
            col: at.col,
        }
    } else if connected_east && !came_from_east {
        Coord {
            row: at.row,
            col: at.col + 1,
        }
    } else if connected_west && !came_from_west {
        Coord {
            row: at.row,
            col: at.col - 1,
        }
    } else {
        at
    }
}

fn get_max_dist(pipe_map: &PipeMap, start: Coord) -> usize {
    let mut steps = 0 as usize;

    let mut at = start;
    let mut prev = start;

    loop {
        let next = get_next_pipe_coord(pipe_map, at, prev);
        steps += 1;
        if next == start {
            break;
        }

        prev = at;
        at = next;
    }

    steps / 2
}

pub fn get_max_dist_from_input(s: &str) -> usize {
    let (pipe_map, start) = parse_input(s);
    get_max_dist(&pipe_map, start)
}

fn gen_clear_drawn_map(pipe_map: &PipeMap) -> DrawnMap {
    pipe_map
        .iter()
        .map(|row| vec!['.' as u8; row.len()])
        .collect::<DrawnMap>()
}

fn draw_map(drawn_map: &DrawnMap) {
    drawn_map.iter().for_each(|row| {
        println!("{}", String::from_utf8(row.clone()).unwrap());
    });
}

fn get_num_enclosing_tiles(pipe_map: &PipeMap, start: Coord) -> usize {
    let mut drawn_map = gen_clear_drawn_map(pipe_map);

    let mut at = start;
    let mut prev = start;

    loop {
        drawn_map[at.row][at.col] = '*' as u8;
        let next = get_next_pipe_coord(pipe_map, at, prev);
        if next == start {
            break;
        }

        prev = at;
        at = next;
    }

    let mut num_inside_spaces = 0 as usize;

    let num_rows = pipe_map.len();
    let num_cols = pipe_map[0].len();

    let mut top_col_idx = -(num_cols as isize) + 1;
    while top_col_idx < num_cols as isize {
        let mut inside = false;

        for i in 0..num_rows {
            let col_idx = top_col_idx + i as isize;
            let row_idx = i;
            if (col_idx < 0) || (col_idx >= num_cols as isize) {
                continue;
            }

            let pipe = pipe_map[row_idx][col_idx as usize];
            let drawn = drawn_map[row_idx][col_idx as usize];
            let is_open = drawn == '.' as u8;
            let ignore_pipe = (pipe == Pipe::NorthEast) || (pipe == Pipe::SouthWest);
            let is_part_of_loop = drawn == '*' as u8;

            if is_open && inside {
                num_inside_spaces += 1;
                drawn_map[row_idx][col_idx as usize] = 'I' as u8;
            } else if is_part_of_loop && !ignore_pipe {
                inside = !inside;
            }
        }

        top_col_idx += 1;
    }

    draw_map(&drawn_map);

    num_inside_spaces
}

pub fn get_num_enclosing_tiles_from_input(s: &str) -> usize {
    let (pipe_map, start) = parse_input(s);
    get_num_enclosing_tiles(&pipe_map, start)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT1: &str = concat!(
        ".....\n", //
        ".S-7.\n", //
        ".|.|.\n", //
        ".L-J.\n", //
        ".....\n", //
    );

    const SAMPLE_INPUT2: &str = concat!(
        "..F7.\n", //
        ".FJ|.\n", //
        "SJ.L7\n", //
        "|F--J\n", //
        "LJ...\n", //
    );

    #[test]
    fn test_get_max_dist_from_input() {
        assert_eq!(get_max_dist_from_input(SAMPLE_INPUT1), 4);
        assert_eq!(get_max_dist_from_input(SAMPLE_INPUT2), 8);
    }

    // #[test]
    // fn test_get_max_dist_from_real_input() {
    //     println!("CWD: {:?}", std::env::current_dir().unwrap());
    //     let input = aoc2023_utils::get_input("../inputs/day10.txt");
    //     assert_eq!(get_max_dist_from_input(&input), 4);
    // }

    // const SAMPLE_INPUT3: &str = concat!(
    //     "...........\n",
    //     ".S-------7.\n",
    //     ".|F-----7|.\n",
    //     ".||.....||.\n",
    //     ".||.....||.\n",
    //     ".|L-7.F-J|.\n",
    //     ".|..|.|..|.\n",
    //     ".L--J.L--J.\n",
    //     "...........\n",
    // );

    // const SAMPLE_INPUT4: &str = concat!(
    //     ".F----7F7F7F7F-7....\n",
    //     ".|F--7||||||||FJ....\n",
    //     ".||.FJ||||||||L7....\n",
    //     "FJL7L7LJLJ||LJ.L-7..\n",
    //     "L--J.L7...LJS7F-7L7.\n",
    //     "....F-J..F7FJ|L7L7L7\n",
    //     "....L7.F7||L7|.L7L7|\n",
    //     ".....|FJLJ|FJ|F7|.LJ\n",
    //     "....FJL-7.||.||||...\n",
    //     "....L---J.LJ.LJLJ...\n",
    // );

    // const SAMPLE_INPUT5: &str = concat!(
    //     "FF7FSF7F7F7F7F7F---7\n",
    //     "L|LJ||||||||||||F--J\n",
    //     "FL-7LJLJ||||||LJL-77\n",
    //     "F--JF--7||LJLJ7F7FJ-\n",
    //     "L---JF-JLJ.||-FJLJJ7\n",
    //     "|F|F-JF---7F7-L7L|7|\n",
    //     "|FFJF7L7F-JF7|JL---7\n",
    //     "7-L-JL7||F7|L7F-7F7|\n",
    //     "L.L7LFJ|||||FJL7||LJ\n",
    //     "L7JLJL-JLJLJL--JLJ.L\n",
    // );

    // #[test]
    // fn test_get_line_diffs() {
    //     let (pipe_map, start) = parse_input(SAMPLE_INPUT5);

    //     let result = get_num_enclosing_tiles(&pipe_map, start);
    //     println!("result: {}", result);

    //     // let drawn_map = gen_clear_drawn_map(&pipe_map);
    //     // draw_map(&drawn_map);
    //     // assert!(false);
    // }
}
