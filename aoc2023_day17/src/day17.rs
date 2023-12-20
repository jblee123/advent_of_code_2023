const MAX_STRAIGHT_STEPS: u32 = 3;

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

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
enum Dir {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct PathHead {
    coord: Coord,
    dir: Dir,
    straight_steps: u32,
    heat_disipation: u32,
}

impl PathHead {
    pub fn new(
        row: usize,
        col: usize,
        dir: Dir,
        straight_steps: u32,
        heat_disipation: u32,
    ) -> Self {
        Self {
            coord: Coord::new(row, col),
            dir,
            straight_steps,
            heat_disipation,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct StepRecord {
    min_disipations: Vec<[u32; 4]>,
}

impl StepRecord {
    pub fn new() -> Self {
        Self {
            min_disipations: vec![[u32::MAX; 4]; MAX_STRAIGHT_STEPS as usize],
        }
    }
}

type HeatMap = Vec<Vec<u32>>;
type MapStepRecord = Vec<Vec<StepRecord>>;

fn parse_input(s: &str) -> HeatMap {
    s.lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|c| (c - '0' as u8) as u32)
                .collect::<Vec<u32>>()
        })
        .collect::<HeatMap>()
}

fn gen_map_step_record(heat_map: &HeatMap) -> MapStepRecord {
    heat_map
        .iter()
        .map(|row| vec![StepRecord::new(); row.len()])
        .collect()
}

fn get_straight_step_count_for_dir(path_head: &PathHead, dir: Dir) -> u32 {
    if path_head.dir == dir {
        path_head.straight_steps + 1
    } else {
        1
    }
}

fn move_path_up(path_head: &PathHead) -> Option<PathHead> {
    let row = path_head.coord.row;
    let col = path_head.coord.col;
    let step_count = get_straight_step_count_for_dir(path_head, Dir::Up);
    if row > 0 {
        Some(PathHead::new(
            row - 1,
            col,
            Dir::Up,
            step_count,
            path_head.heat_disipation,
        ))
    } else {
        None
    }
}

fn move_path_down(path_head: &PathHead, max_rows: usize) -> Option<PathHead> {
    let row = path_head.coord.row;
    let col = path_head.coord.col;
    let step_count = get_straight_step_count_for_dir(path_head, Dir::Down);
    if row < (max_rows - 1) {
        Some(PathHead::new(
            row + 1,
            col,
            Dir::Down,
            step_count,
            path_head.heat_disipation,
        ))
    } else {
        None
    }
}

fn move_path_left(path_head: &PathHead) -> Option<PathHead> {
    let row = path_head.coord.row;
    let col = path_head.coord.col;
    let step_count = get_straight_step_count_for_dir(path_head, Dir::Left);
    if col > 0 {
        Some(PathHead::new(
            row,
            col - 1,
            Dir::Left,
            step_count,
            path_head.heat_disipation,
        ))
    } else {
        None
    }
}

fn move_path_right(path_head: &PathHead, max_cols: usize) -> Option<PathHead> {
    let row = path_head.coord.row;
    let col = path_head.coord.col;
    let step_count = get_straight_step_count_for_dir(path_head, Dir::Right);
    if col < (max_cols - 1) {
        Some(PathHead::new(
            row,
            col + 1,
            Dir::Right,
            step_count,
            path_head.heat_disipation,
        ))
    } else {
        None
    }
}

fn step_path(
    path_head: &PathHead,
    heat_map: &HeatMap,
) -> (Option<PathHead>, Option<PathHead>, Option<PathHead>) {
    let max_rows = heat_map.len();
    let max_cols = heat_map[0].len();

    let new_path_head1 = if path_head.straight_steps < MAX_STRAIGHT_STEPS {
        match path_head.dir {
            Dir::Up => move_path_up(path_head),
            Dir::Down => move_path_down(path_head, max_rows),
            Dir::Left => move_path_left(path_head),
            Dir::Right => move_path_right(path_head, max_cols),
        }
    } else {
        None
    };

    let new_path_head2 = match path_head.dir {
        Dir::Up => move_path_left(path_head),
        Dir::Down => move_path_right(path_head, max_cols),
        Dir::Left => move_path_down(path_head, max_rows),
        Dir::Right => move_path_up(path_head),
    };

    let new_path_head3 = match path_head.dir {
        Dir::Up => move_path_right(path_head, max_cols),
        Dir::Down => move_path_left(path_head),
        Dir::Left => move_path_up(path_head),
        Dir::Right => move_path_down(path_head, max_rows),
    };

    (new_path_head1, new_path_head2, new_path_head3)
}

fn apply_heat_map_to_path(path_head: &mut PathHead, heat_map: &HeatMap) {
    let row = path_head.coord.row;
    let col = path_head.coord.col;
    let heat = heat_map[row][col];
    path_head.heat_disipation += heat;
}

fn filter_paths(map_step_record: &MapStepRecord, path_heads: &[PathHead]) -> Vec<PathHead> {
    path_heads
        .iter()
        .filter(|path_head| {
            let step_record = &map_step_record[path_head.coord.row][path_head.coord.col];
            let step_idx = (path_head.straight_steps - 1) as usize;
            let dir_idx = path_head.dir as usize;
            if step_idx >= 3 {
                println!("wtf? step_idx is {step_idx}!");
            }
            let prev_disipation = step_record.min_disipations[step_idx][dir_idx];
            path_head.heat_disipation < prev_disipation
        })
        .map(|beam| beam.clone())
        .collect()
}

fn filter_finished_paths(
    heat_map: &HeatMap,
    path_heads: &[PathHead],
) -> Vec<PathHead> {
    path_heads
        .iter()
        .filter(|path_head| {
            let end_coord = Coord::new(heat_map.len() - 1, heat_map[0].len() - 1);
            path_head.coord != end_coord
        })
        .map(|path_head| path_head.clone())
        .collect()
}

fn update_map_step_records(map_step_record: &mut MapStepRecord, path_heads: &[PathHead]) {
    path_heads.iter().for_each(|path_head| {
        let step_record = &mut map_step_record[path_head.coord.row][path_head.coord.col];
        let mut step_idx = (path_head.straight_steps - 1) as usize;
        let dir_idx = path_head.dir as usize;
        while step_idx < step_record.min_disipations.len() {
            let prev_disipation = step_record.min_disipations[step_idx][dir_idx];
            let new_disipation = prev_disipation.min(path_head.heat_disipation);
            step_record.min_disipations[step_idx][dir_idx] = new_disipation;

            step_idx += 1;
        }
    });
}

fn filter_paths_by_combat(path_heads: &[PathHead]) -> Vec<PathHead> {
    let mut path_heads = path_heads.to_vec();
    path_heads.sort_by(|a, b| {
        if a.coord.row != b.coord.row {
            a.coord.row.partial_cmp(&b.coord.row).unwrap()
        } else if a.coord.col != b.coord.col {
            a.coord.col.partial_cmp(&b.coord.col).unwrap()
        } else if a.dir != b.dir {
            a.dir.partial_cmp(&a.dir).unwrap()
        } else if a.straight_steps != b.straight_steps {
            a.straight_steps.partial_cmp(&b.straight_steps).unwrap()
        } else {
            a.heat_disipation.partial_cmp(&b.heat_disipation).unwrap()
        }
    });

    let mut new_paths = vec![];
    new_paths.reserve(path_heads.len());

    (0..path_heads.len()).rev().for_each(|i| {
        if i == 0 {
            new_paths.push(path_heads[i]);
            return;
        }

        let mut prev = i - 1;
        let mut found_better = false;
        loop {
            if path_heads[prev].coord != path_heads[i].coord {
                break;
            }
            if path_heads[prev].dir != path_heads[i].dir {
                break;
            }
            if path_heads[prev].heat_disipation <= path_heads[i].heat_disipation {
                found_better = true;
                break;
            }
            if prev == 0 {
                break;
            }

            prev -= 1;
        }

        if !found_better {
            new_paths.push(path_heads[i]);
        }
    });

    new_paths
}

fn step(
    map_step_record: &mut MapStepRecord,
    path_heads: &[PathHead],
    heat_map: &HeatMap,
) -> Vec<PathHead> {
    let mut new_path_heads = vec![];

    path_heads.iter().for_each(|path_head| {
        let (head1, head2, head3) = step_path(path_head, heat_map);
        if let Some(head1) = head1 {
            new_path_heads.push(head1);
        }
        if let Some(head2) = head2 {
            new_path_heads.push(head2);
        }
        if let Some(head3) = head3 {
            new_path_heads.push(head3);
        }
    });

    let mut new_path_heads = filter_paths_by_combat(&new_path_heads);

    new_path_heads.iter_mut().for_each(|new_path_head| {
        apply_heat_map_to_path(new_path_head, heat_map);
    });

    let new_path_heads = filter_paths(map_step_record, &new_path_heads);

    update_map_step_records(map_step_record, &new_path_heads);

    let new_path_heads = filter_finished_paths(heat_map, &new_path_heads);

    new_path_heads
}

fn get_min_disipation(heat_map: &HeatMap) -> u32 {
    let mut map_step_record = gen_map_step_record(heat_map);
    let mut path_heads = vec![
        PathHead::new(0, 0, Dir::Right, 0, 0),
        PathHead::new(0, 0, Dir::Down, 0, 0),
    ];

    // update_map_step_records(&mut map_step_record, &path_heads);

    let mut steps = 0 as u64;
    while !path_heads.is_empty() {
        path_heads = step(&mut map_step_record, &path_heads, &heat_map);
        steps += 1;
        // if steps % 1000 == 0 {
        println!("{} steps: num heads: {}", steps, path_heads.len());
        // }
    }

    let end_record = map_step_record.last().unwrap().last().unwrap();
    end_record
        .min_disipations
        .iter()
        .map(|dir_recs| *(dir_recs.iter().min().unwrap()))
        .min()
        .unwrap()
}

pub fn get_min_disipation_from_input(s: &str) -> u32 {
    let heat_map = parse_input(s);
    get_min_disipation(&heat_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_1: &str = concat!(
        "2413432311323\n",
        "3215453535623\n",
        "3255245654254\n",
        "3446585845452\n",
        "4546657867536\n",
        "1438598798454\n",
        "4457876987766\n",
        "3637877979653\n",
        "4654967986887\n",
        "4564679986453\n",
        "1224686865563\n",
        "2546548887735\n",
        "4322674655533\n",
    );

    #[test]
    fn test_get_min_disipation_from_input() {
        println!();
        assert_eq!(get_min_disipation_from_input(SAMPLE_INPUT_1), 102);
    }

    // #[test]
    // fn test_get_max_num_energized_from_input() {
    //     println!();
    //     assert_eq!(get_max_num_energized_from_input(SAMPLE_INPUT_1), 51);
    // }
}
