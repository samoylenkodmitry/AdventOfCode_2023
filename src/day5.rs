use std::collections::{HashMap, HashSet};

pub(crate) fn day5() {
    /*
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
*/
    let example_lines =
        vec![
            "seeds: 79 14 55 13",
            "",
            "seed-to-soil map:",
            "50 98 2",
            "52 50 48",
            "",
            "soil-to-fertilizer map:",
            "0 15 37",
            "37 52 2",
            "39 0 15",
            "",
            "fertilizer-to-water map:",
            "49 53 8",
            "0 11 42",
            "42 0 7",
            "57 7 4",
            "",
            "water-to-light map:",
            "88 18 7",
            "18 25 70",
            "",
            "light-to-temperature map:",
            "45 77 23",
            "81 45 19",
            "68 64 13",
            "",
            "temperature-to-humidity map:",
            "0 69 1",
            "1 0 69",
            "",
            "humidity-to-location map:",
            "60 56 37",
            "56 93 4",
            "",
        ];
    // convert example lines to String
    let example_lines: Vec<String> =
        example_lines.iter().map(|s| s.to_string()).collect();

    part1(example_lines);

    let input = std::fs::read_to_string("./inputs/day5.txt").unwrap();
    // split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    part1(input);
    // part 2

    let example_lines =
        vec![
            "seeds: 79 14 55 13",
            "",
            "seed-to-soil map:",
            "50 98 2",
            "52 50 48",
            "",
            "soil-to-fertilizer map:",
            "0 15 37",
            "37 52 2",
            "39 0 15",
            "",
            "fertilizer-to-water map:",
            "49 53 8",
            "0 11 42",
            "42 0 7",
            "57 7 4",
            "",
            "water-to-light map:",
            "88 18 7",
            "18 25 70",
            "",
            "light-to-temperature map:",
            "45 77 23",
            "81 45 19",
            "68 64 13",
            "",
            "temperature-to-humidity map:",
            "0 69 1",
            "1 0 69",
            "",
            "humidity-to-location map:",
            "60 56 37",
            "56 93 4",
            "",
        ];
    // convert example lines to String
    let example_lines: Vec<String> =
        example_lines.iter().map(|s| s.to_string()).collect();

    part2(example_lines);

    let input = std::fs::read_to_string("./inputs/day5.txt").unwrap();
    // split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    part2(input);
}

fn part1(lines: Vec<String>) {
    /*
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
*/
    // we can iterate line by line
    // on each section we must find the transformation
    // destination - source - count
    let mut nums: Vec<i128> = Vec::new();
    let mut next_nums: Vec<i128> = Vec::new();
    for line in lines {
        if line == "" { // new section
            next_nums.append(&mut nums);
            nums = next_nums.clone();
            next_nums.clear();
        } else if line.starts_with("seeds:") {
            //seeds: 79 14 55 13
            let mut seeds_str = line.split(": ").nth(1).unwrap();
            nums = seeds_str.split(" ")
                .map(|s| s.parse::<i128>().unwrap())
                .collect();
        } else if (line.ends_with("map:")) {
            // ignore
        } else {
            // numbers destination - source - count
            // 60 56 37
            let mut nums_str = line.split(" ");
            let destination = nums_str.next().unwrap().parse::<i128>().unwrap();
            let source = nums_str.next().unwrap().parse::<i128>().unwrap();
            let count = nums_str.next().unwrap().parse::<i128>().unwrap();
            // now we must scan current nums and check if any of them are
            // in range [source..=source+count]
            // if so, remove from nums[] and push to next_nums[] value of
            // destination + (current - source)
            let mut i = 0;
            while i < nums.len() {
                if nums[i] >= source && nums[i] <= source + count {
                    next_nums.push(destination + (nums[i] - source));
                    nums.remove(i);
                } else {
                    i += 1;
                }
            }
        }
    }
    next_nums.append(&mut nums);
    nums = next_nums.clone();
    next_nums.clear();
    let res = nums.iter().min().unwrap();
    println!("part1: {}", res);
}

fn part2(lines: Vec<String>) {
    /*
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
*/
    // we can iterate line by line
    // on each section we must find the transformation
    // destination - source - count
    let mut nums: Vec<i128> = Vec::new();
    let mut next_nums: Vec<i128> = Vec::new();
    for line in lines {
        if line == "" { // new section
            next_nums.append(&mut nums);
            nums = next_nums.clone();
            next_nums.clear();
        } else if line.starts_with("seeds:") {
            //seeds: 79 14 55 13
            // this is a ranges of nubmers: [79..=79+14], [55..=55+13]
            // add all numbers in ranges to nums[]
            let mut seeds_str = line.split(": ").nth(1).unwrap();
            let ranges = seeds_str.split(" ")
                .map(|s| s.parse::<i128>().unwrap())
                .collect::<Vec<i128>>();
            for i in 0..ranges.len() / 2 {
                let start = ranges[i * 2];
                let end = start + ranges[i * 2 + 1];
                for j in start..=end {
                    nums.push(j);
                }
            }

        } else if (line.ends_with("map:")) {
            // ignore
        } else {
            // numbers destination - source - count
            // 60 56 37
            let mut nums_str = line.split(" ");
            let destination = nums_str.next().unwrap().parse::<i128>().unwrap();
            let source = nums_str.next().unwrap().parse::<i128>().unwrap();
            let count = nums_str.next().unwrap().parse::<i128>().unwrap();
            // now we must scan current nums and check if any of them are
            // in range [source..=source+count]
            // if so, remove from nums[] and push to next_nums[] value of
            // destination + (current - source)
            let mut i = 0;
            while i < nums.len() {
                if nums[i] >= source && nums[i] <= source + count {
                    next_nums.push(destination + (nums[i] - source));
                    nums.remove(i);
                } else {
                    i += 1;
                }
            }
        }
    }
    next_nums.append(&mut nums);
    nums = next_nums.clone();
    next_nums.clear();
    let res = nums.iter().min().unwrap();
    println!("part2: {}", res);
}

