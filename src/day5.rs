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
        } else if line.ends_with("map:") {
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
    let nums = get_final_ranges(lines);
    // find min value in 'from' of ranges
    let res = nums.iter().min_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    println!("part2: {}", res);
}

fn get_final_ranges(lines: Vec<String>) -> Vec<(u128, u128)> {
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
    // to solve for ranges, we must maintain them as ranges (from, count)
    // and analyze on transformation step
    // transformation can intersect range in multiple ways:
    // 1. miss range - do nothing
    // 2. include all range - apply transformation to 'from' and put (new_from, count) to next_nums
    // 3. include part of range:
    // 3.1. be inside range - we need to split range into 3 parts, left, middle, right
    //                        apply transformation only to the middle part
    //                        and put all three parts into next_nums
    // 3.2. intersect left part of range - we need to split range into 2 parts, left and right
    //                                     apply transformation only to the left part
    //                                    and put all two parts into next_nums
    // 3.3. intersect right part of range - we need to split range into 2 parts, left and right
    //                                      apply transformation only to the right part
    //                                    and put all two parts into next_nums
    let mut nums: Vec<(u128, u128)> = Vec::new();
    let mut next_nums: Vec<(u128, u128)> = Vec::new();
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
                .map(|s| s.parse::<u128>().unwrap())
                .collect::<Vec<u128>>();
            for i in 0..ranges.len() / 2 {
                let start = ranges[i * 2];
                let count = ranges[i * 2 + 1];
                nums.push((start, count));
            }
        } else if line.ends_with("map:") {
            // ignore
        } else {
            // numbers destination - source - count
            // 60 56 37
            let mut nums_str = line.split(" ");
            let destination = nums_str.next().unwrap().parse::<u128>().unwrap();
            let source = nums_str.next().unwrap().parse::<u128>().unwrap();
            let transform_count = nums_str.next().unwrap().parse::<u128>().unwrap();
            // now we must scan current nums and check if any of them are
            // in range [source..=source+count]
            // if so, remove from nums[] and push to next_nums[] value of
            // destination + (current - source)
            let mut i = 0;
            while i < nums.len() {
                let (from, count) = nums[i];
                // 1. if range outside of transformation range - do nothing
                if from > source + transform_count || from + count < source {
                    i += 1;
                    continue;
                }
                // 2. if range inside of transformation range - apply transformation to 'from'
                //    and put (new_from, count) to next_nums
                if from >= source && from + count <= source + transform_count {
                    next_nums.push((destination + (from - source), count));
                    nums.remove(i);
                    continue;
                }
                // 3. if range intersects transformation range - split range into 3 parts
                //    left, middle, right
                //    apply transformation only to the middle part
                //    and put all three parts into next_nums
                if from < source && from + count > source + transform_count {
                    // ******************** range
                    // ^                  ^
                    // f                  f+count
                    //       ssss*****-> transformation and shift by ssss (destination - source)
                    //       ^   ^
                    //       s   s+transform_count
                    // ******     *********
                    //
                    // left part must *not* be shifted
                    let left = (from, source - from);
                    // middle part must be shifted by destination - source
                    let middle = (destination, transform_count);
                    // right part must *not* be shifted
                    let right = (source + transform_count+1, count - ((source - from) + transform_count));
                    next_nums.push(left);
                    next_nums.push(middle);
                    next_nums.push(right);
                    nums.remove(i);
                    continue;
                }
                // 4. if range intersects left part of transformation range - split range into 2 parts
                //    left and right
                //    apply transformation only to the left part
                //    and put all two parts into next_nums
                if from < source && from + count >= source {
                    let left = (from, source - from);
                    let right = (destination, from + count - source);
                    next_nums.push(left);
                    next_nums.push(right);
                    nums.remove(i);
                    continue;
                }
                // 5. if range intersects right part of transformation range - split range into 2 parts
                //    left and right
                //    apply transformation only to the right part
                //    and put all two parts into next_nums
                if from <= source + transform_count && from + count > source + transform_count {
                    let left = (destination + (from - source), source + transform_count - from);
                    let right = (destination + transform_count + (from + count - source - transform_count), from + count - source - transform_count);
                    next_nums.push(left);
                    next_nums.push(right);
                    nums.remove(i);
                    continue;
                }
                i += 1;
            }
        }
    }
    next_nums.append(&mut nums);
    nums = next_nums.clone();
    next_nums.clear();
    nums
}

// let's write simple unit-test for part2 checking a single step of all intersection variations
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2_transform_split_range_into_3_parts() {
        // we have a range [1..=1+4] = [1..=5]
        // and transformation [2..=2+1] = [2..=3] with destination 6
        // we must split range into 3 parts
        // left = [1..=2-1] = [1..=1]
        // middle = [2..=2+1] = [2..=3]
        // right = [2+1+1..=1+4] = [4..=5]
        // apply transformation only to the middle part
        // middle must be shifted by destination - source
        // middle = [2+6-2..=3+6-2] = [6..=7]


        let lines =
            vec![
                "seeds: 1 4",
                "",
                "seed-to-soil map:",
                "6 2 1",
            ];
        // convert example lines to String
        let lines: Vec<String> =
            lines.iter().map(|s| s.to_string()).collect();
        let nums = get_final_ranges(lines);
        assert_eq!(nums.len(), 3);
        assert_eq!(nums[0], (1, 1));
        assert_eq!(nums[1], (6, 1));
        assert_eq!(nums[2], (4, 2));
    }

    #[test]
    fn test_part2_transform_split_range_into_2_parts_left() {
        // we have a range [1..=1+4] = [1..=5], or (1, 4)
        // and transformation [0..=0+2] = [0..=2] or (0, 2)  with destination 6
        // we must split range into 2 parts
        let lines =
            vec![
                "seeds: 1 4",
                "",
                "seed-to-soil map:",
                "6 0 2",
            ];
        // convert example lines to String
        let lines: Vec<String> =
            lines.iter().map(|s| s.to_string()).collect();
        let nums = get_final_ranges(lines);
        assert_eq!(nums.len(), 2);
        assert_eq!(nums[0], (6, 3));
        assert_eq!(nums[1], (3, 2));

    }
}
