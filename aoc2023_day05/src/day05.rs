use std::str::FromStr;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Category {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseCategoryError;

impl FromStr for Category {
    type Err = ParseCategoryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "seed" => Ok(Category::Seed),
            "soil" => Ok(Category::Soil),
            "fertilizer" => Ok(Category::Fertilizer),
            "water" => Ok(Category::Water),
            "light" => Ok(Category::Light),
            "temperature" => Ok(Category::Temperature),
            "humidity" => Ok(Category::Humidity),
            "location" => Ok(Category::Location),
            _ => Err(ParseCategoryError),
        }
    }
}

#[derive(Debug, PartialEq)]
struct MapRange {
    from: u32,
    to: u32,
    len: u32,
}

impl MapRange {
    pub fn new(from: u32, to: u32, len: u32) -> MapRange {
        Self { from, to, len }
    }

    pub fn reverse(&self) -> MapRange {
        Self::new(self.to, self.from, self.len)
    }
}

#[derive(Debug, PartialEq)]
struct Map {
    cat_from: Category,
    cat_to: Category,
    ranges: Vec<MapRange>,
}

impl Map {
    pub fn reverse(&self) -> Map {
        let mut new_ranges = self
            .ranges
            .iter()
            .map(|range| range.reverse())
            .collect::<Vec<MapRange>>();
        new_ranges.sort_by(|a, b| a.from.partial_cmp(&b.from).unwrap());
        Self {
            cat_from: self.cat_to,
            cat_to: self.cat_from,
            ranges: new_ranges,
        }
    }
}

struct SeedRange {
    start: u32,
    len: u32,
}

fn parse_category_map(label_line: &str, range_lines: Vec<&str>) -> Map {
    let mut label_parts = label_line.split(' ').next().unwrap().split("-to-");
    let cat_to = Category::from_str(label_parts.next().unwrap()).unwrap();
    let cat_from = Category::from_str(label_parts.next().unwrap()).unwrap();

    let mut map_ranges = range_lines
        .iter()
        .map(|line| {
            let mut parts = line.split(' ');
            let to = parts.next().unwrap().parse().unwrap();
            let from = parts.next().unwrap().parse().unwrap();
            let len = parts.next().unwrap().parse().unwrap();
            MapRange::new(from, to, len)
        })
        .collect::<Vec<MapRange>>();

    map_ranges.sort_by(|a, b| a.from.partial_cmp(&b.from).unwrap());

    Map {
        cat_from: cat_from,
        cat_to: cat_to,
        ranges: map_ranges,
    }
}

fn parse_input(s: &str) -> (Vec<u32>, Vec<Map>) {
    let mut category_maps: Vec<Map> = vec![];

    let mut lines = s.lines();

    let seed_line = lines.next().unwrap();
    let seeds = seed_line
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .map(|num_str| num_str.parse().unwrap())
        .collect::<Vec<u32>>();

    lines.next();

    loop {
        let label_line = lines.next();
        if label_line.is_none() {
            break;
        }

        let mut range_lines: Vec<&str> = vec![];
        loop {
            let range_line = lines.next().unwrap_or_default();
            if range_line.is_empty() {
                break;
            }
            range_lines.push(range_line);
        }

        category_maps.push(parse_category_map(label_line.unwrap(), range_lines));
    }

    (seeds, category_maps)
}

fn get_mapped_val(map: &Map, key: u32) -> u32 {
    for range in &map.ranges {
        if key < range.from {
            return key;
        }

        let diff = key - range.from;
        if diff < range.len {
            return range.to + diff;
        }
    }

    key
}

fn get_loc_for_seed(seed: u32, cat_maps: &Vec<Map>) -> u32 {
    let mut key = seed;
    for map in cat_maps {
        key = get_mapped_val(&map, key);
    }
    key
}

fn is_seed_in_range(seed: u32, ranges: &Vec<SeedRange>) -> bool {
    for range in ranges {
        if seed < range.start {
            return false;
        }

        let diff = seed - range.start;
        if diff < range.len {
            return true;
        }
    }

    false
}

pub fn get_lowest_loc_for_seed(s: &str) -> u32 {
    let (seeds, cat_maps) = parse_input(s);

    seeds
        .iter()
        .map(|seed| get_loc_for_seed(*seed, &cat_maps))
        .min()
        .unwrap()
}

pub fn get_lowest_loc_for_seed_ranges(s: &str) -> u32 {
    let (seeds, cat_maps) = parse_input(s);

    let mut seed_ranges: Vec<SeedRange> = vec![];
    let mut seeds_iter = seeds.iter();
    loop {
        let start = *seeds_iter.next().unwrap_or(&u32::MAX);
        let len = *seeds_iter.next().unwrap_or(&u32::MAX);
        if start == u32::MAX || len == u32::MAX {
            break;
        }
        seed_ranges.push(SeedRange { start, len });
    }

    seed_ranges.sort_by(|a, b| a.start.partial_cmp(&b.start).unwrap());

    let mut cat_maps = cat_maps
        .iter()
        .map(|cat_map| cat_map.reverse())
        .collect::<Vec<Map>>();
    cat_maps.reverse();

    let mut loc = 0 as u32;
    loop {
        // we reversed everything, so this should actually be getting the seed
        // from the location
        let seed = get_loc_for_seed(loc, &cat_maps);
        if loc % 1000000 == 0 {
            println!("From loc {loc}, got seed {seed}");
        }
        if is_seed_in_range(seed, &seed_ranges) {
            return loc;
        }

        loc += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = concat!(
        "seeds: 79 14 55 13\n",
        "\n",
        "seed-to-soil map:\n",
        "50 98 2\n",
        "52 50 48\n",
        "\n",
        "soil-to-fertilizer map:\n",
        "0 15 37\n",
        "37 52 2\n",
        "39 0 15\n",
        "\n",
        "fertilizer-to-water map:\n",
        "49 53 8\n",
        "0 11 42\n",
        "42 0 7\n",
        "57 7 4\n",
        "\n",
        "water-to-light map:\n",
        "88 18 7\n",
        "18 25 70\n",
        "\n",
        "light-to-temperature map:\n",
        "45 77 23\n",
        "81 45 19\n",
        "68 64 13\n",
        "\n",
        "temperature-to-humidity map:\n",
        "0 69 1\n",
        "1 0 69\n",
        "\n",
        "humidity-to-location map:\n",
        "60 56 37\n",
        "56 93 4\n",
    );

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(SAMPLE_INPUT),
            (
                vec![79, 14, 55, 13],
                vec![
                    Map {
                        cat_from: Category::Soil,
                        cat_to: Category::Seed,
                        ranges: vec![MapRange::new(50, 52, 48), MapRange::new(98, 50, 2),],
                    },
                    Map {
                        cat_from: Category::Fertilizer,
                        cat_to: Category::Soil,
                        ranges: vec![
                            MapRange::new(0, 39, 15),
                            MapRange::new(15, 0, 37),
                            MapRange::new(52, 37, 2),
                        ],
                    },
                    Map {
                        cat_from: Category::Water,
                        cat_to: Category::Fertilizer,
                        ranges: vec![
                            MapRange::new(0, 42, 7),
                            MapRange::new(7, 57, 4),
                            MapRange::new(11, 0, 42),
                            MapRange::new(53, 49, 8),
                        ],
                    },
                    Map {
                        cat_from: Category::Light,
                        cat_to: Category::Water,
                        ranges: vec![MapRange::new(18, 88, 7), MapRange::new(25, 18, 70),],
                    },
                    Map {
                        cat_from: Category::Temperature,
                        cat_to: Category::Light,
                        ranges: vec![
                            MapRange::new(45, 81, 19),
                            MapRange::new(64, 68, 13),
                            MapRange::new(77, 45, 23),
                        ],
                    },
                    Map {
                        cat_from: Category::Humidity,
                        cat_to: Category::Temperature,
                        ranges: vec![MapRange::new(0, 1, 69), MapRange::new(69, 0, 1),],
                    },
                    Map {
                        cat_from: Category::Location,
                        cat_to: Category::Humidity,
                        ranges: vec![MapRange::new(56, 60, 37), MapRange::new(93, 56, 4),],
                    },
                ]
            )
        );
    }

    #[test]
    fn test_get_mapped_val() {
        let map = Map {
            cat_from: Category::Soil,
            cat_to: Category::Seed,
            ranges: vec![MapRange::new(50, 52, 48), MapRange::new(98, 50, 2)],
        };

        assert_eq!(get_mapped_val(&map, 79), 81);
        assert_eq!(get_mapped_val(&map, 14), 14);
        assert_eq!(get_mapped_val(&map, 55), 57);
        assert_eq!(get_mapped_val(&map, 13), 13);
    }

    #[test]
    fn test_get_loc_for_seed() {
        let (_, cat_maps) = parse_input(SAMPLE_INPUT);

        assert_eq!(get_loc_for_seed(79, &cat_maps), 82);
        assert_eq!(get_loc_for_seed(14, &cat_maps), 43);
        assert_eq!(get_loc_for_seed(55, &cat_maps), 86);
        assert_eq!(get_loc_for_seed(13, &cat_maps), 35);
    }

    #[test]
    fn test_get_lowest_loc_for_seed() {
        assert_eq!(get_lowest_loc_for_seed(SAMPLE_INPUT), 35);
    }

    #[test]
    fn test_get_lowest_loc_for_seed_ranges() {
        assert_eq!(get_lowest_loc_for_seed_ranges(SAMPLE_INPUT), 46);
    }
}
