use std::collections::HashMap;

pub(crate) fn day3() {
    /*
            example lines are:
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
*/
    let example_lines =
        vec![
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598.."];
    // convert example lines to String
    let example_lines: Vec<String> =
        example_lines.iter().map(|s| s.to_string()).collect();

    part1(example_lines);

    let input = std::fs::read_to_string("./inputs/day3.txt").unwrap();
    // split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    part1(input);

    let example_lines =
        vec![
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598.."];
    // convert example lines to String
    let example_lines: Vec<String> =
        example_lines.iter().map(|s| s.to_string()).collect();

    part2(example_lines);

    let input = std::fs::read_to_string("./inputs/day3.txt").unwrap();
    // split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    part2(input);
}

fn part1(lines: Vec<String>) {
    /*
            example lines are:
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..

find all numbers adjacent to symbols
we can scan each line
if we found a number, scan it all
we need to keep track of the previous symbols positions
and previous numbers positions
for example
0123456789
..35..633.
number 35 must check previous positions 1, 2, 3, 4
and current positions 1, 4 (that just symbol before number and after number)
and if no symbols found, we must place number to check in the next row

*/
    // to simplify the things, let's find all symbols positions first
    let mut symbols_positions: Vec<(i32, i32)> = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' && !c.is_digit(10) {
                symbols_positions.push((x as i32, y as i32));
            }
        }
    }

    // ok, now we can safely find each number
    // and check if position is adjacent
    let mut sum = 0;
    for (y, line) in lines.iter().enumerate() {
        let mut x = 0;
        while x < line.len() {
            let c = line.chars().nth(x).unwrap();
            if c.is_digit(10) {
                // scan number until it ends
                // and check if it's adjacent to any symbol
                let mut xi = x as i32;
                let yi = y as i32;
                let mut is_adjacent =
                    symbols_positions.contains(&(xi - 1, yi)) ||
                        symbols_positions.contains(&(xi - 1, yi - 1)) ||
                        symbols_positions.contains(&(xi - 1, yi + 1)) ||
                        symbols_positions.contains(&(xi + 1, yi)) ||
                        symbols_positions.contains(&(xi + 1, yi - 1)) ||
                        symbols_positions.contains(&(xi + 1, yi + 1)) ||
                        symbols_positions.contains(&(xi, yi - 1)) ||
                        symbols_positions.contains(&(xi, yi + 1));
                let mut number = 0;
                while x < line.len() {
                    let c = line.chars().nth(x).unwrap();
                    if c.is_digit(10) {
                        number = number * 10 + c.to_digit(10).unwrap();
                        xi = x as i32;
                        is_adjacent = is_adjacent ||
                            // we can extract function later
                            symbols_positions.contains(&(xi - 1, yi)) ||
                            symbols_positions.contains(&(xi - 1, yi - 1)) ||
                            symbols_positions.contains(&(xi - 1, yi + 1)) ||
                            symbols_positions.contains(&(xi + 1, yi)) ||
                            symbols_positions.contains(&(xi + 1, yi - 1)) ||
                            symbols_positions.contains(&(xi + 1, yi + 1)) ||
                            symbols_positions.contains(&(xi, yi - 1)) ||
                            symbols_positions.contains(&(xi, yi + 1));
                    } else {
                        break;
                    }
                    x += 1;
                }
                if is_adjacent {
                    sum += number;
                }
            } else {
                x += 1;
            }
        }
    }

    println!("sum part 1 = {}", sum);
}

fn part2(lines: Vec<String>) {
    /*
        example lines are:
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..

we need to find all *-symbols
then find all numbers adjacent to *-symbols
and put them into each *-symbol group
then take only groups with 2 or more numbers
and find sum(multiply(group))
*/
    let mut gear_positions: Vec<(i32, i32)> = Vec::new();
    // let create a map from (each position adjacent to *-symbol) to (*-symbol position)
    let mut gear_adjacent_positions: HashMap<(i32, i32), Vec<(i32, i32)>>
        = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '*' {
                gear_positions.push((x as i32, y as i32));
                let mut xis = x as i32;
                let mut yis = y as i32;
                for xi in xis - 1..=xis + 1 {
                    for yi in yis - 1..=yis + 1 {
                        if xi >= 0 && yi >= 0 {
                            let mut adjacent_positions =
                                gear_adjacent_positions
                                    .entry((xi, yi))
                                    .or_insert(Vec::new());
                            adjacent_positions.push((x as i32, y as i32));
                        }
                    }
                }
            }
        }
    }

    // ok, now we can safely find each number
    // and check if position is adjacent
    // if so, put into HashMap
    let mut gear_numbers: HashMap<(i32, i32), Vec<u32>>
        = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        let mut x = 0;
        while x < line.len() {
            let c = line.chars().nth(x).unwrap();
            if c.is_digit(10) {
                // scan number until it ends
                // find which *-symbol it's adjacent to
                let mut ax = x as i32;
                let mut ay = y as i32;
                let mut is_adjacent_to: Vec<(i32, i32)> = Vec::new();

                let mut number = 0;
                while x < line.len() {
                    let c = line.chars().nth(x).unwrap();
                    if c.is_digit(10) {
                        number = number * 10 + c.to_digit(10).unwrap();
                        ax = x as i32;
                        add_all_gears_adjacent(
                            &mut gear_adjacent_positions,
                            &mut ax,
                            &mut ay,
                            &mut is_adjacent_to);
                    } else {
                        break;
                    }
                    x += 1;
                }
                // filter duplicates from is_adjacent_to
                is_adjacent_to.sort();
                is_adjacent_to.dedup();
                // now add to gear_numbers
                for gear_position in is_adjacent_to {
                    let mut numbers = gear_numbers
                        .entry(gear_position)
                        .or_insert(Vec::new());
                    numbers.push(number)
                }
            } else {
                x += 1;
            }
        }
    }
    // now find sum(multiply(group)) for each group where group.len() >= 2
    let mut sum:u128 = 0;
    for (gear_position, numbers) in gear_numbers {
        // convert numbers to set
        let mut numbers: Vec<u32> = numbers.iter().cloned().collect();
        numbers.sort();
        numbers.dedup();
        if numbers.len() >= 2 {
            let mut multiply:u128 = 1;
            for number in numbers {
                multiply *= number as u128;
            }
            sum += multiply;
        }
    }

    println!("sum part 2 = {}", sum);
}

fn add_all_gears_adjacent(gear_adjacent_positions: &mut HashMap<(i32, i32), Vec<(i32, i32)>>, ax: &mut i32, ay: &mut i32, is_adjacent_to: &mut Vec<(i32, i32)>) {
    if let Some(gear_positions) =
        gear_adjacent_positions.get(&(*ax, *ay)) {
        is_adjacent_to.extend(gear_positions);
    }
}


