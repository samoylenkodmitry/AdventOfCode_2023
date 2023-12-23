#![feature(iter_next_chunk)]

use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::{i32, mem};
use linked_hash_map::LinkedHashMap;
use priority_queue::PriorityQueue;
use rustc_hash::FxHashSet;

pub(crate) fn day23() {
    let raw_str =
        r###"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"###;
    let example_lines: Vec<&str> = raw_str.lines().collect();
    // convert example lines to String
    let example_lines: Vec<String> =
        example_lines.iter().map(|s| s.to_string()).collect();

    part1(example_lines);

    let input = std::fs::read_to_string("./inputs/day23.txt").unwrap();
    //// split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    part1(input);
}

fn part1(lines: Vec<String>) {
    /*

        r###"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"###;
     */
    // find the longest path from top row to bottom row
    let grid: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();
    // let's use a BFS to find the longest path
    // for each path we must maintain the visited set
    // and the current position
    // queue will be (x, y, visited_set<(x, y)>)
    let mut queue: VecDeque<(usize, usize, HashSet<(usize, usize)>)> = VecDeque::new();
    // start at the top row
    for x in 0..grid[0].len() {
        if grid[0][x] == '.' {
            queue.push_back((x, 0, HashSet::new()));
        }
    }
    let mut max_path = 0;
    while !queue.is_empty() {
        let (x, y, mut visited) = queue.pop_front().unwrap();
        if !visited.insert((x, y)) {
            continue;
        }
        if y == grid.len() - 1 {
            //println!("x: {}, y: {}, len: {}, visited: {:?}", x, y, visited.len(),
            //         visited.iter().map(|&(x, y)| format!("{}", grid[y][x])).collect::<Vec<String>>());
            max_path = max(max_path, visited.len());
            continue;
        }
        match grid[y][x] {
            '#' => {}
            '.' => {
                // try 4 directions
                [
                    if x > 0 { (x - 1, y) } else { (x, y) },
                    if x < grid[0].len() - 1 { (x + 1, y) } else { (x, y) },
                    if y > 0 { (x, y - 1) } else { (x, y) },
                    if y < grid.len() - 1 { (x, y + 1) } else { (x, y) }
                ].iter()
                    .filter(|&&(x1, y1)| !(x1 == x && y1 == y))
                    .filter(|&&(x, y)| grid[y][x] != '#')
                    .for_each(|&(x, y)| {
                    queue.push_back((x, y, visited.clone()));
                });
            }
            '<' => {
                // must go left
                if x > 0 && grid[y][x - 1] != '#' {
                    queue.push_back((x - 1, y, visited));
                }
            }
            '>' => {
                // must go right
                if x < grid[0].len() - 1 && grid[y][x + 1] != '#' {
                    queue.push_back((x + 1, y, visited));
                }
            }
            '^' => {
                // must go up
                if y > 0 && grid[y - 1][x] != '#' {
                    queue.push_back((x, y - 1, visited));
                }
            }
            'v' => {
                // must go down
                if y < grid.len() - 1 && grid[y + 1][x] != '#' {
                    queue.push_back((x, y + 1, visited));
                }
            }
            _ => { panic!("invalid char: {}", grid[y][x]) }
        }
    }

    println!("part1: {}", max_path - 1);
}

fn part2(lines: Vec<String>) {}