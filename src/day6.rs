use std::collections::{HashMap, HashSet};

pub(crate) fn day6() {
    /*
Time:      7  15   30
Distance:  9  40  200
*/
    let example_lines =
        vec![
            "Time:      7  15   30",
            "Distance:  9  40  200",
        ];
    // convert example lines to String
    let example_lines: Vec<String> =
        example_lines.iter().map(|s| s.to_string()).collect();

    part1(example_lines);

    let input = std::fs::read_to_string("./inputs/day6.txt").unwrap();
    // split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    part1(input);
}

fn part1(lines: Vec<String>) {
    /*
Time:      7  15   30
Distance:  9  40  200
*/
    // the answer is simulation_count(7, 9) + simulation_count(15, 40) + simulation_count(30, 200)
    // let's pars pairs 7, 9; 15, 40; 30, 200
    let times: Vec<usize> = lines[0].split_whitespace().skip(1)
        .map(|s| s.parse::<usize>().unwrap()).collect();
    let distances: Vec<usize> = lines[1].split_whitespace().skip(1)
        .map(|s| s.parse::<usize>().unwrap()).collect();
    // let's calculate simulation_count(7, 9) + simulation_count(15, 40) + simulation_count(30, 200)
    let mut total = 1;
    for pair in times.iter().zip(distances.iter()) {
        total *= simulation_count(*pair.0, *pair.1)
    }
    println!("part1: {}", total);
}

fn simulation_count(time: usize, distance: usize) -> usize {
    let mut simulation_count = 0;
    // each second we can increase speed ot let go to travel with current speed
    // if after time passed we travel exactly distance, we increase simulation_count

    let mut speed = 0;
    // let's do dfs in time, each time we choose to increase speed or let go
    // if we letting go, then we know what the distance will be

    for t in 0..=time {
        // if we let go now, then
        let distance_traveled = speed * (time - t);
        if distance_traveled > distance {
            simulation_count += 1;
        }
        speed += 1;
    }

    simulation_count
}

fn part2(lines: Vec<String>) {

}

