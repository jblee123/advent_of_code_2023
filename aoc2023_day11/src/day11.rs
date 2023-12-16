#[derive(Debug, PartialEq, Copy, Clone, PartialOrd)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

type SkyMap = Vec<Coord>;

fn parse_input(s: &str) -> SkyMap {
    let mut coords = vec![];
    s.lines().enumerate().for_each(|(row_idx, line)| {
        line.chars().enumerate().for_each(|(col_idx, c)| {
            if c == '#' {
                coords.push(Coord::new(row_idx, col_idx));
            }
        });
    });

    coords
}

fn get_expanded_rows_and_cols(sky_map: &SkyMap) -> (Vec<usize>, Vec<usize>) {
    let row_nums = sky_map
        .iter()
        .map(|coord| coord.row)
        .collect::<Vec<usize>>();
    let col_nums = sky_map
        .iter()
        .map(|coord| coord.col)
        .collect::<Vec<usize>>();

    let max_row = *row_nums.iter().max().unwrap();
    let max_col = *col_nums.iter().max().unwrap();

    let expanded_rows = (0..max_row)
        .filter(|i| !row_nums.contains(i))
        .collect::<Vec<usize>>();
    let expanded_cols = (0..max_col)
        .filter(|i| !col_nums.contains(i))
        .collect::<Vec<usize>>();

    (expanded_rows, expanded_cols)
}

fn expanded_space(sky_map: &SkyMap, expansion_factor: usize) -> SkyMap {
    let (expanded_rows, expanded_cols) = get_expanded_rows_and_cols(sky_map);

    let mut expanded_map = sky_map.clone();

    expanded_rows.iter().rev().for_each(|row| {
        expanded_map.iter_mut().for_each(|coord| {
            if coord.row > *row {
                coord.row += expansion_factor - 1;
            }
        })
    });

    expanded_cols.iter().rev().for_each(|col| {
        expanded_map.iter_mut().for_each(|coord| {
            if coord.col > *col {
                coord.col += expansion_factor - 1;
            }
        })
    });

    expanded_map
}

fn get_pairs(map_len: usize) -> Vec<(usize, usize)> {
    let mut pairs = vec![];
    for i in 0..(map_len - 1) {
        for j in (i + 1)..map_len {
            pairs.push((i, j));
        }
    }

    pairs
}

fn get_dist_between_galaxies(sky_map: &SkyMap, from: usize, to: usize) -> u64 {
    let dy = ((sky_map[from].row as i32) - (sky_map[to].row as i32)).abs() as u64;
    let dx = ((sky_map[from].col as i32) - (sky_map[to].col as i32)).abs() as u64;
    dy + dx
}

pub fn get_sum_of_galaxy_dists(s: &str, expansion_factor: usize) -> u64 {
    let skymap = parse_input(s);
    let skymap = expanded_space(&skymap, expansion_factor);

    get_pairs(skymap.len())
        .iter()
        .map(|(from, to)| get_dist_between_galaxies(&skymap, *from, *to))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT1: &str = concat!(
        "...#......\n",
        ".......#..\n",
        "#.........\n",
        "..........\n",
        "......#...\n",
        ".#........\n",
        ".........#\n",
        "..........\n",
        ".......#..\n",
        "#...#.....\n",
    );

    fn sample_input1_skymap() -> SkyMap {
        vec![
            Coord::new(0, 3),
            Coord::new(1, 7),
            Coord::new(2, 0),
            Coord::new(4, 6),
            Coord::new(5, 1),
            Coord::new(6, 9),
            Coord::new(8, 7),
            Coord::new(9, 0),
            Coord::new(9, 4),
        ]
    }

    fn sample_input1_skymap_expanded() -> SkyMap {
        vec![
            Coord::new(0, 4),
            Coord::new(1, 9),
            Coord::new(2, 0),
            Coord::new(5, 8),
            Coord::new(6, 1),
            Coord::new(7, 12),
            Coord::new(10, 9),
            Coord::new(11, 0),
            Coord::new(11, 5),
        ]
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(SAMPLE_INPUT1), sample_input1_skymap());
    }

    #[test]
    fn test_get_expanded_rows_and_cols() {
        assert_eq!(
            get_expanded_rows_and_cols(&sample_input1_skymap()),
            (vec![3, 7], vec![2, 5, 8])
        );
    }

    #[test]
    fn test_expanded_space() {
        assert_eq!(
            expanded_space(&sample_input1_skymap(), 2),
            sample_input1_skymap_expanded()
        );
    }

    #[test]
    fn test_get_pairs() {
        assert_eq!(
            get_pairs(5),
            vec![
                (0, 1),
                (0, 2),
                (0, 3),
                (0, 4),
                (1, 2),
                (1, 3),
                (1, 4),
                (2, 3),
                (2, 4),
                (3, 4),
            ]
        );
    }

    #[test]
    fn test_get_dist_between_galaxies() {
        let skymap = sample_input1_skymap_expanded();
        assert_eq!(get_dist_between_galaxies(&skymap, 0, 6), 15);
        assert_eq!(get_dist_between_galaxies(&skymap, 6, 0), 15);
        assert_eq!(get_dist_between_galaxies(&skymap, 2, 5), 17);
        assert_eq!(get_dist_between_galaxies(&skymap, 5, 2), 17);
        assert_eq!(get_dist_between_galaxies(&skymap, 7, 8), 5);
        assert_eq!(get_dist_between_galaxies(&skymap, 8, 7), 5);
    }

    #[test]
    fn test_get_sum_of_galaxy_dists() {
        assert_eq!(get_sum_of_galaxy_dists(SAMPLE_INPUT1, 2), 374);
        assert_eq!(get_sum_of_galaxy_dists(SAMPLE_INPUT1, 10), 1030);
        assert_eq!(get_sum_of_galaxy_dists(SAMPLE_INPUT1, 100), 8410);
    }
}
