use std::collections::{BinaryHeap, HashMap, HashSet};
use std::i32;
use linked_hash_map::LinkedHashMap;
use priority_queue::PriorityQueue;

pub(crate) fn day18() {
// example:
    /*
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
*/
    let raw_str =
        r###"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
"###;
    let example_lines: Vec<&str> = raw_str.lines().collect();
    // convert example lines to String
    let example_lines: Vec<String> =
        example_lines.iter().map(|s| s.to_string()).collect();

    //part1(example_lines);

    //let input = std::fs::read_to_string("./inputs/day18.txt").unwrap();
    //// split input into lines
    //let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    //part1(input);

    // part 2
    let example_lines: Vec<&str> = raw_str.lines().collect();
    // convert example lines to String
    let example_lines: Vec<String> =
        example_lines.iter().map(|s| s.to_string()).collect();

    part2(example_lines);

    let input = std::fs::read_to_string("./inputs/day18.txt").unwrap();
    //// split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    part2(input);
}

fn part1(lines: Vec<String>) {
    /*
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
*/
    // 1. we need to follow the path rules and make a grid xy
    //    we can use a hashmap to store the grid
    //    key: (x,y) value: color (like "#70c710")
    //    then normalize the grid so that the smallest x and y are 0
    // 2. then we need to find the inside of the convex hull
    //    we can count intersections of the grid
    //    if the number is odd, then it's inside the convex hull
    // 3. count all the inside points + path points, this will be the answer

    let mut grid: HashMap<(i32, i32), (String, char)> = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    grid.insert((x, y), ("#000000".to_string(), 'S'));
    let mut prev_dir = 'S';
    let mut first_dir: char = '.';
    lines.iter().for_each(|line| {
        //"U 2 (#7a21e3)"
        let mut split = line.split_whitespace();
        let dir = split.next().unwrap().chars().next().unwrap();
        if first_dir == '.' {
            first_dir = dir;
        }
        let steps: i32 = split.next().unwrap().parse().unwrap();
        let color = split.next().unwrap().to_string().replace("(", "").replace(")", "");
        let (dx, dy) = match dir {
            'U' => (0, -1),
            'D' => (0, 1),
            'L' => (-1, 0),
            'R' => (1, 0),
            _ => panic!("invalid direction"),
        };
        let rotation_part = get_path_part(prev_dir, dir);
        prev_dir = dir;
        // update previous point with rotation part
        let prev_point = grid.get_mut(&(x, y)).unwrap();
        prev_point.1 = rotation_part;
        // border part can be | -
        let border_part = match dir {
            'U' | 'D' => '|',
            'L' | 'R' => '-',
            _ => panic!("invalid direction"),
        };
        for _ in 0..steps {
            x += dx;
            y += dy;
            grid.insert((x, y), (color.clone(), border_part));
        }
    });
    let rotation_part = get_path_part(prev_dir, first_dir);
    // update previous point with rotation part
    let prev_point = grid.get_mut(&(x, y)).unwrap();
    prev_point.1 = rotation_part;
    // normalize the grid
    let min_x = grid.keys().map(|(x, _)| x).min().unwrap();
    let min_y = grid.keys().map(|(_, y)| y).min().unwrap();
    let max_x = grid.keys().map(|(x, _)| x).max().unwrap();
    let max_y = grid.keys().map(|(_, y)| y).max().unwrap();

    let mut normalized_grid: HashMap<(i32, i32), (String, char)> = grid.iter()
        .map(|((x, y), (color, segment))| {
            ((x - min_x, y - min_y), (color.clone(), segment.clone()))
        }).collect();
    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;
    // print the grid, # for path, . for empty
    /*
    for y in 0..height {
        for x in 0..width {
            let has_path = normalized_grid.contains_key(&(x, y));
            if has_path {
                let (color, path_part) = normalized_grid.get(&(x, y)).unwrap();
                print!("{}", path_part);
            } else {
                print!(".");
            }

        }
        println!();
    }

     */
    let mut inside_points = 0;
    // ok, now the fun part
    // let cast a ray from the origin (0,0) to the right
    for y in 0..height {
        let mut intersections = 0;
        for x in 0..width {
            let has_path = normalized_grid.contains_key(&(x, y));
            if has_path {
                let (color, path_part) = normalized_grid.get(&(x, y)).unwrap();
                if path_part == &'|' || path_part == &'F' || path_part == &'7' {
                    intersections += 1;
                }
                inside_points += 1;
            } else {
                if intersections % 2 == 1 {
                    inside_points += 1;
                    //println!("inside: ({},{})", x, y);
                }
            }
        }
    }
    println!("part1: {}", inside_points);
}

fn get_path_part(mut prev_dir: char, dir: char) -> char {
// rotation part can be F J L 7
    let rotation_part = match (prev_dir, dir) {
        ('S', 'U') => 'S',
        ('S', 'D') => 'S',
        ('S', 'L') => 'S',
        ('S', 'R') => 'S',
        ('U', 'U') => '-',
        ('U', 'D') => '|',
        ('U', 'L') => '7',
        ('U', 'R') => 'F',
        ('D', 'U') => '|',
        ('D', 'D') => '|',
        ('D', 'L') => 'J',
        ('D', 'R') => 'L',
        ('L', 'U') => 'L',
        ('L', 'D') => 'F',
        ('L', 'L') => '-',
        ('L', 'R') => '-',
        ('R', 'U') => 'J',
        ('R', 'D') => '7',
        ('R', 'L') => '-',
        ('R', 'R') => '-',
        _ => panic!("invalid direction"),
    };
    rotation_part
}

fn part2(lines: Vec<String>) {
    /*
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
*/
    // will use shoelace formula to calculate area

    let mut x: i128 = 0;
    let mut y: i128 = 0;
    let mut perimeter: i128 = 0;
    let mut inner_points: i128 = 0;
    lines.iter().for_each(|line| {
        //"U 2 (#7a21e3)"
        let mut split = line.split_whitespace();
        let color = split.nth(2).unwrap().to_string().replace("(#", "").replace(")", "");
        // we will parse the color like this:
        // take 5 next digits, convert to decimal - this will be 'steps' count
        // take last digit, this will be dir: 0 - R, 1 - D, 2 - L, 3 - U
        let dir = match color.chars().last().unwrap() {
            '0' => 'R',
            '1' => 'D',
            '2' => 'L',
            '3' => 'U',
            x => panic!("invalid direction {}", x),
        };
        // convert from hex
        let steps: i128 = i128_from_hex_str(color[0..5].to_string());
        perimeter += steps;
        let (dx, dy) = match dir {
            'U' => (0, -1),
            'D' => (0, 1),
            'L' => (-1, 0),
            'R' => (1, 0),
            _ => panic!("invalid direction"),
        };

        let new_x = x + dx * steps;
        let new_y = y + dy * steps;
        // shoelace formula
        inner_points += x * new_y - new_x * y;
        x = new_x;
        y = new_y;
    });

    let area = (perimeter + inner_points) / 2 + 1;
    println!("part2: {}", area);
}

fn i128_from_hex_str(hex_str: String) -> i128 {
    i128::from_str_radix(&hex_str, 16).unwrap()
}
