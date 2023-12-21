use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::i32;
use linked_hash_map::LinkedHashMap;
use priority_queue::PriorityQueue;

pub(crate) fn day21() {
    let raw_str =
        r###"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."###;
    let example_lines: Vec<&str> = raw_str.lines().collect();
    // convert example lines to String
    let example_lines: Vec<String> =
        example_lines.iter().map(|s| s.to_string()).collect();

    part1(example_lines);

    let input = std::fs::read_to_string("./inputs/day21.txt").unwrap();
    //// split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    part1(input);

}

fn part1(lines: Vec<String>) {
    /*
        r###"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."###;
     */
    let mut grid: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();
    // we need to find how many `.` cell we can visit in `x` steps
    let steps = 64;
    // let's do BFS
    // find (x, y) of S
    let mut start = (0, 0);
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'S' {
                start = (x, y);
            }
        }
    }
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back(start);
    let mut count = 0;

    for step in 0..=steps {
        let mut len = queue.len();
        count = 0;
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        for _ in 0.. len {
            let (x, y) = queue.pop_front().unwrap();
            if grid[y][x] == '.' || grid[y][x] == 'S' {
                if !visited.insert((x, y)) {
                    continue;
                }
                count += 1;

                // go left, top, right, bottom
                if x > 0 {
                    queue.push_back((x - 1, y));
                }
                if y > 0 {
                    queue.push_back((x, y - 1));
                }
                if x < grid[0].len() - 1 {
                    queue.push_back((x + 1, y));
                }
                if y < grid.len() - 1 {
                    queue.push_back((x, y + 1));
                }
            }
        }
        // print grid
        println!("step: {step}, count {count}");
        if false {
            for y in 0..grid.len() {
                for x in 0..grid[0].len() {
                    if visited.contains(&(x, y)) {
                        print!("O");
                    } else {
                        print!("{}", grid[y][x]);
                    }
                }
                println!();
            }
        }
    }

    println!("part 1: {count}");
}

fn part2(lines: Vec<String>) {

}

