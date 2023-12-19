use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::i32;
use linked_hash_map::LinkedHashMap;
use priority_queue::PriorityQueue;

pub(crate) fn day19() {
    let raw_str =
        r###"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"###;
    let example_lines: Vec<&str> = raw_str.lines().collect();
    // convert example lines to String
    let example_lines: Vec<String> =
        example_lines.iter().map(|s| s.to_string()).collect();

    //part1(example_lines);

    let input = std::fs::read_to_string("./inputs/day19.txt").unwrap();
    //// split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    //part1(input);

    // part 2

    let example_lines: Vec<&str> = raw_str.lines().collect();
    // convert example lines to String
    let example_lines: Vec<String> =
        example_lines.iter().map(|s| s.to_string()).collect();

    part2(example_lines);

    let input = std::fs::read_to_string("./inputs/day19.txt").unwrap();
    //// split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    part2(input);
}

fn part1(lines: Vec<String>) {
    /*
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"###;
     */
    // parse rules until empty line reached
    // then parse parts and apply rules to each, starting with rule named 'in'

    let (rules, parts_to_analyze) = analyze_input(lines);

    let mut sum = 0;
    // apply rules to each part
    // start with rule named 'in'
    for (x, m, a, s) in parts_to_analyze {
        let rule_name = follow_the_rules(&rules, x, m, a, s);
        // if rule_name is 'A' we count it
        if rule_name == "A" {
            sum += x + m + a + s;
        }
    }

    println!("part1: {}", sum);
}

fn follow_the_rules(rules: &LinkedHashMap<String, Vec<(char, char, i32, String)>>, x: i32, m: i32, a: i32, s: i32) -> String {
    let mut rule_name = "in".to_string();
    // finish rule name is 'R' or 'A'
    while rule_name != "R" && rule_name != "A" {
        let rule_parts = rules.get(&rule_name).unwrap();
        for (var_name, comparison, value, rule_name_goto) in rule_parts {
            match var_name {
                'x' => {
                    if *comparison == '<' {
                        if x < *value {
                            rule_name = rule_name_goto.to_string();
                            break;
                        }
                    } else if *comparison == '>' {
                        if x > *value {
                            rule_name = rule_name_goto.to_string();
                            break;
                        }
                    } else {
                        panic!("unknown comparison");
                    }
                }
                'm' => {
                    if *comparison == '<' {
                        if m < *value {
                            rule_name = rule_name_goto.to_string();
                            break;
                        }
                    } else if *comparison == '>' {
                        if m > *value {
                            rule_name = rule_name_goto.to_string();
                            break;
                        }
                    } else {
                        panic!("unknown comparison");
                    }
                }
                'a' => {
                    if *comparison == '<' {
                        if a < *value {
                            rule_name = rule_name_goto.to_string();
                            break;
                        }
                    } else if *comparison == '>' {
                        if a > *value {
                            rule_name = rule_name_goto.to_string();
                            break;
                        }
                    } else {
                        panic!("unknown comparison");
                    }
                }
                's' => {
                    if *comparison == '<' {
                        if s < *value {
                            rule_name = rule_name_goto.to_string();
                            break;
                        }
                    } else if *comparison == '>' {
                        if s > *value {
                            rule_name = rule_name_goto.to_string();
                            break;
                        }
                    } else {
                        panic!("unknown comparison");
                    }
                }
                '.' => {
                    rule_name = rule_name_goto.to_string();
                    break;
                }
                _ => panic!("unknown var_name"),
            }
        }
    }
    rule_name
}

fn analyze_input(lines: Vec<String>) -> (LinkedHashMap<String, Vec<(char, char, i32, String)>>, Vec<(i32, i32, i32, i32)>) {
    let mut rules: LinkedHashMap<String, Vec<(char, char, i32, String)>> = LinkedHashMap::new();
    let mut parse_rules = true;
    let mut parts_to_analyze: Vec<(i32, i32, i32, i32)> = Vec::new();
    // parse rules
    for line in lines {
        if line == "" {
            parse_rules = false;
            continue;
        }
        if parse_rules {

            //px{a<2006:qkq,m>2090:A,rfg}
            let mut cleared = line.replace("}", "");
            let mut parts = cleared.split("{");
            let rule_name = parts.next().unwrap();
            let rule = parts.next().unwrap();
            // each rule has several parts, separated by ','
            // each rule part has:
            // variable name
            // comparison operator < or >
            // value
            // rule name to go to if this rule part is true
            // let's store rule parts as a Vec<(var_name, comparison, value, rule_name)>
            let mut rule_parts: Vec<(char, char, i32, String)> = Vec::new();
            for rule_part in rule.split(",") {
                // can have condition ':' or not
                if rule_part.contains(":") {
                    //a<2006:qkq
                    let mut rule_part_parts = rule_part.split(":");
                    let condition = rule_part_parts.next().unwrap();
                    let rule_name_goto = rule_part_parts.next().unwrap();
                    //condition can have < or >
                    // a<2006
                    if condition.contains("<") {
                        let mut condition_parts = condition.split("<");
                        let var_name = condition_parts.next().unwrap();
                        let value = condition_parts.next().unwrap().parse().unwrap();
                        let var_name_char = var_name.chars().next().unwrap();
                        rule_parts.push((var_name_char, '<', value, rule_name_goto.to_string()));
                    } else if condition.contains(">") {
                        let mut condition_parts = condition.split(">");
                        let var_name = condition_parts.next().unwrap();
                        let value = condition_parts.next().unwrap().parse().unwrap();
                        let var_name_char = var_name.chars().next().unwrap();
                        rule_parts.push((var_name_char, '>', value, rule_name_goto.to_string()));
                    } else {
                        panic!("condition doesn't contain < or >");
                    }
                } else { // unconditional rule, like: 'A'
                    rule_parts.push(('.', '.', 0, rule_part.to_string()));
                }
            }
            rules.insert(rule_name.to_string(), rule_parts);
        } else {
            // parse parts
            //{x=2461,m=1339,a=466,s=291}
            // each part always has x=, m=, a=, s=
            // let's store parts as a Vec<(x, m, a, s)>
            let mut part = (0, 0, 0, 0);
            // let's parse string with pattern: {x=*,m=*,a=*,s=*}
            let cleared_1 = line.replace("}", "");
            let cleared_2 = cleared_1.replace("{", "");
            let mut parts = cleared_2.split(",");
            for part_str in parts {
                let mut part_str_parts = part_str.split("=");
                let var_name = part_str_parts.next().unwrap();
                let value_str = part_str_parts.next().unwrap();
                let value = value_str.parse().unwrap();
                match var_name {
                    "x" => part.0 = value,
                    "m" => part.1 = value,
                    "a" => part.2 = value,
                    "s" => part.3 = value,
                    _ => panic!("unknown var_name"),
                }
            }
            parts_to_analyze.push(part);
        }
    }
    (rules, parts_to_analyze)
}

fn part2(lines: Vec<String>) {
    let (rules, _) = analyze_input(lines);
    // now each variable of x, m, a, s is in range 1..=4000
    // for each combination of x, m, a, s we need to find
    // the number of rules that will lead to 'A'
    /*
    px{a<2006:qkq,m>2090:A,rfg}
    pv{a>1716:R,A}
    lnx{m>1548:A,A}
    rfg{s<537:gd,x>2440:R,A}
    qs{s>3448:A,lnx}
    qkq{x<1416:A,crn}
    crn{x>2662:A,R}
    in{s<1351:px,qqz}
    qqz{s>2770:qs,m<1801:hdj,R}
    gd{a>3333:R,R}
    hdj{m>838:A,pv}
     */
    // we can go from 'in' and do BFS
    // on the path we need to track the ranges for x, m, a, s
    // if we reach 'A' we count it
    // for example
    //
    // in{s<1351:px,qqz}
    // from in we go to px (and set s<1351) and to qqz (and set s>=1351)
    //
    // px{a<2006:qkq,m>2090:A,rfg}
    // from px we go to qkq (and set a<2006), to A (and set m>=2090),
    // to rfg (and set m<2090 and a>=2006)
    // as we reach A, we analyze the ranges:
    // x: 1..=4000
    // m: 2090..=4000
    // a: 2006..=4000
    // s: 1351..=4000
    // this gives us 4000*1911*1995*2650 = 40411917000000
    // and we continue BFS
    // we must track the ranges for each variable
    // and we must track the rules that we have already visited (name, ranges)
    // each range is a pair (min, max), so we need to track 4 pairs
    // queue will contain (rule_name, x_range, m_range, a_range, s_range)
    // we start with (in, (1, 4000), (1, 4000), (1, 4000), (1, 4000))

    let mut queue: Vec<(String, (i32, i32), (i32, i32), (i32, i32), (i32, i32))> = Vec::new();
    queue.push(("in".to_string(), (1, 4000), (1, 4000), (1, 4000), (1, 4000)));
    let mut visited: HashSet<(String, (i32, i32), (i32, i32), (i32, i32), (i32, i32))> = HashSet::new();
    let mut count:u128 = 0;
    while let Some((rule_name, x_range, m_range, a_range, s_range)) = queue.pop() {
        if !visited.insert((rule_name.clone(), x_range, m_range, a_range, s_range)) {
            continue;
        }
        if rule_name == "A" {
            // we reached A, count it
            let x_count = x_range.1 - x_range.0 + 1;
            let m_count = m_range.1 - m_range.0 + 1;
            let a_count = a_range.1 - a_range.0 + 1;
            let s_count = s_range.1 - s_range.0 + 1;
            let to_add = (x_count as u128) * (m_count as u128) * (a_count as u128) * (s_count as u128);
            //println!("x_range: {:?}, m_range: {:?}, a_range: {:?}, s_range: {:?}, to_add {}",
            //         x_range, m_range, a_range, s_range, to_add);
            count += to_add;

            continue;
        }
        if rule_name == "R" {
            continue;
        }
        // if any range is collapsed, continue
        if x_range.0 > x_range.1 || m_range.0 > m_range.1 || a_range.0 > a_range.1 || s_range.0 > s_range.1 {
            continue;
        }
        // copy ranges
        let mut x_range = x_range.clone();
        let mut m_range = m_range.clone();
        let mut a_range = a_range.clone();
        let mut s_range = s_range.clone();
        // traverse next rules
        //rules: LinkedHashMap<String, Vec<(char, char, i32, String)>>
        let rule_parts = rules.get(&rule_name).unwrap();
        for (var_name, comparison, value, rule_name_goto) in rule_parts {
            // if any range is collapsed, continue
            if x_range.0 > x_range.1 || m_range.0 > m_range.1 || a_range.0 > a_range.1 || s_range.0 > s_range.1 {
                continue;
            }
            match var_name {
                '.' => {
                    queue.push((rule_name_goto.to_string(), x_range, m_range, a_range, s_range));
                }
                'x' => {
                    if *comparison == '<' {
                        // compare value with range
                        // if it is intersecting, we need to update the range
                        if value >= &x_range.0 {
                            let mut x_range_clone = x_range.clone();
                            x_range_clone.1 = min(value - 1, x_range.1);
                            queue.push((rule_name_goto.to_string(), x_range_clone, m_range, a_range, s_range));
                        } // else skip
                        // the next rule_part will be '>=', so we need to update the range
                        x_range.0 = max(*value, x_range.0);
                    } else { // otherwise '>'
                        if value <= &x_range.1 {
                            let mut x_range_clone = x_range.clone();
                            x_range_clone.0 = max(value + 1, x_range.0);
                            queue.push((rule_name_goto.to_string(), x_range_clone, m_range, a_range, s_range));
                        } // else skip
                        // the next rule_part will be '<=', so we need to update the range
                        x_range.1 = min(*value, x_range.1);
                    }
                }
                'm' => {
                    if *comparison == '<' {
                        // compare value with range
                        // if it is intersecting, we need to update the range
                        if value >= &m_range.0 {
                            let mut m_range_clone = m_range.clone();
                            m_range_clone.1 = min(value - 1, m_range.1);
                            queue.push((rule_name_goto.to_string(), x_range, m_range_clone, a_range, s_range));
                        } // else skip
                        // the next rule_part will be '>=', so we need to update the range
                        m_range.0 = max(*value, m_range.0);
                    } else { // otherwise '>'
                        if value <= &m_range.1 {
                            let mut m_range_clone = m_range.clone();
                            m_range_clone.0 = max(value + 1, m_range.0);
                            queue.push((rule_name_goto.to_string(), x_range, m_range_clone, a_range, s_range));
                        } // else skip
                        // the next rule_part will be '<=', so we need to update the range
                        m_range.1 = min(*value, m_range.1);
                    }
                }
                'a' => {
                    if *comparison == '<' {
                        // compare value with range
                        // if it is intersecting, we need to update the range
                        if value >= &a_range.0 {
                            let mut a_range_clone = a_range.clone();
                            a_range_clone.1 = min(value - 1, a_range.1);
                            queue.push((rule_name_goto.to_string(), x_range, m_range, a_range_clone, s_range));
                        } // else skip
                        // the next rule_part will be '>=', so we need to update the range
                        a_range.0 = max(*value, a_range.0);
                    } else { // otherwise '>'
                        if value <= &a_range.1 {
                            let mut a_range_clone = a_range.clone();
                            a_range_clone.0 = max(value + 1, a_range.0);
                            queue.push((rule_name_goto.to_string(), x_range, m_range, a_range_clone, s_range));
                        } // else skip
                        // the next rule_part will be '<=', so we need to update the range
                        a_range.1 = min(*value, a_range.1);
                    }
                }
                's' => {
                    if *comparison == '<' {
                        // compare value with range
                        // if it is intersecting, we need to update the range
                        if value >= &s_range.0 {
                            let mut s_range_clone = s_range.clone();
                            s_range_clone.1 = min(value - 1, s_range.1);
                            queue.push((rule_name_goto.to_string(), x_range, m_range, a_range, s_range_clone));
                        } // else skip
                        // the next rule_part will be '>=', so we need to update the range
                        s_range.0 = max(*value, s_range.0);
                    } else { // otherwise '>'
                        if value <= &s_range.1 {
                            let mut s_range_clone = s_range.clone();
                            s_range_clone.0 = max(value + 1, s_range.0);
                            queue.push((rule_name_goto.to_string(), x_range, m_range, a_range, s_range_clone));
                        } // else skip
                        // the next rule_part will be '<=', so we need to update the range
                        s_range.1 = min(*value, s_range.1);
                    }
                }

                _ => panic!("unknown var_name"),
            }
        }



    }
    println!("part2: {}", count);
}

