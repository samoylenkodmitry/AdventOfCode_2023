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

    //part1(example_lines);

    let input = std::fs::read_to_string("./inputs/day9.txt").unwrap();
    // split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    //part1(input);

    // part 2
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

    part2(example_lines);

    let input = std::fs::read_to_string("./inputs/day9.txt").unwrap();
    // split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    part2(input);
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
        // we want to extrapolate TO THE LEFT by calculating the difference until difference is 0
        //    10  13  16  21  30  45
        // 5  10* 13  16  21  30  45    current_first = 10
        //     5   3*  3   5   9  15    current_first = 3, prev
        //        -2   0*  2   4   6
        //             2   2*  2   2
        //                 0*  0   0
        // then the extrapolated number is:
        // 2-2 = 0            2 = 2 - 0
        //   *                *
        // 0 - (-2) = 2      -2 = 0 - 2 = 0 - (2-0)
        //      *             *
        // 3 - 5 = -2         5 = 3 - (-2) = 3 - (0 - (2-0))
        //     *              *
        // 10 - 5 = 5         5 = 10 - 5 = 10 - (3 - (0 - (2-0)))
        //      *             *
        // prev_first - x = current_first
        // x = prev_first - current_first
        // next = sum(prev_first - current_first)
        // or alternate sign:
        // next = sum(first*sign_i), where sign_i = (-1)^(i+1)

        let mut next = *numbers.first().unwrap();
        let mut sign = -1;
        while numbers.iter().any(|&n| n != 0) {
            numbers = numbers.windows(2).map(|w| w[1] - w[0]).collect();
            let last = *numbers.first().unwrap();
            next += last * sign;
            sign *= -1;
        }
        sum += next;
    }

    println!("Day 9 part 2: {}", sum);

}
