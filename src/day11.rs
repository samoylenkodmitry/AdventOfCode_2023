use std::collections::{BinaryHeap, HashMap, HashSet};
use priority_queue::PriorityQueue;

pub(crate) fn day11() {
    /*
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
*/
    let example_lines =
        vec![
            "...#......",
            ".......#..",
            "#.........",
            "..........",
            "......#...",
            ".#........",
            ".........#",
            "..........",
            ".......#..",
            "#...#.....",
        ];
    // convert example lines to String
    let example_lines: Vec<String> =
        example_lines.iter().map(|s| s.to_string()).collect();

    part1(example_lines);

    let input = std::fs::read_to_string("./inputs/day11.txt").unwrap();
    // split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    part1(input);
}

fn part1(lines: Vec<String>) {
    /*
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
*/
    // convert lines to Vec<Vec<char>>
    let mut grid: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();
    // first, let's find all rows and columns without any galaxies #
    let empty_rows: Vec<usize> = grid.iter().enumerate()
        .filter(|(_, row)| !row.contains(&'#'))
        .map(|(i, _)| i)
        .collect();
    let empty_cols: Vec<usize> = (0..grid[0].len())
        .filter(|&j| !grid.iter().any(|row| row[j] == '#'))
        .collect();
    // now let's do BFS from each galaxy to find shortest paths for each pair
    // when we at row or col without any galaxies, we must wait 1 extra step
    // after some thought, I think we must store steps count in a BFS queue
    // and then we must take min of all steps counts
    // we can use sorted data structure to store shortest paths


    let mut galaxy_num = 0;
    let mut galaxy_positions: HashMap<usize, (usize, usize)> = HashMap::new();
    let mut positions_to_galaxy_num: HashMap<(usize, usize), usize> = HashMap::new();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '#' {
                galaxy_num += 1;
                galaxy_positions.insert(galaxy_num, (y, x));
                positions_to_galaxy_num.insert((y, x), galaxy_num);
            }
        }
    }

    // now each pair will be (galaxy_num_small, galaxy_num_large)
    let mut shortest_paths: HashMap<(usize, usize), usize> = HashMap::new();
    println!("galaxy nums {}, galaxy positions {:?}", galaxy_num, galaxy_positions);
    // now we are ready to do BFS from each galaxy
    for galaxy_num in 1..=galaxy_num {
        println!("checking galaxy num {}", galaxy_num);
        let start_galaxy = galaxy_num;
        let start_pos = galaxy_positions.get(&galaxy_num).unwrap();
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        // heap<(steps, (y, x))>
        let mut queue: PriorityQueue<(usize, (usize, usize)), i32> =
            // we need to retrieve the smallest steps first
            PriorityQueue::new();
        let mut item = (0, *start_pos);
        queue.push(item, 0);

        while !queue.is_empty() {
            let ((steps, (y, x)),_) = queue.pop().unwrap();
            if !visited.insert((y, x)) {
                continue;
            }
            // check if we reached some galaxy
            if grid[y][x] == '#' {
                let end_galaxy = positions_to_galaxy_num.get(&(y, x)).unwrap();
                if (*end_galaxy != start_galaxy) {
                    // make a key
                    let small_galaxy = std::cmp::min(start_galaxy, *end_galaxy);
                    let large_galaxy = std::cmp::max(start_galaxy, *end_galaxy);
                    // update shortest path
                    let key = (small_galaxy, large_galaxy);
                    let prev = shortest_paths.get(&key).unwrap_or(&steps);
                    shortest_paths.insert(key, std::cmp::min(steps, *prev));
                }
            }

            let is_in_empty_row_or_col = empty_rows.contains(&y) || empty_cols.contains(&x);
            let new_steps = if is_in_empty_row_or_col { steps + 2 } else { steps + 1 };
            let priority = -(new_steps as i32);

            // up
            if y > 0 && !visited.contains(&(y - 1, x)) {
                let item = (new_steps, (y - 1, x));
                queue.push(item, priority);
            }
            // down
            if y < grid.len() - 1 && !visited.contains(&(y + 1, x)) {
                let item = (new_steps, (y + 1, x));
                queue.push(item, priority);
            }
            // left
            if x > 0 && !visited.contains(&(y, x - 1)) {
                let item = (new_steps, (y, x - 1));
                queue.push(item, priority);
            }
            // right
            if x < grid[0].len() - 1 && !visited.contains(&(y, x + 1)) {
                let item = (new_steps, (y, x + 1));
                queue.push(item, priority);
            }
        }
    }
    // analyze shortest paths
    // let's find sum of the lengths of the shortest paths between all pairs of galaxies
    let mut sum = 0;
    for (_, &steps) in shortest_paths.iter() {
        sum += steps;
    }
    println!("shortest paths {:?}", shortest_paths);
    println!("part1: {}", sum);
}













