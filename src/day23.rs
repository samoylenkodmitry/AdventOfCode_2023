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

    // part 2
    let example_lines: Vec<&str> = raw_str.lines().collect();
    // convert example lines to String
    let example_lines: Vec<String> =
        example_lines.iter().map(|s| s.to_string()).collect();

    part2(example_lines);

    let input = std::fs::read_to_string("./inputs/day23.txt").unwrap();
    //// split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    part2(input);
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

fn part2(lines: Vec<String>) {
    // find the longest path from top row to bottom row
    let grid: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();
    // ok, BFS is not going to work here
    // we need to collapse the grid into a graph
    // any point with more than 2 neighbors is a node
    // to fill in the nodes, we need to do a BFS
    let mut nodes_set: HashSet<(usize, usize)> = HashSet::new();
    let mut nodes_vec: Vec<(usize, usize)> = Vec::new();
    let mut grid_to_node_ind: HashMap<(usize, usize), usize> = HashMap::new();
    // find all nodes
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '#' {
                continue;
            }
            let mut neighbors = 0;
            if x > 0 && grid[y][x - 1] != '#' {
                neighbors += 1;
            }
            if x < grid[0].len() - 1 && grid[y][x + 1] != '#' {
                neighbors += 1;
            }
            if y > 0 && grid[y - 1][x] != '#' {
                neighbors += 1;
            }
            if y < grid.len() - 1 && grid[y + 1][x] != '#' {
                neighbors += 1;
            }
            if neighbors > 2 {
                if nodes_set.insert((x, y)) {
                    nodes_vec.push((x, y));
                    grid_to_node_ind.insert((x, y), nodes_vec.len() - 1);
                }
            }
        }
    }
    nodes_vec.push((1, 0));
    nodes_set.insert((1, 0));
    let start_node_ind = nodes_vec.len() - 1;
    grid_to_node_ind.insert((1, 0), start_node_ind);

    let end_node_x = (0..grid[0].len()).find(|&x| grid[grid.len() - 1][x] == '.').unwrap();
    nodes_vec.push((end_node_x, grid.len() - 1));
    nodes_set.insert((end_node_x, grid.len() - 1));
    grid_to_node_ind.insert((end_node_x, grid.len() - 1), nodes_vec.len() - 1);
    let end_node_ind = nodes_vec.len() - 1;

    // let's store graph as a hashmap of (node_ind) -> Vec<(node_ind, distance)>
    let mut graph: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    // if we find a node, we need to add it to the graph
    // and then continue the BFS
    // queue will be (x, y, dist)
    // let's just start bfs from every node
    for prev_node_ind in 0..nodes_vec.len() {
        let (x, y) = nodes_vec[prev_node_ind];
        // find all adjacent nodes and break;
        let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
        queue.push_back((x, y, 0));

        while let Some((x, y, dist)) = queue.pop_front() {
            if nodes_set.contains(&(x, y)) {
                // found a node, connect it to the previous node
                let node_ind = grid_to_node_ind[&(x, y)];
                if node_ind != prev_node_ind {
                    graph.entry(prev_node_ind).or_insert_with(Vec::new).push((node_ind, dist));
                    graph.entry(node_ind).or_insert_with(Vec::new).push((prev_node_ind, dist));
                    continue;
                }
            }
            if !visited.insert((x, y)) {
                continue;
            }
            if x > 0 && grid[y][x - 1] != '#' {
                queue.push_back((x - 1, y, dist + 1));
            }
            if x < grid[0].len() - 1 && grid[y][x + 1] != '#' {
                queue.push_back((x + 1, y, dist + 1));
            }
            if y > 0 && grid[y - 1][x] != '#' {
                queue.push_back((x, y - 1, dist + 1));
            }
            if y < grid.len() - 1 && grid[y + 1][x] != '#' {
                queue.push_back((x, y + 1, dist + 1));
            }
        }
    }

    // now we have a graph, let's find the longest path
    // we can brute force this

    let mut max_path = 0;
    // each path must have a visited set
    // queue will be (node_ind, visited_set<(node_ind), path_dist>)
    let mut queue: VecDeque<(usize, HashSet<usize>, usize)> = VecDeque::new();
    queue.push_back((start_node_ind, HashSet::new(), 0));
    while !queue.is_empty() {
        let (node_ind, visited, path_dist) = queue.pop_front().unwrap();
        //println!("node_ind: {}, path_dist: {}", node_ind, path_dist);
        if node_ind == end_node_ind {
            max_path = max(max_path, path_dist);
            continue;
        }
        let mut visited = visited.clone();
        if visited.insert(node_ind) {
            for &(next_node_ind, dist) in &graph[&node_ind] {
                queue.push_back((next_node_ind, visited.clone(), dist + path_dist));
            }
        }
    }


    println!("part2: {}", max_path/* - 1*/);
}
