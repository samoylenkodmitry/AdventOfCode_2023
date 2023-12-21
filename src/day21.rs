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

    //part1(example_lines);

    let input = std::fs::read_to_string("./inputs/day21.txt").unwrap();
    //// split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    //part1(input);

    // part 2
    let example_lines: Vec<&str> = raw_str.lines().collect();
    // convert example lines to String
    let example_lines: Vec<String> =
        example_lines.iter().map(|s| s.to_string()).collect();

    //part2(example_lines);

    let input = std::fs::read_to_string("./inputs/day21.txt").unwrap();
    //// split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    part2(input);
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
    let steps = 131*2 + 66;
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
    // for the part 2 we must infinitely expand the grid, so let's also track
    // the grid repetition position, like [grid-2][grid-1][grid0][grid1][grid2]...
    //                                    [grid (-1, -2)][grid (-1, -1)][grid (-1, 0)][grid (-1, 1)][grid (-1, 2)]
    // so, basically we need to track the grid position and the cell position
    let mut queue: VecDeque<(usize, usize, i32, i32)> = VecDeque::new();
    queue.push_back((start.0, start.1, 0, 0));
    let mut count: u128 = 0;

    let mut f65 = 0;
    let mut f131_65 = 0;
    let mut f2_131_65 = 0;
    for step in 0..=steps {
        let mut len = queue.len();
        count = 0;
        let mut visited: HashSet<(usize, usize, i32, i32)> = HashSet::new();
        for _ in 0..len {
            let (x, y, grid_x, grid_y) = queue.pop_front().unwrap();
            if grid[y][x] == '.' || grid[y][x] == 'S' {
                if !visited.insert((x, y, grid_x, grid_y)) {
                    continue;
                }
                count += 1;

                // go left, top, right, bottom
                if x > 0 {
                    queue.push_back((x - 1, y, grid_x, grid_y));
                } else {
                    // go to the left grid
                    queue.push_back((grid[0].len() - 1, y, grid_x - 1, grid_y));
                }
                if y > 0 {
                    queue.push_back((x, y - 1, grid_x, grid_y));
                } else {
                    // go to the top grid
                    queue.push_back((x, grid.len() - 1, grid_x, grid_y - 1));
                }
                if x < grid[0].len() - 1 {
                    queue.push_back((x + 1, y, grid_x, grid_y));
                } else {
                    // go to the right grid
                    queue.push_back((0, y, grid_x + 1, grid_y));
                }
                if y < grid.len() - 1 {
                    queue.push_back((x, y + 1, grid_x, grid_y));
                } else {
                    // go to the bottom grid
                    queue.push_back((x, 0, grid_x, grid_y + 1));
                }
            }
        }
        if (step == 66) {
            f65 = count;
        }
        if (step == 131 + 66) {
            f131_65 = count;
        }
        if (step == 131*2 + 66) {
            f2_131_65 = count;
        }
    }
    /*
y0 = resultFor(65)
y1 = resultFor(131 + 65)
y2 = resultFor(131 * 2 + 65)
// ax^2 + bx + c
c = y0
b = y1 - y0
a = (y2 - y1 - b) / 3
resultFor(131 * 202300 + 65) == a * 202300 * 202300 + b * 202300 + c
     */
    let y0:u128 = f65;
    let y1 = f131_65;
    let y2 = f2_131_65;
    let c = y0;
    let b = y1 - y0;
    let a = (y2 - y1 - b) / 3;
    let result = a * 202300 * 202300 + b * 202300 + c;
    println!("f65: {f65}, f131_65: {f131_65}, f2_131_65: {f2_131_65}");
    println!("part 2: {result}");
}

