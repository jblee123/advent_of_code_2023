use chrono::Utc;
use std::{collections::HashMap, time::Instant};

#[derive(Debug, PartialEq, Clone, PartialOrd)]
struct SpringRow {
    line_str: String,
    working_segs: Vec<u64>,
}

fn n_choose_k(n: u64, k: u64) -> u64 {
    if n < k {
        println!("about to die with n={n}, k={k}!");
    }
    let mut n = n;
    let mut k = k;
    if k > (n - k) {
        k = n - k;
    }

    let mut c = 1 as u64;
    let mut i = 1 as u64;
    while i <= k {
        // panic on potential overflow
        if (c / i) > (u64::MAX / n) {
            panic!("n_choose_k OVERFLOW!!!");
        }

        c = c / i * n + c % i * n / i; // split c * n / i into (c / i * i + c % i) * n / i

        i += 1;
        n -= 1;
    }

    c
}

fn gen_all_slot_combos(num_slots: u64, num_filled: u64) -> Vec<Vec<bool>> {
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
        .collect::<Vec<u64>>();

    SpringRow {
        line_str: line_str,
        working_segs: working_segs,
    }
}

fn gen_all_possibilities(spring_row: &SpringRow) -> Vec<String> {
    let row_len = spring_row.line_str.len();
    let total_filled_slots = spring_row.working_segs.iter().sum::<u64>();
    let open_slots = row_len as u64 - total_filled_slots;
    let num_seg_slots = open_slots + 1;
    let num_segs = spring_row.working_segs.len() as u64;
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

fn get_num_good_configs(spring_row: &SpringRow) -> u64 {
    gen_all_possibilities(spring_row)
        .iter()
        .filter(|opt| matches_template(opt, &spring_row.line_str))
        .count() as u64
}

fn do_get_num_good_configs2(spring_row: &SpringRow, history: &mut HashMap<String, u64>) -> u64 {
    let history_hash = format!(
        "{} {}",
        spring_row.line_str,
        spring_row
            .working_segs
            .iter()
            .map(ToString::to_string)
            .collect::<String>()
    );

    if let Some(prev_val) = history.get(&history_hash) {
        return *prev_val;
    }

    if spring_row.working_segs.is_empty() {
        if spring_row.line_str.contains('#') {
            return 0;
        } else {
            return 1;
        }
    }

    let template = spring_row.line_str.trim_start_matches('.');
    if template.is_empty() {
        return 0;
    }

    let mut str_to_match = vec!['#'; spring_row.working_segs[0] as usize]
        .iter()
        .collect::<String>();
    if (spring_row.working_segs[0] as usize) < template.len() {
        str_to_match.push('.');
    }

    let template_len_to_check = str_to_match.len().min(template.len());
    let can_match_front = matches_template(&str_to_match, &template[..template_len_to_check]);
    let must_match_front = template.as_bytes()[0] == '#' as u8;

    let count_from_matched_front = if can_match_front {
        do_get_num_good_configs2(
            &SpringRow {
                line_str: template[str_to_match.len()..].to_string(),
                working_segs: spring_row.working_segs[1..].into(),
            },
            history,
        )
    } else {
        0
    };

    let count_from_unmatched_front = if must_match_front {
        0
    } else {
        do_get_num_good_configs2(
            &SpringRow {
                line_str: template[1..].to_string(),
                working_segs: spring_row.working_segs.clone(),
            },
            history,
        )
    };

    let result = count_from_matched_front + count_from_unmatched_front;
    history.insert(history_hash, result);
    result
}

fn get_num_good_configs2(spring_row: &SpringRow) -> u64 {
    let row_len = spring_row.line_str.len();
    let total_filled_slots = spring_row.working_segs.iter().sum::<u64>();
    let open_slots = row_len as u64 - total_filled_slots;
    let num_seg_slots = open_slots + 1;
    let num_segs = spring_row.working_segs.len() as u64;

    let num_possibilities = n_choose_k(num_seg_slots, num_segs);
    // println!("{num_seg_slots} choose {num_segs} = {num_possibilities} possibilities");

    // Special case for all '?' since there's no other way to cut down on the
    // search space.
    if spring_row.line_str.chars().all(|c| c == '?') {
        return num_possibilities;
    }

    let mut history = HashMap::<String, u64>::new();

    do_get_num_good_configs2(spring_row, &mut history)
}

pub fn get_sum_of_num_good_configs(s: &str) -> u64 {
    s.lines()
        .map(|line| parse_spring_row(line))
        .map(|spring_row| get_num_good_configs(&spring_row))
        .sum()
}

fn unfold_line(s: &str) -> String {
    let mut parts = s.split(' ');
    let part1 = parts.next().unwrap();
    let part2 = parts.next().unwrap();

    let part1 = vec![part1; 5].join("?");
    let part2 = vec![part2; 5].join(",");

    format!("{part1} {part2}")
}

pub fn get_sum_of_num_good_configs_unfolded(s: &str) -> u64 {
    let func_start = Instant::now();
    println!("starting at {:?}", Utc::now());
    let mut line_num = 1;
    let result = s
        .lines()
        .map(|line| unfold_line(&line))
        .map(|line| parse_spring_row(&line))
        .map(|spring_row| {
            let line_start = Instant::now();
            let result = get_num_good_configs2(&spring_row);
            println!(
                "finished line {line_num} at {:?} in {} sec. result = {result}",
                Utc::now(),
                line_start.elapsed().as_secs()
            );
            line_num += 1;
            result
        })
        .sum();
    println!(
        "ended at {:?} in {} sec",
        Utc::now(),
        func_start.elapsed().as_secs()
    );
    result
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

    #[ignore]
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

    #[ignore]
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

    #[ignore]
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

    #[ignore]
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

    #[ignore]
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
        assert_eq!(
            get_num_good_configs(&SpringRow {
                line_str: "???.###????.###????.###????.###????.###".to_string(),
                working_segs: vec![1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3]
            }),
            1
        );
        // assert_eq!(
        //     get_num_good_configs(&SpringRow {
        //         line_str: "?###??????????###??????????###??????????###??????????###????????"
        //             .to_string(),
        //         working_segs: vec![3, 2, 1, 3, 2, 1, 3, 2, 1, 3, 2, 1, 3, 2, 1]
        //     }),
        //     506250
        // );
    }

    #[test]
    fn test_get_num_good_configs2() {
        assert_eq!(
            get_num_good_configs2(&SpringRow {
                line_str: "???.###".to_string(),
                working_segs: vec![1, 1, 3]
            }),
            1
        );
        assert_eq!(
            get_num_good_configs2(&SpringRow {
                line_str: "???.###????.###????.###????.###????.###".to_string(),
                working_segs: vec![1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3]
            }),
            1
        );

        assert_eq!(
            get_num_good_configs2(&SpringRow {
                line_str: ".??..??...?##.".to_string(),
                working_segs: vec![1, 1, 3]
            }),
            4
        );
        assert_eq!(
            get_num_good_configs2(&SpringRow {
                line_str:
                    ".??..??...?##.?.??..??...?##.?.??..??...?##.?.??..??...?##.?.??..??...?##."
                        .to_string(),
                working_segs: vec![1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3]
            }),
            16384
        );

        assert_eq!(
            get_num_good_configs2(&SpringRow {
                line_str: "?#?#?#?#?#?#?#?".to_string(),
                working_segs: vec![1, 3, 1, 6]
            }),
            1
        );
        assert_eq!(
            get_num_good_configs2(&SpringRow {
                line_str: "?#?#?#?#?#?#?#???#?#?#?#?#?#?#???#?#?#?#?#?#?#???#?#?#?#?#?#?#???#?#?#?#?#?#?#?".to_string(),
                working_segs: vec![1, 3, 1, 6, 1, 3, 1, 6, 1, 3, 1, 6, 1, 3, 1, 6, 1, 3, 1, 6]
            }),
            1
        );

        assert_eq!(
            get_num_good_configs2(&SpringRow {
                line_str: "????.#...#...".to_string(),
                working_segs: vec![4, 1, 1]
            }),
            1
        );
        assert_eq!(
            get_num_good_configs2(&SpringRow {
                line_str: "????.#...#...?????.#...#...?????.#...#...?????.#...#...?????.#...#..."
                    .to_string(),
                working_segs: vec![4, 1, 1, 4, 1, 1, 4, 1, 1, 4, 1, 1, 4, 1, 1]
            }),
            16
        );

        assert_eq!(
            get_num_good_configs2(&SpringRow {
                line_str: "????.######..#####.".to_string(),
                working_segs: vec![1, 6, 5]
            }),
            4
        );
        assert_eq!(
            get_num_good_configs2(&SpringRow {
                line_str: "????.######..#####.?????.######..#####.?????.######..#####.?????.######..#####.?????.######..#####."
                    .to_string(),
                working_segs: vec![1, 6, 5, 1, 6, 5, 1, 6, 5, 1, 6, 5, 1, 6, 5]
            }),
            2500
        );

        assert_eq!(
            get_num_good_configs2(&SpringRow {
                line_str: "?###????????".to_string(),
                working_segs: vec![3, 2, 1]
            }),
            10
        );
        assert_eq!(
            get_num_good_configs2(&SpringRow {
                line_str: "?###??????????###??????????###??????????###??????????###????????"
                    .to_string(),
                working_segs: vec![3, 2, 1, 3, 2, 1, 3, 2, 1, 3, 2, 1, 3, 2, 1]
            }),
            506250
        );

        assert_eq!(
            get_num_good_configs2(&SpringRow {
                line_str: "??????????????????????????????????????????????????????".to_string(),
                working_segs: vec![1, 4, 1, 1, 4, 1, 1, 4, 1, 1, 4, 1, 1, 4, 1]
            }),
            3268760
        );

        assert_eq!(
            get_num_good_configs2(&SpringRow {
                line_str: "?#???.#??#?????.???#???.#??#?????.???#???.#??#?????.???#???.#??#?????.???#???.#??#?????.?".to_string(),
                working_segs: vec![3, 1, 3, 1, 1, 3, 1, 3, 1, 1, 3, 1, 3, 1, 1, 3, 1, 3, 1, 1, 3, 1, 3, 1, 1]
            }),
            1259712
        );
    }

    #[ignore]
    #[test]
    fn test_get_sum_of_num_good_configs() {
        assert_eq!(get_sum_of_num_good_configs(SAMPLE_INPUT1), 21);
    }

    #[ignore]
    #[test]
    fn test_unfold_line() {
        assert_eq!(unfold_line(".# 1"), ".#?.#?.#?.#?.# 1,1,1,1,1");
    }

    #[ignore]
    #[test]
    fn test_get_sum_of_num_good_configs_unfolded() {
        assert_eq!(get_sum_of_num_good_configs_unfolded(SAMPLE_INPUT1), 525152);
    }

    #[ignore]
    #[test]
    fn test_asdf() {
        let mut i = 0 as u64;
        while i < 40000000000 {
            if i % 1000000000 == 0 {
                println!("i: {}", i);
            }
            i += 1;
        }
        println!("done");
        // assert!(false);
    }
}
