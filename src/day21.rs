#![feature(iter_next_chunk)]

use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::{i32, mem};
use linked_hash_map::LinkedHashMap;
use priority_queue::PriorityQueue;
use rustc_hash::FxHashSet;

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

    //part1(example_lines);

    let input = std::fs::read_to_string("./inputs/day21.txt").unwrap();
    //// split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    part1(input);

    // part 2
    let example_lines: Vec<&str> = raw_str.lines().collect();
    // convert example lines to String
    let example_lines: Vec<String> =
        example_lines.iter().map(|s| s.to_string()).collect();

    //part2(example_lines);

    let string = std::fs::read_to_string("./inputs/day21.txt").unwrap();
    let input_str: &str = string.as_str();
    part2(input_str);
}

fn part2(input: &str) {
    let [mut v, mut v2] = [FxHashSet::<[isize; 2]>::from_iter([[131 / 2; 2]]), FxHashSet::<[isize; 2]>::default()];
    let [d, a, b, c] = (1..)
        .filter_map(|step| {
            v.drain().for_each(|[x, y]| {
                [[0, 1], [0, -1], [-1, 0], [1, 0]].iter().for_each(|[dx, dy]| {
                    let [nx, ny] = [x + dx, y + dy];
                    let i = (ny.rem_euclid(131) * (131 + 1) + nx.rem_euclid(131)) as usize;
                    let _ = (input.as_bytes()[i] != b'#').then(|| v2.insert([nx, ny]));
                });
            });

            mem::swap(&mut v, &mut v2);

            (step == 64 || step % 131 == 65 /*26501365 % 131*/).then_some(v.len() as u64)
        })
        .next_chunk()
        .unwrap();

    let n = 202300; //26501365 / 131;
    //println!("d: {}, a: {}, b: {}, c: {}", d, a, b, c);
    let res = [d, a + (b - a) * n + (a + c - 2 * b) * (n * (n - 1) / 2)];
    println!("part 2 {:?}", res)
    /*
d: 3666, a: 3752, b: 33614, c: 93252
part 2 [3666, 609298746763952]
     */
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
        for _ in 0..len {
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