#![feature(iter_next_chunk)]

use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::{i32, mem};
use linked_hash_map::LinkedHashMap;
use priority_queue::PriorityQueue;
use rustc_hash::FxHashSet;

pub(crate) fn day24() {
    let raw_str =
        r###"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3"###;
    let example_lines: Vec<&str> = raw_str.lines().collect();
    // convert example lines to String
    let example_lines: Vec<String> =
        example_lines.iter().map(|s| s.to_string()).collect();

    part1(example_lines);

    let input = std::fs::read_to_string("./inputs/day24.txt").unwrap();
    //// split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    part1(input);
}

#[derive(PartialEq, Clone, Copy, PartialOrd, Debug)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

impl Point {
    // 18, 19, 22 @ -1, -1, -2
    fn from_line(line: &str) -> Self {
        let [x, y, z, vx, vy, vz] = line
            // split by " @ " and ", "
            .split(['@', ','])
            .map(|s| s.trim().parse::<f64>().unwrap_or_else(|_| panic!("hello_pattern {line}")))
            .collect::<Vec<f64>>()[..] else { panic!("hello {line}") };
        Self { x, y, z, vx, vy, vz }
    }

    /*
    two lines:
    a1x + b1y  = c1
    a2x + b2y  = c2

    px = (c1*b2 - c2*b1) / (a1*b2 - a2*b1)
    py = (a1*c2 - a2*c1) / (a1*b2 - a2*b1)


    Now, our lines are defined by point and vector:
    y = vy1/vx1 * (x - x1) + y1
    y = vy2/vx2 * (x - x2) + y2
    so, rewrite them to find a1, b1, c1, a2, b2, c2:
    y - y1 = vy1/vx1 * (x - x1) => y*vx1 - y1*vx1 = vy1*x - vy1*x1 => vy1*x - vx1*y = vy1*x1 - y1*vx1
    or a1 = vy1, b1 = -vx1, c1 = vy1*x1 - y1*vx1
    and a2 = vy2, b2 = -vx2, c2 = vy2*x2 - y2*vx2
     */
    fn find_intersect_xy_point(&self, other: &Self) -> Option<(f64, f64)> {
        let (x1, y1, x2, y2) = (self.x, self.y, other.x, other.y);
        let (vx1, vy1, vx2, vy2) = (self.vx, self.vy, other.vx, other.vy);
        // if they are parallel, vx1*vy2 == vx2*vy1
        let p = vx1 * vy2 - vx2 * vy1;
        if p == 0.0 {
            //println!("p == 0.0");
            return None;
        }
        /*
        or a1 = vy1, b1 = -vx1, c1 = vy1*x1 - y1*vx1
        and a2 = vy2, b2 = -vx2, c2 = vy2*x2 - y2*vx2
         */
        let a1 = vy1;
        let b1 = -vx1;
        let c1 = vy1 * x1 - y1 * vx1;
        let a2 = vy2;
        let b2 = -vx2;
        let c2 = vy2 * x2 - y2 * vx2;
        /*
        px = (c1*b2 - c2*b1) / (a1*b2 - a2*b1)
        py = (a1*c2 - a2*c1) / (a1*b2 - a2*b1)
         */
        let xx = c1 * b2 - c2 * b1;
        let yy = a1 * c2 - a2 * c1;
        let p = a1 * b2 - a2 * b1;
        // otherwise, they intersect
        let (px, py): (f64, f64) = (xx / p, yy / p);

        //println!("px: {}, py: {}", px, py);

        // check the vectors directions: (vx1, vy1) vs (px - x1, py - y1)
        // and (vx2, vy2) vs (px - x2, py - y2)
        // to determine if the intersection point in the future or in the past
        let (dx1, dy1) = (px - x1, py - y1);
        let (dx2, dy2) = (px - x2, py - y2);
        if vx1 * dx1 + vy1 * dy1 < 0.0
            || vx2 * dx2 + vy2 * dy2 < 0.0 {
            return None;
        }
        Some((px, py))
    }

    fn are_intersected_inside_xy_box(
        &self,
        other: &Self,
        from_x: f64,
        to_x: f64,
        from_y: f64,
        to_y: f64,
    ) -> (bool, Option<(f64, f64)>) {
        if let Some((x, y)) = self.find_intersect_xy_point(other) {
            (is_inside_xy_box(x, y, from_x, to_x, from_y, to_y), Some((x, y)))
        } else {
            (false, None)
        }
    }
}

fn is_inside_xy_box(
    x: f64,
    y: f64,
    from_x: f64,
    to_x: f64,
    from_y: f64,
    to_y: f64) -> bool {
    from_x <= x && x <= to_x && from_y <= y && y <= to_y
}

fn part1(lines: Vec<String>) {
    /*
        r###"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3"###;
     */
    let mut points: Vec<Point> = lines.iter()
        .map(|line| Point::from_line(line))
        .collect();
    let mut count = 0;
    //let from = 7.0;
    //let to = 27.0;
    let from = 200000000000000.0;
    let to = 400000000000000.0;
    let (from_x, to_x, from_y, to_y) = (from, to, from, to);
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            //println!("{i} {j} {:?} {:?}", points[i], points[j]);
            let (intersected, point) = points[i].are_intersected_inside_xy_box(
                &points[j],
                from_x,
                to_x,
                from_y,
                to_y,
            );
            if intersected {
                count += 1;
                //println!("intersected! {}: {:?}", count, point);
            } else {
                //println!("not intersected");
            }
        }
    }
    println!("part1: {}", count);
}

fn part2(lines: Vec<String>) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_intersection_point() {
        let p1 = Point::from_line("19, 13, 30 @ -2,  1, -2");
        let p2 = Point::from_line("18, 19, 22 @ -1, -1, -2");
        let (x, y) = p1.find_intersect_xy_point(&p2).unwrap();
        assert_eq!(x, 14.333333333333334);
        assert_eq!(y, 15.333333333333334);
    }
}