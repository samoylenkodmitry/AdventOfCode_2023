use std::collections::{HashMap, HashSet};

pub(crate) fn day9() {
    /*
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
*/
    let example_lines =
        vec![
            "0 3 6 9 12 15",
            "1 3 6 10 15 21",
            "10 13 16 21 30 45",
        ];
    // convert example lines to String
    let example_lines: Vec<String> =
        example_lines.iter().map(|s| s.to_string()).collect();

    part1(example_lines);

    let input = std::fs::read_to_string("./inputs/day9.txt").unwrap();
    // split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    part1(input);

}

fn part1(lines: Vec<String>) {
    /*
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
*/

    // analyze each line separately
    let mut sum = 0;
    for line in lines {
        let mut numbers: Vec<i32> = line.split(" ")
            .map(|s| s.parse::<i32>().unwrap()).collect();

        // each line is a sequence of numbers
        // we want to extrapolate by calculating the difference until difference is 0
        // 0 3 6 9 12 15
        //   3 3 3  3  3
        //     0 0  0  0
        // then the extrapolated number is a sum of the last numbers in differences lists
        // next = 0 + 3 + 15 = 18

        let mut next = *numbers.last().unwrap();
        while numbers.iter().any(|&n| n != 0) {
            numbers = numbers.windows(2).map(|w| w[1] - w[0]).collect();
            let last = *numbers.last().unwrap();
            next += last;
        }
        sum += next;
    }

    println!("Day 9 part 1: {}", sum);

}

fn part2(lines: Vec<String>) {

}
