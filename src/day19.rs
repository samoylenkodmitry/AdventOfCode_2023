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

    part1(example_lines);

    let input = std::fs::read_to_string("./inputs/day19.txt").unwrap();
    //// split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    part1(input);

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

    let mut sum = 0;
    // apply rules to each part
    // start with rule named 'in'
    for (x, m, a, s) in parts_to_analyze {
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
                    },
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
                    },
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
                    },
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
                    },
                    '.' => {
                        rule_name = rule_name_goto.to_string();
                        break;
                    },
                    _ => panic!("unknown var_name"),
                }
            }
        }
        // if rule_name is 'A' we count it
        if rule_name == "A" {
            sum += x + m + a + s;
        }
    }

    println!("part1: {}", sum);
}

fn part2(lines: Vec<String>) {

}

