use std::collections::HashMap;

fn parse_schematic_for_symbols(s: &str) -> Vec<Vec<u8>> {
    s.lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    if char::is_numeric(c) {
                        '.' as u8
                    } else {
                        c as u8
                    }
                })
                .collect()
        })
        .collect()
}

fn get_ids_and_adjacent_cells(
    s: &Vec<u8>,
    row_num: usize,
    max_rows: usize,
) -> Vec<(u32, Vec<(usize, usize)>)> {
    let mut ids = Vec::<(u32, Vec<(usize, usize)>)>::new();

    let mut start_idx = 0 as usize;
    let mut end_idx = 0 as usize;
    while (start_idx < s.len()) && (end_idx < s.len()) {
        if !char::is_numeric(s[start_idx] as char) {
            start_idx += 1;
            continue;
        }

        end_idx = start_idx + 1;
        while end_idx < s.len() && char::is_numeric(s[end_idx] as char) {
            end_idx += 1;
        }

        let id_str = std::str::from_utf8(&s[start_idx..end_idx]).unwrap();
        let id = u32::from_str_radix(&id_str, 10).unwrap();

        let mut adjacencies: Vec<(usize, usize)> = vec![];

        let left = if start_idx == 0 {
            start_idx
        } else {
            start_idx - 1
        };
        let right = if end_idx == s.len() {
            end_idx - 1
        } else {
            end_idx
        };

        // top
        if row_num != 0 {
            for col in left..=right {
                adjacencies.push((row_num - 1, col))
            }
        }
        // left
        if start_idx > 0 {
            adjacencies.push((row_num, left))
        }
        if end_idx < s.len() {
            adjacencies.push((row_num, right))
        }
        // bottom
        if row_num < (max_rows - 1) {
            for col in left..=right {
                adjacencies.push((row_num + 1, col))
            }
        }

        ids.push((id, adjacencies));

        start_idx = end_idx;
    }

    ids
}

pub fn get_sum_of_ids(s: &str) -> u32 {
    let lines = s
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();

    let symbol_locs = parse_schematic_for_symbols(s);

    let mut ids_and_coords = Vec::<(u32, Vec<(usize, usize)>)>::new();

    lines
        .iter()
        .enumerate()
        .map(|(idx, line)| get_ids_and_adjacent_cells(line, idx, lines.len()))
        .for_each(|id_map| {
            ids_and_coords.extend(id_map);
        });

    ids_and_coords
        .iter()
        .filter(|(_, coords)| {
            coords
                .iter()
                .filter(|(row, col)| symbol_locs[*row][*col] != ('.' as u8))
                .next()
                .is_some()
        })
        .map(|(id, _)| *id)
        .sum()
}

pub fn get_sum_of_gear_ratios(s: &str) -> u32 {
    let lines = s
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();

    let symbol_locs = parse_schematic_for_symbols(s);

    let mut ids_and_coords = Vec::<(u32, Vec<(usize, usize)>)>::new();

    lines
        .iter()
        .enumerate()
        .map(|(idx, line)| get_ids_and_adjacent_cells(line, idx, lines.len()))
        .for_each(|id_map| {
            ids_and_coords.extend(id_map);
        });

    let mut possible_gears = HashMap::<(usize, usize), Vec<u32>>::new();

    for (id, coords) in &ids_and_coords {
        for (row, col) in coords {
            if symbol_locs[*row][*col] == ('*' as u8) {
                let entry = possible_gears.entry((*row, *col)).or_default();
                entry.push(*id);
            }
        }
    }

    possible_gears
        .iter()
        .filter(|(_, ids)| ids.len() == 2)
        .map(|(_, ids)| ids[0] * ids[1])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = concat!(
        "467..114..\n",
        "...*......\n",
        "..35..633.\n",
        "......#...\n",
        "617*......\n",
        ".....+.58.\n",
        "..592.....\n",
        "......755.\n",
        "...$.*....\n",
        ".664.598..\n",
    );

    #[test]
    fn test_parse_schematic_for_symbols() {
        let output = vec![
            "..........".as_bytes(),
            "...*......".as_bytes(),
            "..........".as_bytes(),
            "......#...".as_bytes(),
            "...*......".as_bytes(),
            ".....+....".as_bytes(),
            "..........".as_bytes(),
            "..........".as_bytes(),
            "...$.*....".as_bytes(),
            "..........".as_bytes(),
        ];
        assert_eq!(parse_schematic_for_symbols(SAMPLE_INPUT), output);
    }

    #[test]
    fn test_get_ids_and_adjacent_cells() {
        let lines = SAMPLE_INPUT
            .lines()
            .map(|line| line.as_bytes().to_vec())
            .collect::<Vec<Vec<u8>>>();

        let mut output = Vec::<(u32, Vec<(usize, usize)>)>::new();
        output.push((467, vec![(0, 3), (1, 0), (1, 1), (1, 2), (1, 3)]));
        output.push((
            114,
            vec![(0, 4), (0, 8), (1, 4), (1, 5), (1, 6), (1, 7), (1, 8)],
        ));
        assert_eq!(
            get_ids_and_adjacent_cells(&lines[0], 0, lines.len()),
            output
        );

        let mut output = Vec::<(u32, Vec<(usize, usize)>)>::new();
        output.push((
            35,
            vec![
                (1, 1),
                (1, 2),
                (1, 3),
                (1, 4),
                (2, 1),
                (2, 4),
                (3, 1),
                (3, 2),
                (3, 3),
                (3, 4),
            ],
        ));
        output.push((
            633,
            vec![
                (1, 5),
                (1, 6),
                (1, 7),
                (1, 8),
                (1, 9),
                (2, 5),
                (2, 9),
                (3, 5),
                (3, 6),
                (3, 7),
                (3, 8),
                (3, 9),
            ],
        ));
        assert_eq!(
            get_ids_and_adjacent_cells(&lines[2], 2, lines.len()),
            output
        );

        let mut output = Vec::<(u32, Vec<(usize, usize)>)>::new();
        output.push((
            664,
            vec![(8, 0), (8, 1), (8, 2), (8, 3), (8, 4), (9, 0), (9, 4)],
        ));
        output.push((
            598,
            vec![(8, 4), (8, 5), (8, 6), (8, 7), (8, 8), (9, 4), (9, 8)],
        ));
        assert_eq!(
            get_ids_and_adjacent_cells(&lines[9], 9, lines.len()),
            output
        );
    }

    #[test]
    fn test_get_sum_of_ids() {
        assert_eq!(get_sum_of_ids(SAMPLE_INPUT), 4361);
    }

    #[test]
    fn test_get_sum_of_gear_ratios() {
        assert_eq!(get_sum_of_gear_ratios(SAMPLE_INPUT), 467835);
    }
}
