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

    //part1(example_lines);

    let input = std::fs::read_to_string("./inputs/day10.txt").unwrap();
    // split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    //part1(input);

    // part 2

    /*
OF----7F7F7F7F-7OOOO
O|F--7||||||||FJOOOO
O||OFJ||||||||L7OOOO
FJL7L7LJLJ||LJIL-7OO
L--JOL7IIILJS7F-7L7O
OOOOF-JIIF7FJ|L7L7L7
OOOOL7IF7||L7|IL7L7|
OOOOO|FJLJ|FJ|F7|OLJ
OOOOFJL-7O||O||||OOO
OOOOL---JOLJOLJLJOOO
*/
    let example_lines =
        vec![
            "OF----7F7F7F7F-7OOOO",
            "O|F--7||||||||FJOOOO",
            "O||OFJ||||||||L7OOOO",
            "FJL7L7LJLJ||LJIL-7OO",
            "L--JOL7IIILJS7F-7L7O",
            "OOOOF-JIIF7FJ|L7L7L7",
            "OOOOL7IF7||L7|IL7L7|",
            "OOOOO|FJLJ|FJ|F7|OLJ",
            "OOOOFJL-7O||O||||OOO",
            "OOOOL---JOLJOLJLJOOO",
        ];
    // convert example lines to String
    let example_lines: Vec<String> =
        example_lines.iter().map(|s| s.to_string()).collect();

    part2(example_lines);

    let input = std::fs::read_to_string("./inputs/day10.txt").unwrap();
    // split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    part2(input);
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

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let steps = do_bfs(&mut map, s_pos, &mut visited);

    println!("part1: {}", steps);
}

fn do_bfs(map: &mut Vec<Vec<char>>, mut s_pos: (i32, i32), visited: &mut HashSet<(i32, i32)>) -> i32 {
// now do BFS from S
    let mut steps = 0;
    let mut queue: Vec<(i32, i32)> = Vec::new();
    queue.push(s_pos);
    visited.insert(s_pos);
    while !queue.is_empty() {
        steps += 1;
        let sz = queue.len();
        for _ in 0..sz {
            let (x, y) = queue.remove(0);
            let curr_char = map[y as usize][x as usize];
            // check if we can go left
            if x > 0 {
                let left_char = map[y as usize][(x - 1) as usize];
                if (left_char == 'F' || left_char == 'L' || left_char == '-') &&
                    (curr_char == 'S' || curr_char == '7' || curr_char == 'J' || curr_char == '-') {
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
                if (right_char == 'J' || right_char == '7' || right_char == '-') &&
                    (curr_char == 'S' || curr_char == 'F' || curr_char == 'L' || curr_char == '-') {
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
                if (up_char == 'F' || up_char == '7' || up_char == '|') &&
                    (curr_char == 'S' || curr_char == 'J' || curr_char == 'L' || curr_char == '|') {
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
                if (down_char == 'J' || down_char == 'L' || down_char == '|') &&
                    (curr_char == 'S' || curr_char == 'F' || curr_char == '7' || curr_char == '|') {
                    let down_pos = (x, y + 1);
                    if !visited.contains(&down_pos) {
                        queue.push(down_pos);
                        visited.insert(down_pos);
                    }
                }
            }
        }
    }
    steps
}

fn part2(lines: Vec<String>) {
    /*
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|**|.|**|.
.L--J.L--J.
...........
*/
    // we need to find enclosed tiles inside a loop
    // they are marked with *
    // we can exclude every tile that is not in a loop and connected to borders:
    // x = 0, x = max, y = 0, y = max
    /*
```````````
`S-------7`
`|F-----7|`
`||`````||`
`||`````||`
`|L-7`F-J|`
`|**|`|**|`
`L--J`L--J`
```````````
*/
    // however, there is a corner case, when loop is like this:

    /*
``````````
`S------7`
`|F----7|`
`||....||`
`||....||`
`|L-7F-J|`
`|**||**|`
`L--JL--J`
``````````
     */
    // we somehow need to exclude remaining tiles `.`
    // we sure know every tile that is a pipe (from part1)
    // we sure know every tile that is on a border

    // chatgpt tells, there is a ray casting algorithm
    // if we cast a ray from a point,
    // and it intersects with an odd number of lines,
    // then the point is inside the loop

    /*
OF----7F7F7F7F-7OOOO
O|F--7||||||||FJOOOO
O||OFJ||||||||L7OOOO
FJL7L7LJLJ||LJIL-7OO
L--JOL7IIILJS7F-7L7O
OOOOF-JIIF7FJ|L7L7L7
OOOOL7IF7||L7|IL7L7|
OOOOO|FJLJ|FJ|F7|OLJ
OOOOFJL-7O||O||||OOO
OOOOL---JOLJOLJLJOOO
     */

    // let's convert map into a 2d array (store as Vec<Vec<char>>)
    let mut map: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();
    // now find S position
    // then do BFS from S

    // find S position
    let mut s_pos: (i32, i32) = (0, 0);
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'S' {
                s_pos = (x as i32, y as i32);
            }
        }
    }

    let mut pipes: HashSet<(i32, i32)> = HashSet::new();
    do_bfs(&mut map, s_pos, &mut pipes);

    // now we have all pipes that are in a loop
    // for each cell that is not in a pipe, we need to cast a ray
    // if it intersects with an odd number of lines, then the cell is in a loop

    let mut count_tiles_inside_loop = 0;
    for y in 0..map.len() - 0 {
        for x in 0..map[0].len() - 0 {
            // if it is not a pipe
            if pipes.contains(&(x as i32, y as i32)) {
                let char_in_pipe = match map[y][x] {
                    '-' => '─',
                    '|' => '│',
                    '7' => '┐',
                    'J' => '┘',
                    'F' => '┌',
                    'L' => '└',
                    'S' => ' ',

                    _ => panic!("unknown char "),
                };
                print!("{}", char_in_pipe);
            } else {
                if x == 0 || x == map[0].len() - 1 || y == 0 || y == map.len() - 1 {
                    print!(" ");
                    continue;
                }
                // cast a ray to the right
                let mut count_right = 0;
                for x2 in x + 1..map[0].len() {
                    let symbol = map[y][x2];
                    // this was the time when I gave up and searched for a solution
                    if (symbol == 'F' || symbol == '7' || symbol == '|') && pipes.contains(&(x2 as i32, y as i32)) {
                        count_right += 1;
                    }
                }
                if count_right % 2 == 1 {
                    count_tiles_inside_loop += 1;
                }
                if count_right != 0 {
                    print!(".");
                } else {
                    print!(" ");
                }
            }
        }
        println!();
    }

    println!("part2: {}", count_tiles_inside_loop);
}
