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

    //part1(example_lines);

    let input = std::fs::read_to_string("./inputs/day24.txt").unwrap();
    //// split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    //part1(input);

    // part 2
    let example_lines: Vec<&str> = raw_str.lines().collect();
    // convert example lines to String
    let example_lines: Vec<String> =
        example_lines.iter().map(|s| s.to_string()).collect();

    //part2(example_lines);

    let input = std::fs::read_to_string("./inputs/day24.txt").unwrap();
    //// split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    part2(input);
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

    fn position_at_time(&self, time: i32) -> (f64, f64, f64) {
        if time == 0 {
            return (self.x, self.y, self.z);
        }
        (
            self.x + self.vx * time as f64,
            self.y + self.vy * time as f64,
            self.z + self.vz * time as f64,
        )
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

fn part2(lines: Vec<String>) {
    // for part 2 we need to find
    // the starting point and vector of the line
    // so that it intersects with all other lines
    let points: Vec<Point> = lines.iter()
        .map(|line| Point::from_line(line))
        .collect();
    // sort points by (x and y and z)
    let mut points = points;
    points.sort_by(|a, b| {
        let x = a.x.partial_cmp(&b.x).unwrap();
        let y = a.y.partial_cmp(&b.y).unwrap();
        let z = a.z.partial_cmp(&b.z).unwrap();
        x.then(y).then(z)
    });
    // we must throw a stone from some point (Int, Int, Int)
    // with some speed (Int, Int, Int)
    // and intersect all the points
    //
    // if two points has the same X speed,
    // like this
    // A --->
    //        B --->
    // to shoot A then B with Int precision
    // our X speed must be: Bx - Ax % shoot_speed_x == 0
    // same for Y and Z
    // let's collect all possible X, Y, Z speeds
    // by analyzing all pairs of points with same X, Y or Z speeds

    // speed range will be -250..250
    let speed_range: i128 = 1000;
    let mut possible_x_speeds: HashSet<i128> = HashSet::new();
    let mut x_first = true;
    let mut possible_y_speeds: HashSet<i128> = HashSet::new();
    let mut y_first = true;
    let mut possible_z_speeds: HashSet<i128> = HashSet::new();
    let mut z_first = true;
    // for each pair of points
    for a in 0..points.len() {
        let pa = points[a];
        for b in 0..points.len() {
            if a == b {
                continue;
            }
            let pb = points[b];
            let mut current_possible_x_speeds: HashSet<i128> = HashSet::new();
            if pa.vx == pb.vx {
                let dx = (pb.x - pa.x) as i128;
                let vx = pa.vx as i128;
                for speed in -speed_range..=speed_range {
                    if speed != vx && dx % (speed - vx) == 0 {
                        current_possible_x_speeds.insert(speed);
                    }
                }
                // current set must intersect with previous set
                if x_first {
                    possible_x_speeds = current_possible_x_speeds;
                    x_first = false;
                } else {
                    possible_x_speeds = possible_x_speeds
                        .intersection(&current_possible_x_speeds)
                        .map(|&x| x)
                        .collect();
                }
            }
            // y
            let mut current_possible_y_speeds: HashSet<i128> = HashSet::new();
            if pa.vy == pb.vy {
                let dy = (pb.y - pa.y) as i128;
                let vy = pa.vy as i128;
                for speed in -speed_range..=speed_range {
                    if speed != vy && dy % (speed - vy) == 0 {
                        current_possible_y_speeds.insert(speed);
                    }
                }
                // current set must intersect with previous set
                if y_first {
                    possible_y_speeds = current_possible_y_speeds;
                    y_first = false;
                } else {
                    possible_y_speeds = possible_y_speeds
                        .intersection(&current_possible_y_speeds)
                        .map(|&x| x)
                        .collect();
                }
            }
            // z
            let mut current_possible_z_speeds: HashSet<i128> = HashSet::new();
            if pa.vz == pb.vz {
                let dz = (pb.z - pa.z) as i128;
                let vz = pa.vz as i128;
                for speed in -speed_range..=speed_range {
                    if speed != vz && dz % (speed - vz) == 0 {
                        current_possible_z_speeds.insert(speed);
                    }
                }
                // current set must intersect with previous set
                if z_first {
                    possible_z_speeds = current_possible_z_speeds;
                    z_first = false;
                } else {
                    possible_z_speeds = possible_z_speeds
                        .intersection(&current_possible_z_speeds)
                        .map(|&x| x)
                        .collect();
                }
            }
        }
    }
    //println!("possible_x_speeds: {:?}", possible_x_speeds.len());
    //println!("possible_y_speeds: {:?}", possible_y_speeds.len());
    //println!("possible_z_speeds: {:?}", possible_z_speeds.len());
    /*
    possible_x_speeds: 1
    possible_y_speeds: 1
    possible_z_speeds: 1
     */
    let vx = possible_x_speeds.iter().next().unwrap();
    let vy = possible_y_speeds.iter().next().unwrap();
    let vz = possible_z_speeds.iter().next().unwrap();
    let rvx:f64 = vx.clone() as f64;
    let rvy = vy.clone() as f64;
    let rvz = vz.clone() as f64;
    //println!("vx : {} rvx: {}", vx, rvx);
    // now calculate the starting point
    // I don't want to solve this, just copy-pasted from the other solution
    // I am sorry
    /*
    MA = (AVY-RVY)/(AVX-RVX)
    MB = (BVY-RVY)/(BVX-RVX)
    CA = APY - (MA*APX)
    CB = BPY - (MB*BPX)
    XPos = int((CB-CA)/(MA-MB))
    YPos = int(MA*XPos + CA)
    Time = (XPos - APX)//(AVX-RVX)
    ZPos = APZ + (AVZ - RVZ)*Time
     */
    let a = points[0];
    let avx = a.vx;
    let avy = a.vy;
    let avz = a.vz;
    let apx = a.x;
    let apy = a.y;
    let apz = a.z;

    let b = points[1];
    let bvx = b.vx;
    let bvy = b.vy;
    let bpx = b.x;
    let bpy = b.y;

    let ma = (avy - rvy) / (avx - rvx);
    let mb = (bvy - rvy) / (bvx - rvx);
    println!("ma: {}, mb: {}", ma, mb);

    let ca = apy - (ma * apx);
    let cb = bpy - (mb * bpx);
    println!("ca: {}, cb: {}", ca, cb);

    let x_pos = (cb - ca) / (ma - mb);
    let x_pos: i128 = x_pos as i128;
    let x_pos:f64 = x_pos as f64;

    let y_pos = ma * x_pos + ca;
    let y_pos: i128 = y_pos as i128;
    let y_pos:f64 = y_pos as f64;

    let time = (x_pos as i128 - apx as i128) / (avx as i128 - rvx as i128);

    let z_pos = apz as i128 + (avz - rvz)as i128 * time;

    let x_pos: i128 = x_pos as i128;
    let y_pos: i128 = y_pos as i128;
    let z_pos: i128 = z_pos as i128;
    println!("part2: {}", x_pos + y_pos + z_pos);
    //not the right answer; your answer is too low  597670936440402
    //not the right answer; your answer is too high 822086828221122
    //                                              741991571910536
}

fn almost_equal(
    p1: (f64, f64, f64),
    p2: (f64, f64, f64)) -> bool {
    let (x1, y1, z1) = p1;
    let (x2, y2, z2) = p2;
    let e = 10.;
    //            9999999999995
    //            0000000000146
    (x1 - x2).abs() <= e && (y1 - y2).abs() <= e && (z1 - z2).abs() <= e
}


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