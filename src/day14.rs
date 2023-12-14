use std::collections::{BinaryHeap, HashMap, HashSet};
use priority_queue::PriorityQueue;

pub(crate) fn day14() {
    /*
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
*/
    let example_lines =
        vec![
            "O....#....",
            "O.OO#....#",
            ".....##...",
            "OO.#O....O",
            ".O.....O#.",
            "O.#..O.#.#",
            "..O..#O..O",
            ".......O..",
            "#....###..",
            "#OO..#....",

        ];
    // convert example lines to String
    let example_lines: Vec<String> =
        example_lines.iter().map(|s| s.to_string()).collect();

    //part1(example_lines);

    let input = std::fs::read_to_string("./inputs/day14.txt").unwrap();
    // split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    //part1(input);

    // part 2
    let example_lines =
        vec![
            "O....#....",
            "O.OO#....#",
            ".....##...",
            "OO.#O....O",
            ".O.....O#.",
            "O.#..O.#.#",
            "..O..#O..O",
            ".......O..",
            "#....###..",
            "#OO..#....",

        ];
    // convert example lines to String
    let example_lines: Vec<String> =
        example_lines.iter().map(|s| s.to_string()).collect();

    //part2(example_lines);

    let input = std::fs::read_to_string("./inputs/day14.txt").unwrap();
    // split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    part2(input);
}

fn part1(lines: Vec<String>) {
    /*
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
*/
    // tilt to the up, all 'O's will slide up, but '#' will stay
    // we may iterate over each column
    // and maintain position of 'O' binding to the top
    let grid: Vec<Vec<char>> =
        lines.iter().map(|s| s.chars().collect()).collect();
    // main loop is horizontal
    let sum = check_sum(grid);
    println!("part1: {}", sum);
}

fn check_sum(grid: Vec<Vec<char>>) -> i32 {
    let mut sum = 0;
    for x in 0..grid[0].len() {
        let mut binding = grid.len() as i32; // will go 10, 9, 8, ..., 1
        for y in 0..grid.len() {
            if grid[y][x] == 'O' {
                sum += binding;
                //println!("x={} b={} y={}", x, binding, y);
                binding -= 1;
            } else if grid[y][x] == '#' {
                // rock # will prevent movement of 'O' up
                binding = (grid.len() as i32) - (y as i32) - 1;
                //println!("# x={} b={} y={}", x, binding, y);
            }
        }
    }
    sum
}

fn check_sum2(grid: Vec<Vec<char>>) -> i32 {
    let mut sum = 0;
    let mut h = grid.len() as i32;
    for x in 0..grid[0].len() {
        for y in 0..grid.len() {
            if grid[y][x] == 'O' {
                // y: 0 -> f = h
                sum += h - y as i32;
            }
        }
    }
    sum
}

fn part2(lines: Vec<String>) {
    // let's do the actual swapping

    /*
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
*/
    // tilt to the up, all 'O's will slide up, but '#' will stay
    // we may iterate over each column
    // and maintain position of 'O' binding to the top
    let mut grid: Vec<Vec<char>> =
        lines.iter().map(|s| s.chars().collect()).collect();
    // repeat x times
    let x = 1000;
    let need_calc = false;
    if need_calc {
        for i in 1..=x {
            flip_up(&mut grid);
            flip_left(&mut grid);
            flip_down(&mut grid);
            flip_right(&mut grid);
            let sum = check_sum2(grid.clone());
            println!("i={} sum: {}", i, sum);
        }
    }

    /*
    // print grid
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            print!("{}", grid[y][x]);
        }
        println!();
    }

     */
    // it looks like this
    // i: [1.... 901]
    //             ^ at iter = 901 we start cycle of 54 elements
    /*
    i=901 sum: 94888
i=902 sum: 94924
i=903 sum: 94959
i=904 sum: 94977
i=905 sum: 94988
i=906 sum: 95004
i=907 sum: 95020
i=908 sum: 95044
i=909 sum: 95046
i=910 sum: 95041
i=911 sum: 95026
i=912 sum: 95021
i=913 sum: 95018
i=914 sum: 95009
i=915 sum: 94988
i=916 sum: 94965
i=917 sum: 94930
i=918 sum: 94894
i=919 sum: 94876
i=920 sum: 94856
i=921 sum: 94839
i=922 sum: 94828
i=923 sum: 94811
i=924 sum: 94816
i=925 sum: 94831
i=926 sum: 94841
i=927 sum: 94864
i=928 sum: 94888
i=929 sum: 94924
i=930 sum: 94959
i=931 sum: 94977
i=932 sum: 94988
i=933 sum: 95004
i=934 sum: 95020
i=935 sum: 95044
i=936 sum: 95046
i=937 sum: 95041
i=938 sum: 95026
i=939 sum: 95021
i=940 sum: 95018
i=941 sum: 95009
i=942 sum: 94988
i=943 sum: 94965
i=944 sum: 94930
i=945 sum: 94894
i=946 sum: 94876
i=947 sum: 94856
i=948 sum: 94839
i=949 sum: 94828
i=950 sum: 94811
i=951 sum: 94816
i=952 sum: 94831
i=953 sum: 94841
i=954 sum: 94864

     */
    // so at iter = 1_000_000_000
    // we will have the following sequence of indices repeated:
    // [1...900] [901..954] [(901+54)..(905+54*2)][(901+54*2)..(901+54*3)]...
    // ..[(901+54*n)..1_000_000_000..(901+54*(n+1))]
    //                ^ we need to find this index
    // imagine we have period of 3 elements, starting from 2 and need to find 10th
    // 1 2 3 4 5 6 7 8 9 10
    // - 1 2 3 1 2 3 1 2 3
    //  |     |     |    ^ |
    //                   we need to find this index
    // the formula is: (10 - (2 - 1)) % 3 = 3
    // so we need to find (1_000_000_000 - (901 - 1)) % 54 = 46


    println!("part2: {}", 94876);
}

fn flip_up(grid: &mut Vec<Vec<char>>) {
// main loop is horizontal
    for x in 0..grid[0].len() {
        let mut binding = 0;
        for y in 0..grid.len() {
            if grid[y][x] == 'O' {
                // swap with binding
                grid[y][x] = '.';
                grid[binding][x] = 'O';
                binding += 1;
            } else if grid[y][x] == '#' {
                // rock # will prevent movement of 'O' up
                binding =  y + 1
            }
        }
    }
}

fn flip_left(grid: &mut Vec<Vec<char>>) {
    // main loop is vertical
    for y in 0..grid.len() {
        let mut binding = 0;
        for x in 0..grid[0].len() {
            if grid[y][x] == 'O' {
                // swap with binding
                grid[y][x] = '.';
                grid[y][binding] = 'O';
                binding += 1;
            } else if grid[y][x] == '#' {
                // rock # will prevent movement of 'O' up
                binding =  x + 1
            }
        }
    }
}


fn flip_down(grid: &mut Vec<Vec<char>>) {
// main loop is horizontal
    for x in 0..grid[0].len() {
        let mut binding = (grid.len() - 1) as i32;
        for y in (0..grid.len()).rev() {
            if grid[y][x] == 'O' {
                // swap with binding
                grid[y][x] = '.';
                grid[binding as usize][x] = 'O';
                binding -= 1;
            } else if grid[y][x] == '#' {
                // rock # will prevent movement of 'O' up
                binding =  y as i32 - 1
            }
        }
    }
}

fn flip_right(grid: &mut Vec<Vec<char>>) {
    // main loop is vertical
    for y in 0..grid.len() {
        let mut binding = grid[0].len() as i32 - 1;
        for x in (0..grid[0].len()).rev() {
            if grid[y][x] == 'O' {
                // swap with binding
                grid[y][x] = '.';
                grid[y][binding as usize] = 'O';
                binding -= 1;
            } else if grid[y][x] == '#' {
                // rock # will prevent movement of 'O' up
                binding =  x as i32 - 1
            }
        }
    }
}









