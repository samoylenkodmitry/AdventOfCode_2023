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

    part1(example_lines);

    let input = std::fs::read_to_string("./inputs/day14.txt").unwrap();
    // split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    part1(input);

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
    println!("part1: {}", sum);
}

fn part2(lines: Vec<String>) {

}












