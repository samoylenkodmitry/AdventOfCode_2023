use std::collections::{BinaryHeap, HashMap, HashSet};
use linked_hash_map::LinkedHashMap;
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

    //part1(example_lines);

    let input = std::fs::read_to_string("./inputs/day15.txt").unwrap();
    // split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    //part1(input);

    // part 2
    let example_lines =
        vec![
            "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7",
        ];
    // convert example lines to String
    let example_lines: Vec<String> =
        example_lines.iter().map(|s| s.to_string()).collect();

    part2(example_lines);

    let input = std::fs::read_to_string("./inputs/day15.txt").unwrap();
    // split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    part2(input);
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
        let hash = hash(s);
        sum += hash;
    }

    println!("part1: {}", sum);
}

fn hash(s: String) -> i32 {
    let mut hash = 0;
    for c in s.chars() {
        hash = ((hash + (c as i32)) * 17) % 256;
    }
    hash
}

fn part2(lines: Vec<String>) {
    /*
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
*/
    // we have 256 boxes, that will be filled with list of numbers
    let mut strs: Vec<String> = lines[0].split(",")
        .map(|s| s.to_string()).collect();
    // we must have 256 linked lists
    // within each list cell we have pair [label and num]
    // we must preserve order of first insertion
    let mut boxes: HashMap<i32, LinkedHashMap<String, i32>> = HashMap::new();
    // add empty 256 boxes
    for i in 0..256 {
        boxes.insert(i, LinkedHashMap::new());
    }
    for s in strs {
        // hash must be computed only for part before '=' or '-'
        if s.contains('-') {
            let label = s.split("-").next().unwrap();
            let hash = hash(label.to_string());
            // search for label in curr_box and remove if found
            boxes.get_mut(&hash).unwrap().remove(label);
        } else {
            let label = s.split("=").next().unwrap();
            let num = s.chars().last().unwrap().to_digit(10);
            let hash = hash(label.to_string());
            // search for label in curr_box and replace if found, otherwise add
            let curr_box = boxes.get_mut(&hash).unwrap();
            if (*curr_box).contains_key(label) {
                *(curr_box.get_mut(&label.to_string())).unwrap() = num.unwrap() as i32;
            } else {
                curr_box.insert(label.to_string(), num.unwrap() as i32);
            }
        }
    }
    //
    // now analyze boxes
    let mut sum = 0;
    for (hash, box_) in boxes {
        if box_.len() == 0 {
            continue;
        }
        for (ind, (label, num)) in box_.iter().enumerate() {
            let x = (hash + 1) * ((ind as i32) + 1) * (*num);
            sum += x;
        }
    }

    println!("part2: {}", sum);
}
