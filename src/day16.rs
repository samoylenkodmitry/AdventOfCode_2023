use std::collections::{BinaryHeap, HashMap, HashSet};
use linked_hash_map::LinkedHashMap;
use priority_queue::PriorityQueue;

pub(crate) fn day16() {
// example:
    /*
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
*/
    let raw_str =
r###".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"###;
    let example_lines:Vec<&str> = raw_str.lines().collect();
    // convert example lines to String
    let example_lines: Vec<String> =
        example_lines.iter().map(|s| s.to_string()).collect();

    part1(example_lines);

    let input = std::fs::read_to_string("./inputs/day16.txt").unwrap();
    // split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    //part1(input);

    // part 2

    let example_lines:Vec<&str> = raw_str.lines().collect();
    // convert example lines to String
    let example_lines: Vec<String> =
        example_lines.iter().map(|s| s.to_string()).collect();

    part2(example_lines);

    let input = std::fs::read_to_string("./inputs/day16.txt").unwrap();
    // split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    part2(input);
}

fn part1(lines: Vec<String>) {
    /*
beam     -->.|...\....
            |.-.\.....
            .....|-...
            ........|.
            ..........
            .........\
            ..../.\\..
            .-.-/..|..
            .|....-|.\
            ..//.|....
*/
    let initial_beam = (0, 0, 'r');
    let mut grid: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();

    let count_visited = fire_beam(initial_beam, &mut grid);
    println!("Part 1: {}", count_visited);
}

fn fire_beam(initial_beam: (usize, usize, char), grid: &mut Vec<Vec<char>>) -> i32 {
// let's do a BFS and remember each visited node with a HashSet
    // we must store (x, y, beam_direction) in the HashSet
    // we will encode direction as a char: 'u', 'd', 'l', 'r'
    let mut visited: HashSet<(usize, usize, char)> = HashSet::new();
    // we will use a Queue to store the next nodes to visit
    let mut queue: Vec<(usize, usize, char)> = Vec::new();
    queue.push(initial_beam);
    // we don't need to store path
    // let's however, store count of visits for each cell
    let mut count: HashMap<(usize, usize), usize> = HashMap::new();

    let last_x = grid[0].len() - 1;
    let last_y = grid.len() - 1;
    // BFS
    while !queue.is_empty() {
        let (x, y, beam_direction) = queue.remove(0);
        if !visited.insert((x, y, beam_direction)) {
            continue;
        }
        // count visits
        *count.entry((x, y)).or_insert(0) += 1;

        // check current cell
        let current_cell = grid[y][x];
        // match beam direction with current cell
        // / and \ are mirrors, rotating the beam 90 degrees
        // | and - are splitters, splitting the beam like this -> | (goes up and down) and so on
        // . is empty space, beam goes straight

        match (beam_direction, current_cell) {
            ('u', '/') => if x < last_x { queue.push((x + 1, y, 'r')) },
            ('u', '\\') => if x > 0 { queue.push((x - 1, y, 'l')) },
            ('u', '|') => if y > 0 { queue.push((x, y - 1, 'u')) },
            ('u', '-') => {
                if x > 0 { queue.push((x - 1, y, 'l')) };
                if x < last_x { queue.push((x + 1, y, 'r')) };
            },
            ('u', '.') => if y > 0 { queue.push((x, y - 1, 'u')) },
            ('d', '/') => if x > 0 { queue.push((x - 1, y, 'l')) },
            ('d', '\\') => if x < last_x { queue.push((x + 1, y, 'r')) },
            ('d', '|') => if y < last_y { queue.push((x, y + 1, 'd')) },
            ('d', '-') => {
                if x > 0 { queue.push((x - 1, y, 'l')) };
                if x < last_x { queue.push((x + 1, y, 'r')) };
            },
            ('d', '.') => if y < last_y { queue.push((x, y + 1, 'd')) },
            ('l', '/') => if y < last_y { queue.push((x, y + 1, 'd')) },
            ('l', '\\') => if y > 0 { queue.push((x, y - 1, 'u')) },
            ('l', '|') => {
                if y > 0 { queue.push((x, y - 1, 'u')) };
                if y < last_y { queue.push((x, y + 1, 'd')) };
            },
            ('l', '-') => if x > 0 { queue.push((x - 1, y, 'l')) },
            ('l', '.') => if x > 0 { queue.push((x - 1, y, 'l')) },
            ('r', '/') => if y > 0 { queue.push((x, y - 1, 'u')) },
            ('r', '\\') => if y < last_y { queue.push((x, y + 1, 'd')) },
            ('r', '|') => {
                if y > 0 { queue.push((x, y - 1, 'u')) };
                if y < last_y { queue.push((x, y + 1, 'd')) };
            },
            ('r', '-') => if x < last_x { queue.push((x + 1, y, 'r')) },
            ('r', '.') => if x < last_x { queue.push((x + 1, y, 'r')) },

            _ => panic!("Invalid beam direction: {} and/or cell: {}", beam_direction, current_cell),
        }
    }
    // count cells visited more than once
    let mut count_visited = 0;
    for (_, v) in count.iter() {
        if *v > 0 {
            count_visited += 1;
        }
    }
    count_visited
}

fn part2(lines: Vec<String>) {

    /*
beam     -->.|...\....
            |.-.\.....
            .....|-...
            ........|.
            ..........
            .........\
            ..../.\\..
            .-.-/..|..
            .|....-|.\
            ..//.|....
*/
    let mut grid: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();
    let mut max_count = 0;
    let directions = vec!['u', 'd', 'l', 'r'];
    // try to fire all beams from the sides
    for y in 0..grid.len() {
        let count_visited = fire_beam((0, y, 'r'), &mut grid);
        if count_visited > max_count {
            max_count = count_visited;
        }
        let beam = (grid[0].len() - 1, y, 'l');
        let count_visited = fire_beam(beam, &mut grid);
        if count_visited > max_count {
            max_count = count_visited;
        }
    }
    for x in 0..grid[0].len() {
        let count_visited = fire_beam((x, 0, 'd'), &mut grid);
        if count_visited > max_count {
            max_count = count_visited;
        }
        let count_visited = fire_beam((x, grid.len() - 1, 'u'), &mut grid);
        if count_visited > max_count {
            max_count = count_visited;
        }
    }

    println!("Part 2: {}", max_count);
}
