#[derive(Debug, PartialEq, Clone, PartialOrd)]
struct Pattern {
    lines: Vec<String>,
    lines_rev: Vec<String>,
    lines_transposed: Vec<String>,
    lines_transposed_rev: Vec<String>,
}

impl Pattern {
    pub fn new(
        lines: Vec<String>,
        lines_rev: Vec<String>,
        lines_transposed: Vec<String>,
        lines_transposed_rev: Vec<String>,
    ) -> Self {
        Self {
            lines,
            lines_rev,
            lines_transposed,
            lines_transposed_rev,
        }
    }
}

fn parse_input(s: &str) -> Vec<Vec<String>> {
    let mut patterns = vec![];

    let mut pattern = vec![];
    for line in s.lines() {
        if line.is_empty() {
            patterns.push(pattern);
            pattern = vec![];
        } else {
            pattern.push(line.to_string());
        }
    }

    if !pattern.is_empty() {
        patterns.push(pattern);
    }

    patterns
}

fn build_pattern(orig_pattern: &[String]) -> Pattern {
    let lines = orig_pattern.to_vec();
    let lines_rev = lines
        .iter()
        .rev()
        .map(|s| s.clone())
        .collect::<Vec<String>>();

    let lines_transposed = (0..orig_pattern[0].len())
        .map(|i| {
            orig_pattern
                .iter()
                .map(|line| line.as_bytes()[i] as char)
                .collect::<String>()
        })
        .collect::<Vec<String>>();

    let lines_transposed_rev = lines_transposed
        .iter()
        .rev()
        .map(|s| s.clone())
        .collect::<Vec<String>>();

    Pattern::new(lines, lines_rev, lines_transposed, lines_transposed_rev)
}

fn build_patterns(orig_patterns: &[Vec<String>]) -> Vec<Pattern> {
    orig_patterns
        .iter()
        .map(|orig_pattern| build_pattern(orig_pattern))
        .collect()
}

fn find_horizontal_line_of_symmetry(strs: &[String]) -> Option<u32> {
    let mut top = 0;
    let mut bottom = strs.len() - 1;

    while (bottom > top) && (strs[top] != strs[bottom]) {
        bottom -= 1;
    }

    if bottom == top {
        return None;
    }

    let matched_bottom = bottom;

    while top < bottom {
        if strs[top] != strs[bottom] {
            return find_horizontal_line_of_symmetry(&strs[..matched_bottom]);
        }
        top += 1;
        bottom -= 1;
    }

    // The last pass of the while loop should have put top *past* bottom. If top
    // and bottom are equal here, it means the reflection line is *on* a line
    // and not between them.
    if top > bottom {
        Some((top - 1) as u32)
    } else {
        None
    }
}

fn get_mirror_val_for_pattern(pattern: &Pattern) -> u32 {
    if let Some(val) = find_horizontal_line_of_symmetry(&pattern.lines) {
        (val + 1) * 100
    } else if let Some(val) = find_horizontal_line_of_symmetry(&pattern.lines_rev) {
        ((pattern.lines_rev.len() as u32) - (val + 1)) * 100
    } else if let Some(val) = find_horizontal_line_of_symmetry(&pattern.lines_transposed) {
        val + 1
    } else if let Some(val) = find_horizontal_line_of_symmetry(&pattern.lines_transposed_rev) {
        (pattern.lines_transposed_rev.len() as u32) - (val + 1)
    } else {
        0
    }
}

fn get_mirror_val_for_pattern_ignoring(pattern: &Pattern, to_ignore: u32) -> u32 {
    if let Some(val) = find_horizontal_line_of_symmetry(&pattern.lines) {
        let val = (val + 1) * 100;
        if val != to_ignore {
            return val;
        }
    }
    if let Some(val) = find_horizontal_line_of_symmetry(&pattern.lines_rev) {
        let val = ((pattern.lines_rev.len() as u32) - (val + 1)) * 100;
        if val != to_ignore {
            return val;
        }
    }
    if let Some(val) = find_horizontal_line_of_symmetry(&pattern.lines_transposed) {
        let val = val + 1;
        if val != to_ignore {
            return val;
        }
    }
    if let Some(val) = find_horizontal_line_of_symmetry(&pattern.lines_transposed_rev) {
        let val = (pattern.lines_transposed_rev.len() as u32) - (val + 1);
        if val != to_ignore {
            return val;
        }
    }

    0
}

pub fn get_summary_val(s: &str) -> u32 {
    build_patterns(&parse_input(s))
        .iter()
        .map(|pattern| get_mirror_val_for_pattern(&pattern))
        .sum()
}

fn reverse_coord(lines: &[String], row: usize, col: usize) -> Vec<String> {
    lines
        .iter()
        .enumerate()
        .map(|(row_idx, line)| {
            let mut bytes = Vec::from(line.as_bytes());
            if row == row_idx {
                bytes[col] = if bytes[col] == '.' as u8 {
                    '#' as u8
                } else {
                    '.' as u8
                };
            }
            String::from_utf8(bytes).unwrap()
        })
        .collect()
}

pub fn get_smudged_summary_val(s: &str) -> u32 {
    let parsed_inputs = parse_input(s);

    let mut sum = 0 as u32;

    for (input_idx, parsed_input) in parsed_inputs.iter().enumerate() {
        println!("processing input {} (idx {})", input_idx + 1, input_idx);
        let orig_pattern = build_pattern(&parsed_input);
        let orig_value = get_mirror_val_for_pattern(&orig_pattern);
        'outer: for row in 0..parsed_input.len() {
            for col in 0..parsed_input[0].len() {
                let smudged_input = reverse_coord(&parsed_input, row, col);
                let smudged_pattern = build_pattern(&smudged_input);
                let value = get_mirror_val_for_pattern_ignoring(&smudged_pattern, orig_value);
                if value != 0 {
                    sum += value;
                    break 'outer;
                }
            }
        }
    }
    
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_1: &str = concat!(
        "#.##..##.\n",
        "..#.##.#.\n",
        "##......#\n",
        "##......#\n",
        "..#.##.#.\n",
        "..##..##.\n",
        "#.#.##.#.\n",
        "\n",
        "#...##..#\n",
        "#....#..#\n",
        "..##..###\n",
        "#####.##.\n",
        "#####.##.\n",
        "..##..###\n",
        "#....#..#\n",
    );

    const SAMPLE_INPUT_1A: &str = concat!(
        "#.##..##.\n",
        "..#.##.#.\n",
        "##......#\n",
        "##......#\n",
        "..#.##.#.\n",
        "..##..##.\n",
        "#.#.##.#.\n",
    );

    const SAMPLE_INPUT_1B: &str = concat!(
        "#...##..#\n",
        "#....#..#\n",
        "..##..###\n",
        "#####.##.\n",
        "#####.##.\n",
        "..##..###\n",
        "#....#..#\n",
    );

    fn get_sample_pattern_01a() -> Pattern {
        Pattern::new(
            vec![
                "#.##..##.".to_string(),
                "..#.##.#.".to_string(),
                "##......#".to_string(),
                "##......#".to_string(),
                "..#.##.#.".to_string(),
                "..##..##.".to_string(),
                "#.#.##.#.".to_string(),
            ],
            vec![
                "#.#.##.#.".to_string(),
                "..##..##.".to_string(),
                "..#.##.#.".to_string(),
                "##......#".to_string(),
                "##......#".to_string(),
                "..#.##.#.".to_string(),
                "#.##..##.".to_string(),
            ],
            vec![
                "#.##..#".to_string(),
                "..##...".to_string(),
                "##..###".to_string(),
                "#....#.".to_string(),
                ".#..#.#".to_string(),
                ".#..#.#".to_string(),
                "#....#.".to_string(),
                "##..###".to_string(),
                "..##...".to_string(),
            ],
            vec![
                "..##...".to_string(),
                "##..###".to_string(),
                "#....#.".to_string(),
                ".#..#.#".to_string(),
                ".#..#.#".to_string(),
                "#....#.".to_string(),
                "##..###".to_string(),
                "..##...".to_string(),
                "#.##..#".to_string(),
            ],
        )
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(SAMPLE_INPUT_1),
            vec![
                vec![
                    "#.##..##.".to_string(),
                    "..#.##.#.".to_string(),
                    "##......#".to_string(),
                    "##......#".to_string(),
                    "..#.##.#.".to_string(),
                    "..##..##.".to_string(),
                    "#.#.##.#.".to_string(),
                ],
                vec![
                    "#...##..#".to_string(),
                    "#....#..#".to_string(),
                    "..##..###".to_string(),
                    "#####.##.".to_string(),
                    "#####.##.".to_string(),
                    "..##..###".to_string(),
                    "#....#..#".to_string(),
                ]
            ]
        );
    }

    #[test]
    fn test_build_pattern() {
        assert_eq!(
            build_pattern(&vec![
                "#.##..##.".to_string(),
                "..#.##.#.".to_string(),
                "##......#".to_string(),
                "##......#".to_string(),
                "..#.##.#.".to_string(),
                "..##..##.".to_string(),
                "#.#.##.#.".to_string(),
            ]),
            get_sample_pattern_01a()
        );
    }

    #[test]
    fn test_find_horizontal_line_of_symmetry() {
        assert_eq!(
            find_horizontal_line_of_symmetry(&vec![
                "#.##..##.".to_string(),
                "..#.##.#.".to_string(),
                "##......#".to_string(),
                "##......#".to_string(),
                "..#.##.#.".to_string(),
                "..##..##.".to_string(),
                "#.#.##.#.".to_string(),
            ],),
            None
        );
        assert_eq!(
            find_horizontal_line_of_symmetry(&vec![
                "#.##..#".to_string(),
                "..##...".to_string(),
                "##..###".to_string(),
                "#....#.".to_string(),
                ".#..#.#".to_string(),
                ".#..#.#".to_string(),
                "#....#.".to_string(),
                "##..###".to_string(),
                "..##...".to_string(),
            ],),
            None
        );
        assert_eq!(
            find_horizontal_line_of_symmetry(&vec![
                "..##...".to_string(),
                "##..###".to_string(),
                "#....#.".to_string(),
                ".#..#.#".to_string(),
                ".#..#.#".to_string(),
                "#....#.".to_string(),
                "##..###".to_string(),
                "..##...".to_string(),
                "#.##..#".to_string(),
            ],),
            Some(3)
        );
    }

    #[test]
    fn test_get_mirror_val_for_pattern() {
        let pattern_strs1 = parse_input(SAMPLE_INPUT_1A);
        let pattern1 = build_pattern(&pattern_strs1[0]);
        assert_eq!(get_mirror_val_for_pattern(&pattern1), 5);

        let pattern_strs2 = parse_input(SAMPLE_INPUT_1A);
        let new_pattern_strs2 = pattern_strs2[0]
            .iter()
            .map(|s| s.chars().rev().collect::<String>())
            .collect::<Vec<String>>();
        let pattern2 = build_pattern(&new_pattern_strs2);
        assert_eq!(get_mirror_val_for_pattern(&pattern2), 4);

        let pattern_strs3 = parse_input(SAMPLE_INPUT_1B);
        let pattern3 = build_pattern(&pattern_strs3[0]);
        assert_eq!(get_mirror_val_for_pattern(&pattern3), 400);

        let mut pattern_strs4 = parse_input(SAMPLE_INPUT_1B);
        pattern_strs4[0].reverse();
        let pattern4 = build_pattern(&pattern_strs4[0]);
        assert_eq!(get_mirror_val_for_pattern(&pattern4), 300);
    }

    #[test]
    fn test_get_summary_val() {
        assert_eq!(get_summary_val(SAMPLE_INPUT_1), 405);

        let cwd = std::env::current_dir().unwrap();
        println!("CWD: {:?}", cwd);

        let mut infile = "inputs/day13.txt";
        if cwd.ends_with("aoc2023_day13") {
            infile = "../inputs/day13.txt";
        }

        let input = aoc2023_utils::get_input(infile);
        assert_eq!(get_summary_val(&input), 34993);
    }

    #[test]
    fn test_reverse_coord() {
        assert_eq!(
            reverse_coord(
                &vec![
                    "#.##..##.".to_string(),
                    "..#.##.#.".to_string(),
                    "##......#".to_string(),
                    "##......#".to_string(),
                    "..#.##.#.".to_string(),
                    "..##..##.".to_string(),
                    "#.#.##.#.".to_string(),
                ],
                1,
                2
            ),
            vec![
                "#.##..##.".to_string(),
                "....##.#.".to_string(),
                "##......#".to_string(),
                "##......#".to_string(),
                "..#.##.#.".to_string(),
                "..##..##.".to_string(),
                "#.#.##.#.".to_string(),
            ]
        );
        assert_eq!(
            reverse_coord(
                &vec![
                    "#.##..##.".to_string(),
                    "..#.##.#.".to_string(),
                    "##......#".to_string(),
                    "##......#".to_string(),
                    "..#.##.#.".to_string(),
                    "..##..##.".to_string(),
                    "#.#.##.#.".to_string(),
                ],
                1,
                3
            ),
            vec![
                "#.##..##.".to_string(),
                "..####.#.".to_string(),
                "##......#".to_string(),
                "##......#".to_string(),
                "..#.##.#.".to_string(),
                "..##..##.".to_string(),
                "#.#.##.#.".to_string(),
            ]
        );
    }

    #[test]
    fn test_get_smudged_summary_val() {
        assert_eq!(get_smudged_summary_val(SAMPLE_INPUT_1), 400);

        let cwd = std::env::current_dir().unwrap();
        println!("CWD: {:?}", cwd);

        let mut infile = "inputs/day13.txt";
        if cwd.ends_with("aoc2023_day13") {
            infile = "../inputs/day13.txt";
        }

        let input = aoc2023_utils::get_input(infile);
        assert_eq!(get_smudged_summary_val(&input), 29341);
    }
}
