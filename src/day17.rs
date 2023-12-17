use std::collections::{BinaryHeap, HashMap, HashSet};
use linked_hash_map::LinkedHashMap;
use priority_queue::PriorityQueue;

pub(crate) fn day17() {
// example:
    /*
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
*/
    let raw_str =
        r###"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
"###;
    let example_lines: Vec<&str> = raw_str.lines().collect();
    // convert example lines to String
    let example_lines: Vec<String> =
        example_lines.iter().map(|s| s.to_string()).collect();

    part1(example_lines);

    let input = std::fs::read_to_string("./inputs/day17.txt").unwrap();
    // split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    part1(input);
}

fn part1(lines: Vec<String>) {
    /*
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
*/
    /*
    we start at top left corner
    we end at bottom right corner
    we can move up, down, left, right,
    we can't reverse direction (so keep track of previous direction)
    we can't move more than 3 blocks in a single direction (so keep track of distance)
    we must minimize the total path sum (so keep track of total path sum)

    let's use algorithm A* (A star)
     */
    let mut grid: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();
    // store direction like 'l', 'r', 'u', 'd'
    let mut visited_to_total: HashMap<(usize, usize, char, usize), usize> = HashMap::new();
    // in queue we must store (x, y, direction, distance, total_path_sum)
    // priority is total_path_sum
    let mut queue: PriorityQueue<(usize, usize, char, usize, usize), (i32, i32)> = PriorityQueue::new();
    // we start at top left corner, but don't have any direction
    queue.push((1, 0, 'r', 2, 0), (0, 0));
    queue.push((0, 1, 'd', 2, 0), (0, 0));
    // we end at bottom right corner
    let last_x = grid.len() - 1;
    let last_y = grid[0].len() - 1;
    let mut res: i32 = -1;
    while !queue.is_empty() {
        let ((x, y, direction, count, total_path_sum), (priority, manhattan)) = queue.pop().unwrap();
        //println!("x: {}, y: {}, direction: {}, distance: {}, total_path_sum: {}, cell {}",
        //         x, y, direction, count, total_path_sum, grid[y][x]);

        let mh = (last_x as i32 - x as i32).abs() + (last_y as i32 - y as i32).abs();
        let prev_total = visited_to_total.entry((x, y, direction, count)).or_insert(0);
        // check if we visited this cell
        if *prev_total > 0 && *prev_total <= total_path_sum {
            continue;
        }
        visited_to_total.insert((x, y, direction, count), total_path_sum);

        let total = total_path_sum + grid[y][x].to_digit(10).unwrap() as usize;
        // check if we reached the end
        if x == last_x && y == last_y {
            if (res == -1) || (total < res as usize) {
                res = total as i32;
                //println!("path: {:?}", new_path);
                break;
            }
        }
        let priority: i32 = -(total as i32);
        // let's match (direction)
        let can_go_straight = count < 3;
        match direction {
            ('-') => {
                // entry point is top left
            }
            ('l') => {
                // explore left, up, down
                if can_go_straight && x > 0 { queue.push((x - 1, y, 'l', count + 1, total), (priority, -mh)) } else { None };
                if y > 0 { queue.push((x, y - 1, 'u', 1, total), (priority, -mh)) } else { None };
                if y < last_y { queue.push((x, y + 1, 'd', 1, total), (priority, -mh)) } else { None };
            }
            ('r') => {
                // explore right, up, down
                if can_go_straight && x < last_x { queue.push((x + 1, y, 'r', count + 1, total), (priority, -mh)) } else { None };
                if y > 0 { queue.push((x, y - 1, 'u', 1, total), (priority, -mh)) } else { None };
                if y < last_y { queue.push((x, y + 1, 'd', 1, total), (priority, -mh)) } else { None };
            }
            ('u') => {
                // explore up, left, right
                if can_go_straight && y > 0 { queue.push((x, y - 1, 'u', count + 1, total), (priority, -mh)) } else { None };
                if x > 0 { queue.push((x - 1, y, 'l', 1, total), (priority, -mh)) } else { None };
                if x < last_x { queue.push((x + 1, y, 'r', 1, total), (priority, -mh)) } else { None };
            }
            ('d') => {
                // explore down, left, right
                if can_go_straight && y < last_y { queue.push((x, y + 1, 'd', count + 1, total), (priority, -mh)) } else { None };
                if x > 0 { queue.push((x - 1, y, 'l', 1, total), (priority, -mh)) } else { None };
                if x < last_x { queue.push((x + 1, y, 'r', 1, total), (priority, -mh)) } else { None };
            }

            (_) => { panic!("unknown case {}", direction); }
        }
    }


    println!("Part 1: {}", res);
}

fn part2(lines: Vec<String>) {}
