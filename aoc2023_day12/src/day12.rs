#[derive(Debug, PartialEq, Clone, PartialOrd)]
struct SpringRow {
    line_str: String,
    working_segs: Vec<u32>,
}

fn gen_all_slot_combos(num_slots: u32, num_filled: u32) -> Vec<Vec<bool>> {
    if num_slots <= num_filled {
        return vec![vec![true; num_slots as usize]];
    }
    if num_filled == 0 {
        return vec![vec![false; num_slots as usize]];
    }

    let results_for_first_slot_filled = gen_all_slot_combos(num_slots - 1, num_filled - 1);
    let results_for_first_slot_empty = gen_all_slot_combos(num_slots - 1, num_filled);

    let results_for_first_slot_filled = results_for_first_slot_filled
        .iter()
        .map(|val| {
            let mut combos = vec![true];
            combos.extend(val);
            combos
        })
        .collect::<Vec<Vec<bool>>>();
    let results_for_first_slot_empty = results_for_first_slot_empty
        .iter()
        .map(|val| {
            let mut combos = vec![false];
            combos.extend(val);
            combos
        })
        .collect::<Vec<Vec<bool>>>();

    let mut results = results_for_first_slot_filled;
    results.extend(results_for_first_slot_empty);

    results
}

fn parse_spring_row(line: &str) -> SpringRow {
    let mut parts = line.split(' ');
    let line_str = parts.next().unwrap().to_string();
    let working_segs = parts
        .next()
        .unwrap()
        .split(',')
        .map(|num_str| num_str.parse().unwrap())
        .collect::<Vec<u32>>();

    SpringRow {
        line_str: line_str,
        working_segs: working_segs,
    }
}

fn gen_all_possibilities(spring_row: &SpringRow) -> Vec<String> {
    let row_len = spring_row.line_str.len();
    let total_filled_slots = spring_row.working_segs.iter().sum::<u32>();
    let open_slots = row_len as u32 - total_filled_slots;
    let num_seg_slots = open_slots + 1;
    let num_segs = spring_row.working_segs.len() as u32;
    let seg_slot_combos = gen_all_slot_combos(num_seg_slots, num_segs);

    let seg_strs = spring_row
        .working_segs
        .iter()
        .map(|seg_len| vec!['#'; *seg_len as usize].iter().collect())
        .collect::<Vec<String>>();
    let empty_str = String::new();

    seg_slot_combos
        .iter()
        .map(|seg_slot_spec| {
            let mut seg_idx = 0 as usize;
            let seg_strs = seg_slot_spec
                .iter()
                .map(|is_filled| {
                    if *is_filled {
                        seg_idx += 1;
                        &seg_strs[seg_idx - 1][..]
                    } else {
                        &empty_str[..]
                    }
                })
                .collect::<Vec<&str>>();
            seg_strs.join(".")
        })
        .collect::<Vec<String>>()
}

fn matches_template(s: &str, template: &str) -> bool {
    if s.len() != template.len() {
        return false;
    }

    let s = s.as_bytes();
    let template = template.as_bytes();

    for i in 0..s.len() {
        if template[i] == '?' as u8 {
            continue;
        }
        if template[i] != s[i] {
            return false;
        }
    }

    true
}

fn get_num_good_configs(spring_row: &SpringRow) -> u32 {
    gen_all_possibilities(spring_row)
        .iter()
        .filter(|opt| matches_template(opt, &spring_row.line_str))
        .count() as u32
}

pub fn get_sum_of_num_good_configs(s: &str) -> u32 {
    s.lines()
        .map(|line| parse_spring_row(line))
        .map(|spring_row| get_num_good_configs(&spring_row))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT1: &str = concat!(
        "???.### 1,1,3\n",
        ".??..??...?##. 1,1,3\n",
        "?#?#?#?#?#?#?#? 1,3,1,6\n",
        "????.#...#... 4,1,1\n",
        "????.######..#####. 1,6,5\n",
        "?###???????? 3,2,1\n",
    );

    #[test]
    fn test_gen_all_slot_combos() {
        assert_eq!(
            gen_all_slot_combos(5, 3),
            vec![
                vec![true, true, true, false, false],
                vec![true, true, false, true, false],
                vec![true, true, false, false, true],
                vec![true, false, true, true, false],
                vec![true, false, true, false, true],
                vec![true, false, false, true, true],
                vec![false, true, true, true, false],
                vec![false, true, true, false, true],
                vec![false, true, false, true, true],
                vec![false, false, true, true, true],
            ]
        );
    }

    #[test]
    fn test_parse_spring_row() {
        assert_eq!(
            parse_spring_row("????.######..#####. 1,6,5"),
            SpringRow {
                line_str: "????.######..#####.".to_string(),
                working_segs: vec![1, 6, 5]
            }
        );
    }

    #[test]
    fn test_gen_all_possibilities() {
        let expected = vec![
            "#.######.#####..".to_string(),
            "#.######..#####.".to_string(),
            "#.######...#####".to_string(),
            "#..######.#####.".to_string(),
            "#..######..#####".to_string(),
            "#...######.#####".to_string(),
            ".#.######.#####.".to_string(),
            ".#.######..#####".to_string(),
            ".#..######.#####".to_string(),
            "..#.######.#####".to_string(),
        ];

        assert_eq!(
            gen_all_possibilities(&SpringRow {
                line_str: "................".to_string(),
                working_segs: vec![1, 6, 5]
            }),
            expected
        );
    }

    #[test]
    fn test_matches_template() {
        assert_eq!(
            matches_template("XXXX.######..#####.", "????.######..#####."),
            true
        );
        assert_eq!(
            matches_template("XXXX..#####..#####.", "????.######..#####."),
            false
        );
    }

    #[test]
    fn test_get_num_good_configs() {
        assert_eq!(
            get_num_good_configs(&SpringRow {
                line_str: "???.###".to_string(),
                working_segs: vec![1, 1, 3]
            }),
            1
        );
        assert_eq!(
            get_num_good_configs(&SpringRow {
                line_str: "?###????????".to_string(),
                working_segs: vec![3, 2, 1]
            }),
            10
        );
    }

    #[test]
    fn test_get_sum_of_num_good_configs() {
        assert_eq!(get_sum_of_num_good_configs(SAMPLE_INPUT1), 21);
    }
}
