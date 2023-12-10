use std::collections::{HashMap, HashSet};

pub(crate) fn day10() {
    /*
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
*/
    let example_lines =
        vec![
            "..F7.",
            ".FJ|.",
            "SJ.L7",
            "|F--J",
            "LJ...",
        ];
    // convert example lines to String
    let example_lines: Vec<String> =
        example_lines.iter().map(|s| s.to_string()).collect();

    part1(example_lines);

    let input = std::fs::read_to_string("./inputs/day10.txt").unwrap();
    // split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    part1(input);
}

fn part1(lines: Vec<String>) {
    /*
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
*/
    // let's convert map into a 2d array (store as Vec<Vec<char>>)
    let mut map: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();
    // now find S position
    // then do BFS from S
    // we must follow the shape logic:
    // J is _| shape,
    // F is |- shape,
    // L is L shape,
    // 7 is -| shape
    // we can go to the left if left char is F or L
    // we can go to the right if right char is J or 7
    // we can go up if up char is F or 7
    // we can go down if down char is J or L
    // also there are - and | chars

    // find S position
    let mut s_pos: (i32, i32) = (0, 0);
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'S' {
                s_pos = (x as i32, y as i32);
            }
        }
    }

    // now do BFS from S
    let mut queue: Vec<(i32, i32)> = Vec::new();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    queue.push(s_pos);
    visited.insert(s_pos);
    let mut steps = -1;
    while !queue.is_empty() {
        steps += 1;
        let sz = queue.len();
        for _ in 0..sz {
            let (x, y) = queue.remove(0);
            let curr_char = map[y as usize][x as usize];
            // check if we can go left
            if x > 0 {
                let left_char = map[y as usize][(x - 1) as usize];
                if left_char == 'F' || left_char == 'L' || left_char == '-' {
                    let left_pos = (x - 1, y);
                    if !visited.contains(&left_pos) {
                        queue.push(left_pos);
                        visited.insert(left_pos);
                    }
                }
            }
            // check if we can go right
            if x < map[0].len() as i32 - 1 {
                let right_char = map[y as usize][(x + 1) as usize];
                if right_char == 'J' || right_char == '7' || right_char == '-' {
                    let right_pos = (x + 1, y);
                    if !visited.contains(&right_pos) {
                        queue.push(right_pos);
                        visited.insert(right_pos);
                    }
                }
            }
            // check if we can go up
            if y > 0 {
                let up_char = map[(y - 1) as usize][x as usize];
                if up_char == 'F' || up_char == '7' || up_char == '|' {
                    let up_pos = (x, y - 1);
                    if !visited.contains(&up_pos) {
                        queue.push(up_pos);
                        visited.insert(up_pos);
                    }
                }
            }
            // check if we can go down
            if y < map.len() as i32 - 1 {
                let down_char = map[(y + 1) as usize][x as usize];
                if down_char == 'J' || down_char == 'L' || down_char == '|' {
                    let down_pos = (x, y + 1);
                    if !visited.contains(&down_pos) {
                        queue.push(down_pos);
                        visited.insert(down_pos);
                    }
                }
            }
        }
    }

    println!("part1: {}", steps);
}

fn part2(lines: Vec<String>) {}
