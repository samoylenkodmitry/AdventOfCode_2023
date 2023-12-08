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

    part1(example_lines);

    let input = std::fs::read_to_string("./inputs/day8.txt").unwrap();
    // split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    part1(input);

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
    let pattern:Vec<char> = lines[0].chars().collect();
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

    let mut queue: Vec<(String, usize)> = Vec::new();
    let mut visited: HashSet<(String, usize)> = HashSet::new();
    queue.push(("AAA".to_string(), 0));
    visited.insert(("AAA".to_string(), 0));

    let mut ops = 0;
    while !queue.is_empty() {
        let sz = queue.len();
        for _ in 0..sz {
            let (node, pattern_position) = queue.remove(0);

            println!("node: {}, pattern_position: {}", node, pattern_position);
            let (left, right) = graph.get(&node).unwrap();
            let left = left.to_string();
            let right = right.to_string();

            let next_pattern_position = (pattern_position + 1) % pattern.len();
            let next_node = if pattern[pattern_position] == 'L' {
                left
            } else {
                right
            };

            if next_node == "ZZZ" {
                break;
            }

            if visited.insert((next_node.clone(), next_pattern_position)) {
                queue.push((next_node.clone(), next_pattern_position));
            }

        }
        ops += 1;
    }

    println!("part1: {}", ops);

}

fn part2(lines: Vec<String>) {

}

