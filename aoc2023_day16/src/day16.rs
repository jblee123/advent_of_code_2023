#[derive(Debug, PartialEq, Clone, PartialOrd, Default)]
struct Space {
    seen_dirs: [bool; 4],
}

type Board = Vec<Vec<Space>>;

#[derive(Debug, PartialEq, Copy, Clone)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, Copy, Clone)]
enum Dir {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Dir {
    pub fn is_horizontal(&self) -> bool {
        *self == Dir::Left || *self == Dir::Right
    }

    pub fn is_vertical(&self) -> bool {
        *self == Dir::Up || *self == Dir::Down
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Beam {
    coord: Coord,
    dir: Dir,
}

impl Beam {
    pub fn new(row: usize, col: usize, dir: Dir) -> Self {
        Self {
            coord: Coord::new(row, col),
            dir,
        }
    }
}

fn parse_input(s: &str) -> Vec<String> {
    s.lines().map(|line| line.to_string()).collect()
}

fn create_board(rows: usize, cols: usize) -> Board {
    vec![vec![Space::default(); cols]; rows]
}

fn move_beam_up(beam: &Beam) -> Option<Beam> {
    let row = beam.coord.row;
    let col = beam.coord.col;
    if row > 0 {
        Some(Beam::new(row - 1, col, Dir::Up))
    } else {
        None
    }
}

fn move_beam_down(beam: &Beam, max_rows: usize) -> Option<Beam> {
    let row = beam.coord.row;
    let col = beam.coord.col;
    if row < (max_rows - 1) {
        Some(Beam::new(row + 1, col, Dir::Down))
    } else {
        None
    }
}

fn move_beam_left(beam: &Beam) -> Option<Beam> {
    let row = beam.coord.row;
    let col = beam.coord.col;
    if col > 0 {
        Some(Beam::new(row, col - 1, Dir::Left))
    } else {
        None
    }
}

fn move_beam_right(beam: &Beam, max_cols: usize) -> Option<Beam> {
    let row = beam.coord.row;
    let col = beam.coord.col;
    if col < (max_cols - 1) {
        Some(Beam::new(row, col + 1, Dir::Right))
    } else {
        None
    }
}

fn step_beam(beam: &Beam, optics: &Vec<String>) -> (Option<Beam>, Option<Beam>) {
    let row = beam.coord.row;
    let col = beam.coord.col;
    let max_rows = optics.len();
    let max_cols = optics[0].len();
    let optic = optics[row].chars().nth(col).unwrap();
    if beam.dir.is_horizontal() && (optic == '|') {
        let new_beam1 = move_beam_up(beam);
        let new_beam2 = move_beam_down(beam, max_rows);
        return (new_beam1, new_beam2);
    }

    if beam.dir.is_vertical() && (optic == '-') {
        let new_beam1 = move_beam_left(beam);
        let new_beam2 = move_beam_right(beam, max_cols);
        return (new_beam1, new_beam2);
    }

    let go_up =
        (beam.dir == Dir::Left && optic == '\\') || (beam.dir == Dir::Right && optic == '/');
    if go_up {
        return (move_beam_up(beam), None);
    }

    let go_down =
        (beam.dir == Dir::Left && optic == '/') || (beam.dir == Dir::Right && optic == '\\');
    if go_down {
        return (move_beam_down(beam, max_rows), None);
    }

    let go_left = (beam.dir == Dir::Up && optic == '\\') || (beam.dir == Dir::Down && optic == '/');
    if go_left {
        return (move_beam_left(beam), None);
    }

    let go_right =
        (beam.dir == Dir::Up && optic == '/') || (beam.dir == Dir::Down && optic == '\\');
    if go_right {
        return (move_beam_right(beam, max_cols), None);
    }

    match beam.dir {
        Dir::Up => (move_beam_up(beam), None),
        Dir::Down => (move_beam_down(beam, max_rows), None),
        Dir::Left => (move_beam_left(beam), None),
        Dir::Right => (move_beam_right(beam, max_cols), None),
    }
}

fn filter_beams(board: &mut Board, beams: &[Beam]) -> Vec<Beam> {
    beams
        .iter()
        .filter(|beam| {
            let space = &board[beam.coord.row][beam.coord.col];
            !space.seen_dirs[beam.dir as usize]
        })
        .map(|beam| beam.clone())
        .collect()
}

fn update_board(board: &mut Board, beams: &[Beam]) {
    beams.iter().for_each(|beam| {
        let space = &mut board[beam.coord.row][beam.coord.col];
        space.seen_dirs[beam.dir as usize] = true;
    });
}

fn step(board: &mut Board, beams: &[Beam], optics: &Vec<String>) -> Vec<Beam> {
    let mut new_beams = vec![];

    beams.iter().for_each(|beam| {
        let (beam1, beam2) = step_beam(beam, optics);
        if let Some(beam1) = beam1 {
            new_beams.push(beam1);
        }
        if let Some(beam2) = beam2 {
            new_beams.push(beam2);
        }
    });

    let new_beams = filter_beams(board, &new_beams);
    update_board(board, &new_beams);

    new_beams
}

fn space_is_energized(space: &Space) -> bool {
    space.seen_dirs.iter().any(|seen| *seen)
}

fn count_energized(board: &Board) -> u32 {
    board
        .iter()
        .map(|row| row.iter().filter(|space| space_is_energized(space)).count() as u32)
        .sum()
}

fn board_to_energized_str(board: &Board) -> String {
    let row_strs = board
        .iter()
        .map(|row| {
            row.iter()
                .map(|space| if space_is_energized(space) { '#' } else { '.' })
                .collect::<String>()
        })
        .collect::<Vec<String>>();

    row_strs.join("\n")
}

#[allow(dead_code)]
fn print_board_energized(board: &Board) {
    println!("\n{}", board_to_energized_str(board));
}

fn get_num_energized(optics: &Vec<String>, first_beam: &Beam) -> u32 {
    let mut board = create_board(optics.len(), optics[0].len());
    let mut beams = vec![first_beam.clone()];

    update_board(&mut board, &beams);
    // print_board_energized(&board);

    while !beams.is_empty() {
        beams = step(&mut board, &beams, &optics);
        // print_board_energized(&board);
    }

    count_energized(&board)
}

pub fn get_num_energized_from_input(s: &str) -> u32 {
    let optics = parse_input(s);
    get_num_energized(&optics, &Beam::new(0, 0, Dir::Right))
}

pub fn get_max_num_energized_from_input(s: &str) -> u32 {
    let optics = parse_input(s);

    let num_rows = optics.len();
    let num_cols = optics[0].len();

    let mut max_energized = 0 as u32;

    (0..num_cols).for_each(|col| {
        let energized = get_num_energized(&optics, &Beam::new(0, col, Dir::Down));
        max_energized = max_energized.max(energized);

        let energized = get_num_energized(&optics, &Beam::new(num_rows - 1, col, Dir::Up));
        max_energized = max_energized.max(energized);
    });

    (0..num_rows).for_each(|row| {
        let energized = get_num_energized(&optics, &Beam::new(row, 0, Dir::Right));
        max_energized = max_energized.max(energized);

        let energized = get_num_energized(&optics, &Beam::new(row, num_cols - 1, Dir::Left));
        max_energized = max_energized.max(energized);
    });

    max_energized
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_1: &str = concat!(
        ".|...\\....\n",
        "|.-.\\.....\n",
        ".....|-...\n",
        "........|.\n",
        "..........\n",
        ".........\\\n",
        "..../.\\\\..\n",
        ".-.-/..|..\n",
        ".|....-|.\\\n",
        "..//.|....\n",
    );

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(SAMPLE_INPUT_1),
            vec![
                ".|...\\....".to_string(),
                "|.-.\\.....".to_string(),
                ".....|-...".to_string(),
                "........|.".to_string(),
                "..........".to_string(),
                ".........\\".to_string(),
                "..../.\\\\..".to_string(),
                ".-.-/..|..".to_string(),
                ".|....-|.\\".to_string(),
                "..//.|....".to_string(),
            ]
        );
    }

    #[test]
    fn test_get_num_energized_from_input() {
        println!();
        assert_eq!(get_num_energized_from_input(SAMPLE_INPUT_1), 46);
    }

    #[test]
    fn test_get_max_num_energized_from_input() {
        println!();
        assert_eq!(get_max_num_energized_from_input(SAMPLE_INPUT_1), 51);
    }
}
