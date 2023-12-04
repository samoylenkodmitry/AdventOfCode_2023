use std::collections::{HashMap, HashSet};

pub(crate) fn day4() {
    /*
            example lines are:
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
*/
    let example_lines =
        vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"];
    // convert example lines to String
    let example_lines: Vec<String> =
        example_lines.iter().map(|s| s.to_string()).collect();

    part1(example_lines);

    let input = std::fs::read_to_string("./inputs/day4.txt").unwrap();
    // split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    part1(input);

    // part 2
    let example_lines =
        vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"];
    // convert example lines to String
    let example_lines: Vec<String> =
        example_lines.iter().map(|s| s.to_string()).collect();

    part2(example_lines);

    let input = std::fs::read_to_string("./inputs/day4.txt").unwrap();
    // split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    part2(input);
}

fn part1(lines: Vec<String>) {
    /*
            example lines are:
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
*/
    // for each line
    // let's put winning numbers in a hashset
    // then check count of numbers after | that are also in set
    let mut sum = 0;
    for line in lines.iter() {
        // split the line
        let mut parts = line.split(" | ");
        // get winning numbers
        let winning_numbers_vec = parts.next().unwrap().split(": ").nth(1).unwrap()
            .split(" ")
            .filter(|&n| !n.trim().is_empty())
            .collect::<Vec<&str>>();
        let winning_numbers_set: HashSet<&str> = winning_numbers_vec.iter().cloned().collect();
        // count numbers after | that are in set
        // Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        let checked_numbers_vec = parts.next().unwrap().split(" ")
            .filter(|&n| !n.trim().is_empty())
            .collect::<Vec<&str>>();
        let intersect :Vec<&str>= checked_numbers_vec.iter()
            .filter(|&n| winning_numbers_set.contains(n)).cloned().collect();
        let count = intersect.len();
        if count > 0 {
            let add = 1 << (count - 1);
            sum += add;
        }
    }

    println!("part1: {}", sum);
}

fn part2(lines: Vec<String>) {

    /*
            example lines are:
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
*/
    // for each line
    // let's put winning numbers in a hashset
    // then check count of numbers after | that are also in set

    // let store how many times each card won
    // if card 1 have 4 matching numbers, then cards 1+1, 1+2, 1+3, and 1+4 added
    // cards_count[1] = 1
    // cards_count[2] += 1
    // cards_count[3] += 1
    // cards_count[4] += 1
    // cards_count[5] += 1
    // however, if we have x number of card 1, we must add not 1 but x, so
    // first add to curr: cards_count[curr] += 1
    // cards_count[curr..=curr+matching_numbers_count] += cards_count[curr]

    let mut cards_count: HashMap<usize, usize> = HashMap::new();
    let mut sum = 0;
    for (curr, line) in lines.iter().enumerate() {
        // split the line
        let mut parts = line.split(" | ");
        // get winning numbers
        let winning_numbers_vec = parts.next().unwrap().split(": ").nth(1).unwrap()
            .split(" ")
            .filter(|&n| !n.trim().is_empty())
            .collect::<Vec<&str>>();
        let winning_numbers_set: HashSet<&str> = winning_numbers_vec.iter().cloned().collect();
        // count numbers after | that are in set
        // Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        let checked_numbers_vec = parts.next().unwrap().split(" ")
            .filter(|&n| !n.trim().is_empty())
            .collect::<Vec<&str>>();
        let intersect :Vec<&str>= checked_numbers_vec.iter()
            .filter(|&n| winning_numbers_set.contains(n)).cloned().collect();
        let count = intersect.len();
        // add to curr
        *cards_count.entry(curr).or_insert(0) += 1;
        if count > 0 {
            // cards_count[curr..=curr+matching_numbers_count] += cards_count[curr]
            for i in curr+1..=curr+count {
                *cards_count.entry(i).or_insert(0) += cards_count[&curr];
            }
            // sum is how many cards we have total
        }
        sum += cards_count[&curr];
    }

    println!("part1: {}", sum);
}

