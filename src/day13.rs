use std::collections::{BinaryHeap, HashMap, HashSet};
use priority_queue::PriorityQueue;

pub(crate) fn day13() {
    /*
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
*/
    let example_lines =
        vec![
            "#.##..##.",
            "..#.##.#.",
            "##......#",
            "##......#",
            "..#.##.#.",
            "..##..##.",
            "#.#.##.#.",
            "",
            "#...##..#",
            "#....#..#",
            "..##..###",
            "#####.##.",
            "#####.##.",
            "..##..###",
            "#....#..#",

        ];
    // convert example lines to String
    let example_lines: Vec<String> =
        example_lines.iter().map(|s| s.to_string()).collect();

    part1(example_lines);

    let input = std::fs::read_to_string("./inputs/day13.txt").unwrap();
    // split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    part1(input);
}

fn part1(lines: Vec<String>) {
    /*
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
*/
    // let's split into subproblems divided by empty lines
    let mut subproblems: Vec<Vec<String>> = Vec::new();
    let mut subproblem: Vec<String> = Vec::new();
    for line in lines {
        if line == "" {
            subproblems.push(subproblem);
            subproblem = Vec::new();
        } else {
            subproblem.push(line);
        }
    }
    subproblems.push(subproblem);

    let mut sum = 0;
    for subproblem in subproblems {
        let (v, h) = solve_subproblem(subproblem.clone());
        // print grid
        for (pos, line) in subproblem.clone().iter().enumerate() {
            println!("{}: {}", pos, line);
        }
        println!("v={} h={} ", v, h);
        sum += h * 100 + v;
    }

    println!("part1: {}", sum);
}

fn solve_subproblem(lines: Vec<String>) -> (usize, usize) {
    /*
012345678
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.
     */
    // let's convert into grid
    let grid: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();
    // now we need to find vertical and horizontal symmetries lines

    let v_mirror_pos = vertical_mirror_pos(&grid);
    let h_mirror_pos = horizontal_mirror_pos(&grid);
    return (v_mirror_pos,  h_mirror_pos);
}

fn vertical_mirror_pos(grid: &Vec<Vec<char>>) -> usize {
// vertical
    // we need to scan horizontally pairs of columns, then expand to the left and right
    let mut mirror_pos = 0;
    while mirror_pos < grid[0].len() - 1 {
        let mut left = mirror_pos as i32;
        let mut right = (mirror_pos + 1) as i32;
        /*
    012345678
    #.##..##.
    ..#.##.#.
    lr
         */
        let mut is_mirror = true;
        while left >= 0 && right < grid[0].len() as i32 {
            // scan columns, they must be equal
            for y in 0..grid.len() {
                if grid[y][left as usize] != grid[y][right as usize] {
                    is_mirror = false;
                    break;
                }
            }
            if is_mirror {
                left -= 1;
                right += 1;
            } else {
                break;
            }
        }
        if is_mirror {
            break;
        } else {
            mirror_pos += 1;
        }
    }
    if mirror_pos == grid[0].len() - 1 {
        0
    } else {
        mirror_pos + 1
    }
}
fn horizontal_mirror_pos(grid: &Vec<Vec<char>>) -> usize {
// vertical
    // we need to scan vertically pairs of columns, then expand to up and down
    let mut mirror_pos = 0;
    while mirror_pos < grid.len() - 1 {
        let mut up = mirror_pos as i32;
        let mut down = (mirror_pos + 1) as i32;
        let mut is_mirror = true;
        while up >= 0 && down < grid.len() as i32 {
            // scan rows, they must be equal
            for x in 0..grid[0].len() {
                if grid[up as usize][x] != grid[down as usize][x] {
                    is_mirror = false;
                    break;
                }
            }
            if is_mirror {
                up -= 1;
                down += 1;
            } else {
                break;
            }
        }
        if is_mirror {
            break;
        } else {
            mirror_pos += 1;
        }
    }
    if mirror_pos == grid.len() - 1 {
        0
    } else {
        mirror_pos + 1
    }
}

fn part2(lines: Vec<String>) {

}













