use std::collections::{BinaryHeap, HashMap, HashSet};
use priority_queue::PriorityQueue;

pub(crate) fn day15() {
    /*
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
*/
    let example_lines =
        vec![
            "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7",
        ];
    // convert example lines to String
    let example_lines: Vec<String> =
        example_lines.iter().map(|s| s.to_string()).collect();

    part1(example_lines);

    let input = std::fs::read_to_string("./inputs/day15.txt").unwrap();
    // split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    part1(input);

}

fn part1(lines: Vec<String>) {
    /*
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
*/
    // split by comma
    let mut strs: Vec<String> = lines[0].split(",")
        .map(|s| s.to_string()).collect();
    let mut sum = 0;
    // for each str compute hash like this, considering every char:
    // hash = ((hash + char_asci_value) * 17) % 256
    // add result hashes in sum
    for s in strs {
        let mut hash = 0;
        for c in s.chars() {
            hash = ((hash + (c as i32)) * 17) % 256;
        }
        sum += hash;
    }

    println!("part1: {}", sum);
}

fn part2(lines: Vec<String>) {

}
