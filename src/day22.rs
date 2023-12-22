#![feature(iter_next_chunk)]

use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::{i32, mem};
use linked_hash_map::LinkedHashMap;
use priority_queue::PriorityQueue;
use rustc_hash::FxHashSet;

pub(crate) fn day22() {
    let raw_str =
        r###"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"###;
    let example_lines: Vec<&str> = raw_str.lines().collect();
    // convert example lines to String
    let example_lines: Vec<String> =
        example_lines.iter().map(|s| s.to_string()).collect();

    part1(example_lines);

    let input = std::fs::read_to_string("./inputs/day22.txt").unwrap();
    //// split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    part1(input);
}

fn part1(lines: Vec<String>) {
    /*
    r###"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"###;
     */
    // let's parse bricks coordinates [x1,y1,z1,x2,y2,z2], all numbers are 0..=512
    let mut bricks: Vec<Vec<u16>> = lines.iter().map(|line| {
        line.split(['~', ',']).map(|s| s.parse::<u16>().unwrap()).collect()
    }).collect();

    /*
    Now we need to make all the bricks fall down on 'z' axis
    and find on which bricks each brick falls
    We can use a HashMap to store the bricks that are under each brick
    As bricks fall they act like in a tetris game, they fall until they hit
    another brick or the ground
    We can use a HashSet to store the bricks that are on the ground
     */
    let mut bricks_under: HashMap<usize, HashSet<usize>> = HashMap::new();
    // and inverted HashMap
    let mut bricks_over: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut stopped_bricks: HashSet<usize> = HashSet::new();
    // let's sort the bricks by their 'y' coordinate
    let mut bricks_indices: Vec<usize> = (0..bricks.len()).collect();
    bricks_indices.sort_by(|a, b| {
        // each Vec has 6 coordinates, x, y, z, x, y, z
        let a = &bricks[*a];
        let b = &bricks[*b];
        a[2].cmp(&b[2])
    });
    // now we can fall bricks in order of their 'y' coordinate
    for brick_fall_ind in bricks_indices {
        let brick_fall = &bricks[brick_fall_ind];
        // if brick is already on the ground, add it to stopped_bricks, then continue
        if brick_fall[2] == 0 {
            stopped_bricks.insert(brick_fall_ind);
            continue;
        }
        // if brick is not on the ground, find the brick that is under it
        // and drop the brick on top of it
        // there can be multiple bricks under the falling brick
        // we must compare the z2 coordinate of the stopped_bricks
        // with the z1 coordinate of the falling brick
        // we can scan xy coordinates of the stopped_bricks
        // find all that intersect with the falling brick
        // then find all with the max z2 coordinate

        let (x1, y1, z1, x2, y2, z2) = (brick_fall[0], brick_fall[1], brick_fall[2],
                                        brick_fall[3], brick_fall[4], brick_fall[5]);
        // we can use a HashMap<z2, ind> to store the bricks that are under the falling brick
        let mut bricks_under_fall: HashMap<u16, Vec<usize>> = HashMap::new();
        let mut sz2_max = 0;
        for stopped_brick_ind in &stopped_bricks {
            let stopped_brick = &bricks[*stopped_brick_ind];
            // check if stopped_brick intersects with brick_fall
            // on XY plane bricks coordinates are rectangular x1,y1,x2,y2
            let (sx1, sy1, sz1, sx2, sy2, sz2) = (stopped_brick[0], stopped_brick[1], stopped_brick[2],
                                                  stopped_brick[3], stopped_brick[4], stopped_brick[5]);
            if intersect(x1, y1, x2, y2, sx1, sy1, sx2, sy2) {
                // if stopped_brick intersects with brick_fall
                // add it to bricks_under_fall
                bricks_under_fall.entry(sz2).or_insert(Vec::new()).push(*stopped_brick_ind);
                sz2_max = max(sz2_max, sz2);
            }
        }
        println!("sz2_max: {}", sz2_max);
        println!("bricks_under_fall: {:?}", bricks_under_fall);
        if bricks_under_fall.is_empty() {
            // if there are no bricks under the falling brick
            // put it on the ground
            // modify the brick_fall coordinates z1 and z2
            // and add it to stopped_bricks
            let brick_fall = &mut bricks[brick_fall_ind];
            let diff = z2 - z1;
            brick_fall[2] = sz2_max + 1;
            brick_fall[5] = sz2_max + diff + 1;
            stopped_bricks.insert(brick_fall_ind);
            continue;
        }
        // now we have all the bricks that are under the falling brick
        // we can add them to bricks_under
        let x3 = bricks_under_fall.get(&sz2_max).unwrap();
        let bricks_under_fall_set: HashSet<usize> = x3.iter().map(|x| *x).collect();
        bricks_under.insert(brick_fall_ind, bricks_under_fall_set.clone());
        // and add the falling brick to bricks_over
        for brick_under_fall_ind in bricks_under_fall_set {
            bricks_over.entry(brick_under_fall_ind).or_insert(HashSet::new()).insert(brick_fall_ind);
        }
        // finally modify the brick_fall coordinates z1 and z2
        // and add it to stopped_bricks
        let brick_fall = &mut bricks[brick_fall_ind];
        let diff = z2 - z1;
        brick_fall[2] = sz2_max + 1;
        brick_fall[5] = sz2_max + diff + 1;
        stopped_bricks.insert(brick_fall_ind);
    }

    println!("bricks: {:?}", bricks);
    // print XZ of bricks
    for z in (0..9).rev() {
        for x in 0..=2 {
            let mut found = 0;
            let mut found_ind = -1;
            for (ind, brick) in bricks.iter().enumerate() {
                if brick[0] <= x && x <= brick[3] && brick[2] <= z && z <= brick[5] {
                    found += 1;
                    found_ind = ind as i32;
                }
            }
            if found == 0 {
                print!(".");
            } else if found == 1 {
                print!("{}", found_ind);
            } else {
                print!("?");
            }
        }
        println!();
    }
    println!("---");
    // print YZ of bricks
    for z in (0..9).rev() {
        for y in 0..=2 {
            let mut found = 0;
            let mut found_ind = -1;
            for (ind, brick) in bricks.iter().enumerate() {
                if brick[1] <= y && y <= brick[4] && brick[2] <= z && z <= brick[5] {
                    found += 1;
                    found_ind = ind as i32;
                }
            }
            if found == 0 {
                print!(".");
            } else if found == 1 {
                print!("{}", found_ind);
            } else {
                print!("?");
            }
        }
        println!();
    }
    println!("bricks_under: {:?}", bricks_under);
    println!("bricks_over: {:?}", bricks_over);

    // now we need to count the number of bricks that can be safely removed in one step
    // if brick lies on just a single brick, that supporting brick can NOT be removed
    // however if brick lies on multiple bricks, all of them can be removed
    // we need to analyze who is supporting who

    // start from simple, let's count all bricks that have no bricks on top of them
    let mut bricks_no_over: HashSet<usize> = HashSet::new();
    for brick_ind in 0..bricks.len() {
        if !bricks_over.contains_key(&brick_ind) {
            bricks_no_over.insert(brick_ind);
        }
    }
    println!("bricks_no_over: {:?}", bricks_no_over);

    // now let's count all bricks that have only single brick below them, let's remember that brick
    // we can use bricks_under HashMap for that
    let mut bricks_single_under: HashSet<usize> = HashSet::new();
    for (brick_ind, bricks_under_set) in &bricks_under {
        if bricks_under_set.len() == 1 {
            bricks_single_under.insert(bricks_under_set.iter().next().unwrap().clone());
        }
    }
    println!("bricks_single_under: {:?}", bricks_single_under);

    let count = bricks.len() - bricks_single_under.len();

    println!("Part 1: {}", count);
}

// check if two rectangles intersect
// x1,y1,x2,y2 and sx1,sy1,sx2,sy2
fn intersect(x1: u16, y1: u16, x2: u16, y2: u16, sx1: u16, sy1: u16, sx2: u16, sy2: u16) -> bool {
    // check if x1,y1 is inside sx1,sy1,sx2,sy2
    if x1 >= sx1 && x1 <= sx2 && y1 >= sy1 && y1 <= sy2 {
        return true;
    }
    // check if x2,y2 is inside sx1,sy1,sx2,sy2
    if x2 >= sx1 && x2 <= sx2 && y2 >= sy1 && y2 <= sy2 {
        return true;
    }
    // check if sx1,sy1 is inside x1,y1,x2,y2
    if sx1 >= x1 && sx1 <= x2 && sy1 >= y1 && sy1 <= y2 {
        return true;
    }
    // check if sx2,sy2 is inside x1,y1,x2,y2
    if sx2 >= x1 && sx2 <= x2 && sy2 >= y1 && sy2 <= y2 {
        return true;
    }
    false
}

fn part2(input: &str) {}