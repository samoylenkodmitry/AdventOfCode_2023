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

    //part1(example_lines);

    let input = std::fs::read_to_string("./inputs/day22.txt").unwrap();
    //// split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    part1(input);
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Brick {
    x1: u16,
    y1: u16,
    z1: u16,
    x2: u16,
    y2: u16,
    z2: u16,
}

impl Brick {
    fn from_str(s: &str) -> Brick {
        let mut nums: Vec<u16> = s.split([',', '~']).map(|s| s.parse::<u16>().unwrap()).collect();
        if nums.len() != 6 {
            panic!("nums.len() != 6");
        }
        if nums[2] > nums[5] {
            panic!("nums[2] > nums[5]");
        }
        Brick {
            x1: nums[0],
            y1: nums[1],
            z1: nums[2],
            x2: nums[3],
            y2: nums[4],
            z2: nums[5],
        }
    }
    fn intersects(&self, other: &Self) -> bool {
        !(self.x2 < other.x1 || self.x1 > other.x2 ||
            self.y2 < other.y1 || self.y1 > other.y2 ||
            self.z2 < other.z1 || self.z1 > other.z2)
    }
    fn intersects_xy(&self, other: &Self) -> bool {
        !(self.x2 < other.x1 || self.x1 > other.x2 ||
            self.y2 < other.y1 || self.y1 > other.y2)
    }
}

fn parse_bricks(lines: Vec<String>) -> Vec<Brick> {
    lines.iter().map(|line| Brick::from_str(line)).collect()
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
    let mut bricks: Vec<Brick> = parse_bricks(lines);

    /*
    Now we need to make all the bricks fall down on 'z' axis
    and find on which bricks each brick falls
    We can use a HashMap to store the bricks that are under each brick
    As bricks fall they act like in a tetris game, they fall until they hit
    another brick or the ground
    We can use a HashSet to store the bricks that are on the ground
     */
    let mut brick_to_bottoms: HashMap<usize, HashSet<usize>> = HashMap::new();
    // and inverted HashMap
    let mut brick_to_tops: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut stopped_bricks: HashSet<usize> = HashSet::new();
    // let's sort the bricks by their 'y' coordinate
    let mut bricks_indices: Vec<usize> = (0..bricks.len()).collect();
    bricks_indices.sort_by(|a, b| {
        // each Vec has 6 coordinates, x, y, z, x, y, z
        let a = &bricks[*a];
        let b = &bricks[*b];
        a.z1.cmp(&b.z1)
    });
    // now we can fall bricks in order of their 'y' coordinate
    for brick_fall_ind in bricks_indices {
        let brick_fall = &bricks[brick_fall_ind];
        // if brick is already on the ground, add it to stopped_bricks, then continue
        if brick_fall.z1 == 0 {
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

        // we can use a Vec<ind> to store the bricks that are under the falling brick
        // and intersect it
        let mut bricks_under_fall: HashSet<usize> = HashSet::new();
        let mut sz2_max = 0;
        for stopped_brick_ind in &stopped_bricks {
            let stopped_brick = &bricks[*stopped_brick_ind];
            // check if stopped_brick intersects with brick_fall
            // on XY plane bricks coordinates are rectangular x1,y1,x2,y2
            if stopped_brick.intersects_xy(brick_fall) {
                if stopped_brick.z2 > brick_fall.z1 {
                    // if stopped_brick is above the falling brick
                    panic!("sz2 > z1");
                }
                // if stopped_brick intersects with brick_fall
                // add it to bricks_under_fall
                if stopped_brick.z2 > sz2_max {
                    sz2_max = stopped_brick.z2;
                    bricks_under_fall.clear();
                    bricks_under_fall.insert(*stopped_brick_ind);
                } else if stopped_brick.z2 == sz2_max {
                    bricks_under_fall.insert(*stopped_brick_ind);
                }
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
            let diff = brick_fall.z2 - brick_fall.z1;
            brick_fall.z1 = sz2_max + 1;
            brick_fall.z2 = sz2_max + diff + 1;
            stopped_bricks.insert(brick_fall_ind);
            continue;
        }
        // now we have all the bricks that are under the falling brick
        // we can add them to bricks_under
        brick_to_bottoms.insert(brick_fall_ind, bricks_under_fall.clone());
        // and add the falling brick to bricks_over
        // let's check that z2 is the same for all bricks_under_fall
        let mut sz2 = u16::MAX;
        for brick_under_fall_ind in &bricks_under_fall {
            let brick_under_fall = &bricks[*brick_under_fall_ind];
            if sz2 == u16::MAX {
                sz2 = brick_under_fall.z2;
            } else if brick_under_fall.z2 != sz2 {
                panic!("brick_under_fall[5] != sz2");
            }
        }
        for brick_under_fall_ind in bricks_under_fall {
            brick_to_tops.entry(brick_under_fall_ind).or_insert(HashSet::new()).insert(brick_fall_ind);
        }
        // finally modify the brick_fall coordinates z1 and z2
        // and add it to stopped_bricks
        let brick_fall = &mut bricks[brick_fall_ind];
        let diff = brick_fall.z2 - brick_fall.z1;
        brick_fall.z1 = sz2_max + 1;
        brick_fall.z2 = sz2_max + diff + 1;
        stopped_bricks.insert(brick_fall_ind);
    }

    println!("bricks_under: {:?}", brick_to_bottoms);
    println!("bricks_over: {:?}", brick_to_tops);

    // now let's count all bricks that have only single brick below them, let's remember that brick
    // we can use bricks_under HashMap for that
    let mut bricks_single_under: HashSet<usize> = HashSet::new();
    for (brick_ind, bricks_under_set) in &brick_to_bottoms {
        if bricks_under_set.len() == 1 {
            bricks_single_under.insert(bricks_under_set.iter().next().unwrap().clone());
        }
    }
    println!("bricks_single_under: {:?}", bricks_single_under);

    let count = bricks.len() - bricks_single_under.len();

    println!("Part 1: {}", count);
}

fn part2(input: &str) {}

#[cfg(test)]
mod tests_day22 {
    use super::*;

    #[test]
    fn test_day22_part1_intersect_same_z() {
        let brick1 = Brick::from_str("0,0,0~1,1,0");
        let brick2 = Brick::from_str("0,0,0~1,1,0");
        assert!(brick1.intersects(&brick2));
    }

    #[test]
    fn test_day22_part1_intersect_xy_different_z() {
        let brick1 = Brick::from_str("0,0,0~1,1,0");
        let brick2 = Brick::from_str("0,0,1~1,1,1");
        assert!(brick1.intersects_xy(&brick2));
    }
}