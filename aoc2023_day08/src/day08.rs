use std::collections::HashMap;

type Tree = HashMap<String, (String, String)>;

fn parse_input(s: &str) -> (String, Tree) {
    let mut lines = s.lines();
    let directions = lines.next().unwrap();
    lines.next();

    let mut tree = Tree::default();

    while let Some(line) = lines.next() {
        let node_name = &line[0..3];
        let l_node = &line[7..10];
        let r_node = &line[12..15];
        tree.insert(
            node_name.to_string(),
            (l_node.to_string(), r_node.to_string()),
        );
    }

    (directions.to_string(), tree)
}

pub fn get_traversal_steps(s: &str) -> u32 {
    let (directions, tree) = parse_input(s);

    let directions = directions.as_bytes();

    let mut steps = 0 as usize;
    let mut current = "AAA";

    while current != "ZZZ" {
        let idx = steps % directions.len();
        if directions[idx] == ('L' as u8) {
            current = &tree[current].0;
        } else {
            current = &tree[current].1;
        }

        steps += 1;
    }

    steps as u32
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else if a < b {
        gcd(b, a)
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn lcm_nums(nums: &Vec<usize>) -> usize {
    let mut result = nums[0];

    let mut idx = 1 as usize;
    while idx < nums.len() {
        result = lcm(result, nums[idx]);
        idx += 1;
    }

    println!("result: {result}");

    result
}

pub fn get_ghost_traversal_steps(s: &str) -> u64 {
    let (directions, tree) = parse_input(s);

    let directions = directions.as_bytes();

    let start_nodes = tree
        .keys()
        .filter(|key| key.chars().last().unwrap() == 'A')
        .map(|key| &key[..])
        .collect::<Vec<&str>>();

    let cycle_lens = start_nodes
        .iter()
        .map(|node| {
            let mut current = *node;
            let mut steps = 0 as usize;
            while current.chars().last().unwrap() != 'Z' {
                let idx = steps % directions.len();
                let go_left = directions[idx] == ('L' as u8);
                current = if go_left {
                    &tree[current].0[..]
                } else {
                    &tree[current].1[..]
                };

                steps += 1;
            }

            steps
        })
        .collect::<Vec<usize>>();

    println!("cycle_lens: {:?}", cycle_lens);

    lcm_nums(&cycle_lens) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = concat!(
        "LLR\n",
        "\n",
        "AAA = (BBB, BBB)\n",
        "BBB = (AAA, ZZZ)\n",
        "ZZZ = (ZZZ, ZZZ)\n",
    );

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(SAMPLE_INPUT),
            (
                "LLR".to_string(),
                HashMap::from([
                    ("AAA".to_string(), ("BBB".to_string(), "BBB".to_string())),
                    ("BBB".to_string(), ("AAA".to_string(), "ZZZ".to_string())),
                    ("ZZZ".to_string(), ("ZZZ".to_string(), "ZZZ".to_string())),
                ])
            )
        );
    }

    #[test]
    fn test_get_traversal_steps() {
        assert_eq!(get_traversal_steps(SAMPLE_INPUT), 6);
    }

    #[test]
    fn test_get_ghost_traversal_steps() {
        assert_eq!(get_ghost_traversal_steps(SAMPLE_INPUT), 6);
    }
}
