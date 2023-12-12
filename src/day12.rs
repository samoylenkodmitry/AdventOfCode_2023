use std::collections::{BinaryHeap, HashMap, HashSet};
use priority_queue::PriorityQueue;

pub(crate) fn day12() {
    /*
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
*/
    let example_lines =
        vec![
                "???.### 1,1,3",
              ".??..??...?##. 1,1,3",
              "?#?#?#?#?#?#?#? 1,3,1,6", // wrong
                "????.#...#... 4,1,1",
                "????.######..#####. 1,6,5",
                "?###???????? 3,2,1",
        ];
    // convert example lines to String
    let example_lines: Vec<String> =
        example_lines.iter().map(|s| s.to_string()).collect();

    //part1(example_lines);

    let input = std::fs::read_to_string("./inputs/day12.txt").unwrap();
    // split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    //part1(input);
    // part2
    let example_lines =
        vec![
            "???.### 1,1,3",
            ".??..??...?##. 1,1,3",
            "?#?#?#?#?#?#?#? 1,3,1,6", // wrong
            "????.#...#... 4,1,1",
            "????.######..#####. 1,6,5",
            "?###???????? 3,2,1",
        ];
    // convert example lines to String
    let example_lines: Vec<String> =
        example_lines.iter().map(|s| s.to_string()).collect();

    part2(example_lines);

    let input = std::fs::read_to_string("./inputs/day12.txt").unwrap();
    // split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    part2(input);
}

fn part1(lines: Vec<String>) {
    /*
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
*/
    // let's solve for each line
    let mut sum = 0;

    for line in lines.iter() {
        // split into pattern and sizes
        let mut parts = line.split(" ");
        let pattern = parts.next().unwrap();
        let sizes = parts.next().unwrap();
        let sizes: Vec<usize> = sizes.split(",").map(|s| s.parse::<usize>().unwrap()).collect();
        let pattern: Vec<char> = pattern.chars().collect();
        let curr_sum = solve(pattern, sizes);
        //println!("{}: {}", line, curr_sum);
        sum += curr_sum;
    }

    println!("part1: {}", sum);
}

fn solve(pattern: Vec<char>, sizes: Vec<usize>) -> usize {
    // there is a pattern and sizes
    // .??..??...?##. 1,1,3
    // each size - is a size of continuous #s separated by .
    // each ? can be either # or .
    // we need to find the number of ways to fill in the ?s
    // let's try to solve this recursively

    // each pattern position and size position will give us a number of ways to fill in the ?s
    // this can be memoized

    // ok, we also need some `canUse` variable, because if we right after #s block, we can't use #
    let mut memo: HashMap<(bool, (usize, usize)), usize> = HashMap::new();
    solve_rec(&pattern, &sizes, 0, 0, true, &mut memo)
}

fn solve_rec(pattern: &Vec<char>, sizes: &Vec<usize>,
             sz_pos: usize, pattern_pos: usize,
             can_use: bool,
             dp: &mut HashMap<(bool, (usize, usize)), usize>) -> usize {
    // there is a pattern and sizes
    //
    // pattern sizes
    // 0123456 0 1 2
    // ???.### 1,1,3
    //
    //           1111
    // 01234567890123 0 1 2
    // .??..??...?##. 1,1,3
    //
    //            11111
    //  012345678901234 0 1 2 3
    // "?#?#?#?#?#?#?#? 1,3,1,6",
    //
    // each size - is a size of continuous #s separated by .
    // each ? can be either # or .
    // we need to find the number of ways to fill in the ?s

    // key is (pos, pattern_pos)
    // if we have already solved this position, return the answer
    if dp.contains_key(&(can_use, (sz_pos, pattern_pos))) {
        //println!("found in memo {} {} {} res:{}", can_use, sz_pos, pattern_pos, dp[&(can_use, (sz_pos, pattern_pos))]);
        //println!("memo: {:?}", dp);
        return dp[&(can_use, (sz_pos, pattern_pos))];
    }

    // if we have reached the end of the pattern, and the end of the sizes, return 1
    if pattern_pos == pattern.len() && sz_pos == sizes.len() {
        //println!("found end: 1");
        return 1;
    }

    // if we have reached the end of the pattern, but not the end of the sizes, return 0
    if pattern_pos >= pattern.len() {
        //println!("found end: 0");
        return 0;
    }

    // if we have reached the end of the sizes, but not the end of the pattern, return 0
    if sz_pos >= sizes.len() {
        // we must check there is no #s left
        for i in pattern_pos..pattern.len() {
            if pattern[i] == '#' {
                dp.insert((can_use, (sz_pos, pattern_pos)), 0);
                //println!("found end: 0 sz={}, pp={}", sz_pos, pattern_pos);
                return 0;
            }
        }
        //println!("found end_: 1 sz={}, pp={}", sz_pos, pattern_pos);
        return 1;
    }

    // if we at `.`, then we can only move to the next position
    if pattern[pattern_pos] == '.' {
        let ans = solve_rec(pattern, sizes, sz_pos, pattern_pos + 1, true, dp);
        dp.insert((can_use, (sz_pos, pattern_pos)), ans);
        //println!("found .: {}, sz={}, pp={}", ans, sz_pos, pattern_pos);
        return ans;
    }


    let current_size = sizes[sz_pos];
    // if we at `#` we must start continuous block of #s of the current_size

    let curr_char = pattern[pattern_pos];

    if curr_char == '#' { // must use
        if !can_use {
            // if we can't use #, then return 0
            dp.insert((can_use, (sz_pos, pattern_pos)), 0);
            //println!("found # and !can_use: 0 sz={}, pp={}", sz_pos, pattern_pos);
            return 0;
        }

        // check for overflow
        if pattern_pos + current_size > pattern.len() {
            dp.insert((can_use, (sz_pos, pattern_pos)), 0);
            //println!("found # and overflow: 0 sz={}, pp={}", sz_pos, pattern_pos);
            return 0;
        }

        // check if block doesn't have any `.`s
        // if it has a dot, then return 0
        for i in pattern_pos..pattern_pos + current_size {
            if pattern[i] == '.' {
                // if it has a dot, then return 0
                dp.insert((can_use, (sz_pos, pattern_pos)), 0);
                //println!("found # and .: 0 sz={}, pp={}", sz_pos, pattern_pos);
                return 0;
            }
        }
        // also char next after block must not be `#`
        if pattern_pos + current_size < pattern.len() && pattern[pattern_pos + current_size] == '#' {
            dp.insert((can_use, (sz_pos, pattern_pos)), 0);
            //println!("found # and next #: 0 sz={}, pp={}", sz_pos, pattern_pos);
            return 0;
        }

        // otherwise, we can move to the next block
        let ans = solve_rec(pattern, sizes, sz_pos + 1, pattern_pos + current_size, false, dp);
        dp.insert((can_use, (sz_pos, pattern_pos)), ans);
        //println!("found # res: {} sz={} pp={}", ans, sz_pos, pattern_pos);
        return ans;
    }

    // now we are at `?`
    // we can either start a block of #s of the current_size or skip
    let mut can_use = can_use;
    // check for overflow
    if pattern_pos + current_size > pattern.len() {
        dp.insert((can_use, (sz_pos, pattern_pos)), 0);
        //println!("found ? and overflow: 0 sz={}, pp={}", sz_pos, pattern_pos);
        return 0;
    }
    // if we start a block we need to check if it doesn't have any `.`s
    for i in pattern_pos..pattern_pos + current_size {
        if pattern[i] == '.' {
            can_use = false;
            break;
        }
    }
    // also char next after block must not be `#`
    if pattern_pos + current_size < pattern.len() && pattern[pattern_pos + current_size] == '#' {
        can_use = false;
    }
    // otherwise, we can move to the next block
    let mut ans = 0;
    if can_use {
        ans += solve_rec(pattern, sizes, sz_pos + 1, pattern_pos + current_size, false, dp);
        //println!("found ? and can_use: {} sz={} pp={}", ans, sz_pos, pattern_pos);
    }
    // we can also skip this block
    ans += solve_rec(pattern, sizes, sz_pos, pattern_pos + 1, true, dp);
    //println!("found ? and skip: {} sz={} pp={} cu={}", ans, sz_pos, pattern_pos, can_use);

    dp.insert((can_use, (sz_pos, pattern_pos)), ans);

    ans
}

fn part2(lines: Vec<String>) {
    /*
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
*/
    // let's solve for each line
    let mut sum = 0;

    for line in lines.iter() {
        // split into pattern and sizes
        let mut parts = line.split(" ");
        let pattern = parts.next().unwrap();
        let sizes = parts.next().unwrap();
        let sizes: Vec<usize> = sizes.split(",").map(|s| s.parse::<usize>().unwrap()).collect();
        let pattern: Vec<char> = pattern.chars().collect();
        // for part 2 we need to reapeat patter 5 times inserting ? in between
        let mut pattern2 = Vec::new();
        for i in 0..5 {
            pattern2.extend(pattern.iter());
            if i < 4 {
                pattern2.push('?');
            }
        }
        // for part 2 we need to repeat sizes 5 times
        let mut sizes2 = Vec::new();
        for i in 0..5 {
            sizes2.extend(sizes.iter());
        }

        let curr_sum = solve(pattern2, sizes2);
        //println!("{}: {}", line, curr_sum);
        sum += curr_sum;
    }

    println!("part1: {}", sum);
}













