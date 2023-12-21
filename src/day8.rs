use std::collections::{HashMap, HashSet};

pub(crate) fn day8() {
    /*
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
*/
    let example_lines =
        vec![
            "RL",
            "",
            "AAA = (BBB, CCC)",
            "BBB = (DDD, EEE)",
            "CCC = (ZZZ, GGG)",
            "DDD = (DDD, DDD)",
            "EEE = (EEE, EEE)",
            "GGG = (GGG, GGG)",
            "ZZZ = (ZZZ, ZZZ)",
        ];
    // convert example lines to String
    let example_lines: Vec<String> =
        example_lines.iter().map(|s| s.to_string()).collect();

    //part1(example_lines);

    let input = std::fs::read_to_string("./inputs/day8.txt").unwrap();
    // split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    //part1(input);

    // part 2
    /*
   LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
     */

    let example_lines =
        vec![
            "LR",
            "",
            "11A = (11B, XXX)",
            "11B = (XXX, 11Z)",
            "11Z = (11B, XXX)",
            "22A = (22B, XXX)",
            "22B = (22C, 22C)",
            "22C = (22Z, 22Z)",
            "22Z = (22B, 22B)",
            "XXX = (XXX, XXX)",
        ];
    // convert example lines to String
    let example_lines: Vec<String> =
        example_lines.iter().map(|s| s.to_string()).collect();

    part2(example_lines);

    let input = std::fs::read_to_string("./inputs/day8.txt").unwrap();
    // split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    part2(input);
}

fn part1(lines: Vec<String>) {
    /*
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
*/
    // we go in a pattern R - right, L - left, like this right, left, right, left
    // we start at AAA, then go right of pair (bbb, ccc) to ccc
    // then go left at ccc in pair (zzz, ggg) to zzz
    // total operations are 2
    // the actual pattern is infinitely repeated, like RLRLRLRL...
    // what we really need to do is to find the num_ops[from][to][pattern_position]
    // num_ops[AAA][BBB][0] = 1
    // so this is a dynamic programming problem

    // pattern is a first line in lines
    let pattern: Vec<char> = lines[0].chars().collect();
    // num_ops[aaa][bbb]['L'] = 1
    // num_ops[aaa][bbb]['R'] = 1 + num_ops[ccc][bbb]['L']

    // let's build a graph stored in a hashmap
    // key is a node, value is a pair of nodes

    let mut graph: HashMap<String, (String, String)> = HashMap::new();
    for line in lines.iter().skip(2) {
        let line = line.split(" = ").collect::<Vec<&str>>();
        let node = line[0];
        let pair = line[1].split(", ")
            .map(|s| s.trim_matches(|c| c == '(' || c == ')'))
            .collect::<Vec<&str>>();

        graph.insert(node.to_string(), (pair[0].to_string(), pair[1].to_string()));
    }


    // to find a shortest path from aaa to zzz we can use bfs
    // however, we also must follow the pattern
    // so let's use bfs with a queue of (node, pattern_position)
    // we also need to store visited nodes

    let mut queue: Vec<String> = Vec::new();
    let mut visited: HashSet<(String, usize)> = HashSet::new();
    queue.push("AAA".to_string());
    visited.insert(("AAA".to_string(), 0));

    let mut ops = 0;
    while !queue.is_empty() {
        let sz = queue.len();
        for _ in 0..sz {
            let node = queue.remove(0);

            let (left, right) = graph.get(&node).unwrap();
            let left = left.to_string();
            let right = right.to_string();

            let next_node = if pattern[ops % pattern.len()] == 'L' {
                left
            } else {
                right
            };

            if next_node == "ZZZ" {
                break;
            }

            if visited.insert((next_node.clone(), ops % pattern.len())) {
                queue.push(next_node.clone());
            }
        }
        ops += 1;
    }

    println!("part1: {}", ops);
}

fn part2(lines: Vec<String>) {
    /*
   LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
*/
    // we go in a pattern R - right, L - left, like this right, left, right, left
    // the actual pattern is infinitely repeated, like RLRLRLRL...
    // 11A and 22A are starting points
    // 11A goes by LR pattern in two steps to 11Z
    // 22A goes by 22A(L) -> 22B(R) -> 22C(L) -> 22Z in tree steps
    // they both will reach ..Z items in 6 - the minimum common multiple of 2 and 3

    // pattern is a first line in lines
    let pattern: Vec<char> = lines[0].chars().collect();

    let mut graph: HashMap<String, (String, String)> = HashMap::new();
    for line in lines.iter().skip(2) {
        let line = line.split(" = ").collect::<Vec<&str>>();
        let node = line[0];
        let pair = line[1].split(", ")
            .map(|s| s.trim_matches(|c| c == '(' || c == ')'))
            .collect::<Vec<&str>>();

        graph.insert(node.to_string(), (pair[0].to_string(), pair[1].to_string()));
    }


    // to find a shortest path from aaa to zzz we can use bfs
    // however, we also must follow the pattern
    // so let's use bfs with a queue of (node, pattern_position)
    // we also need to store visited nodes

    let mut queue: Vec<String> = Vec::new();
    let mut visited: HashSet<(String, usize)> = HashSet::new();
    // push to queue every graph key, ending with 'a'
    for (node, _) in graph.iter() {
        if node.ends_with('A') {
            queue.push(node.to_string());
            visited.insert((node.to_string(), 0));
        }
    }

    let mut ops = 0;
    let mut result: u128 = 1;
    while !queue.is_empty() {
        let sz = queue.len();
        // the result will be multiplier of individual cycles
        for _ in 0..sz {
            let node = queue.remove(0);

            if node.ends_with('Z') {
                result = least_common_multiple(result, ops as u128);
                println!("another result ops: {}, queue: {:?}, result: {}", ops, queue, result);
                continue;
            }

            let (left, right) = graph.get(&node).unwrap();
            let left = left.to_string();
            let right = right.to_string();

            let next_node = if pattern[ops % pattern.len()] == 'L' {
                left
            } else {
                right
            };

            //if visited.insert((next_node.clone(), ops % pattern.len())) {
                queue.push(next_node.clone());
            //}
        }
        ops += 1;
    }

    println!("part2: {}", result);
}

fn least_common_multiple(a: u128, b: u128) -> u128 {
    a * b / greatest_common_divisor(a, b)
}

fn greatest_common_divisor(a: u128, b: u128) -> u128 {
    if b == 0 {
        return a;
    }
    greatest_common_divisor(b, a % b)
}

