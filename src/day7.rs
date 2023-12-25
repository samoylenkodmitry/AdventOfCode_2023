#![feature(iter_next_chunk)]

use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::{i32, mem};
use itertools::Itertools;
use linked_hash_map::LinkedHashMap;
use priority_queue::PriorityQueue;
use rustc_hash::FxHashSet;

pub(crate) fn day7() {
    let raw_str =
        r###"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"###;
    let example_lines: Vec<&str> = raw_str.lines().collect();
    // convert example lines to String
    let example_lines: Vec<String> =
        example_lines.iter().map(|s| s.to_string()).collect();

    //part1(example_lines);

    let input = std::fs::read_to_string("./inputs/day7.txt").unwrap();
    //// split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    //part1(input);

    // part 2

    let example_lines: Vec<&str> = raw_str.lines().collect();
    // convert example lines to String
    let example_lines: Vec<String> =
        example_lines.iter().map(|s| s.to_string()).collect();

    part2(example_lines);

    let input = std::fs::read_to_string("./inputs/day7.txt").unwrap();
    //// split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    part2(input);
}


fn part1(lines: Vec<String>) {
    /*
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
     */
    // we need to sort those hands by their rank
    // then result will be 1*rank1_num + 2*rank2_num + 3*rank3_num + 4*rank4_num + 5*rank5_num
    let hands: Vec<(String, u64)> = lines.iter().map(|s| {
        let mut iter = s.split_whitespace();
        let hand = iter.next().unwrap();
        let rank = iter.next().unwrap().parse().unwrap();
        (hand.to_string(), rank)
    }).collect();

    let ranks = hands.iter().map(|(hand, _)| {
        let mut groups: HashMap<char, Vec<char>> = HashMap::new();
        for c in hand.chars() {
            let entry = groups.entry(c).or_insert(vec![]);
            entry.push(c);
        }
        //println!(" hand: {}, groups: {:?}", hand, groups);
        let groups_sizes = groups.values().map(|v| v.len()).collect::<Vec<usize>>();
        let sorted_groups_sizes = groups_sizes.iter().sorted().collect::<Vec<&usize>>();
        //println!(" hand: {}, r: {}, sorted_groups_sizes: {:?}, group: {:?}", hand, r, sorted_groups_sizes, groups);
        match sorted_groups_sizes.as_slice() {
            [5] => 6,             // 0 Five of a kind, where all five cards have the same label: AAAAA
            [1, 4] => 5,          // 1 Four of a kind, where four cards have the same label and one card has a different label: AA8AA
            [2, 3] => 4,          // 2 Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
            [1, 1, 3] => 3,       // 3 Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
            [1, 2, 2] => 2,       // 4 Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
            [1, 1, 1, 2] => 1,    // 5 One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
            [1, 1, 1, 1, 1] => 0, // 6 High card, where all cards' labels are distinct: 23456
            _ => panic!("invalid hand {}", hand)
        }
    }).collect::<Vec<i32>>();

    let mut indices = (0..hands.len()).collect::<Vec<usize>>();
    // now sort hands by their rank, then by lexicographical order
    let lex_order = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];

    indices.sort_by(|&i, &j| {
        let (hand1, _) = &hands[i];
        let (hand2, _) = &hands[j];
        let rank1 = ranks[i];
        let rank2 = ranks[j];
        let rank_cmp = rank1.cmp(&rank2);
        if rank_cmp == std::cmp::Ordering::Equal {
            let mut iter1 = hand1.chars();
            let mut iter2 = hand2.chars();
            loop {
                let c1 = iter1.next().unwrap();
                let c2 = iter2.next().unwrap();
                let c1_idx = lex_order.iter().position(|&c| c == c1).unwrap();
                let c2_idx = lex_order.iter().position(|&c| c == c2).unwrap();
                let c_cmp = c1_idx.cmp(&c2_idx);
                if c_cmp != std::cmp::Ordering::Equal {
                    return c_cmp;
                }
            }
        } else {
            rank_cmp
        }
    });

    let result = indices.iter().enumerate().map(|(i, &idx)| {
        let (hand, num) = &hands[idx];
        let rank = ranks[idx];
        //println!(" hand: {}, num: {}, rank: {}", hand, num, rank);
        (i + 1) as u64 * num
    }).sum::<u64>();

    println!("part1: {}", result);
}

fn part2(lines: Vec<String>) {
    // for part 2 Joker now is a wildcard
    let hands: Vec<(String, u64)> = lines.iter().map(|s| {
        let mut iter = s.split_whitespace();
        let hand = iter.next().unwrap();
        let rank = iter.next().unwrap().parse().unwrap();
        (hand.to_string(), rank)
    }).collect();

    let ranks = hands.iter().map(|(hand, _)| {
        let mut groups: HashMap<char, Vec<char>> = HashMap::new();
        let mut jokers_count = 0;
        for c in hand.chars() {
            if c == 'J' {
                jokers_count += 1;
                continue;
            }
            let entry = groups.entry(c).or_insert(vec![]);
            entry.push(c);
        }
        //println!(" hand: {}, groups: {:?}", hand, groups);
        let groups_sizes = groups.values().map(|v| v.len()).collect::<Vec<usize>>();
        let mut sorted_groups_sizes = groups_sizes.iter().sorted().collect::<Vec<&usize>>();
        //println!(" hand: {}, r: {}, sorted_groups_sizes: {:?}, group: {:?}", hand, r, sorted_groups_sizes, groups);
        // so we need to improve rank by adding joker in a way to maximize rank
        // add joker to the group with the largest size
        if sorted_groups_sizes.len() > 0 {
            let ind = sorted_groups_sizes.len() - 1;
            jokers_count += sorted_groups_sizes[ind];
            sorted_groups_sizes[ind] = &jokers_count;
        } else {
            sorted_groups_sizes.push(&jokers_count);
        }
        match sorted_groups_sizes.as_slice() {
            [5] => 6,             // 0 Five of a kind, where all five cards have the same label: AAAAA
            [1, 4] => 5,          // 1 Four of a kind, where four cards have the same label and one card has a different label: AA8AA
            [2, 3] => 4,          // 2 Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
            [1, 1, 3] => 3,       // 3 Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
            [1, 2, 2] => 2,       // 4 Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
            [1, 1, 1, 2] => 1,    // 5 One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
            [1, 1, 1, 1, 1] => 0, // 6 High card, where all cards' labels are distinct: 23456
            _ => panic!("invalid hand {}", hand)
        }
    }).collect::<Vec<i32>>();

    let mut indices = (0..hands.len()).collect::<Vec<usize>>();
    // now sort hands by their rank, then by lexicographical order
    let lex_order = ['J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'];

    indices.sort_by(|&i, &j| {
        let (hand1, _) = &hands[i];
        let (hand2, _) = &hands[j];
        let rank1 = ranks[i];
        let rank2 = ranks[j];
        let rank_cmp = rank1.cmp(&rank2);
        if rank_cmp == std::cmp::Ordering::Equal {
            let mut iter1 = hand1.chars();
            let mut iter2 = hand2.chars();
            loop {
                let c1 = iter1.next().unwrap();
                let c2 = iter2.next().unwrap();
                let c1_idx = lex_order.iter().position(|&c| c == c1).unwrap();
                let c2_idx = lex_order.iter().position(|&c| c == c2).unwrap();
                let c_cmp = c1_idx.cmp(&c2_idx);
                if c_cmp != std::cmp::Ordering::Equal {
                    return c_cmp;
                }
            }
        } else {
            rank_cmp
        }
    });

    let result = indices.iter().enumerate().map(|(i, &idx)| {
        let (hand, num) = &hands[idx];
        let rank = ranks[idx];
        //println!(" hand: {}, num: {}, rank: {}", hand, num, rank);
        (i + 1) as u64 * num
    }).sum::<u64>();

    println!("part2: {}", result);
}