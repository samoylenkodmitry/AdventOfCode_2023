use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::i32;
use linked_hash_map::LinkedHashMap;
use priority_queue::PriorityQueue;

pub(crate) fn day20() {
    let raw_str =
        r###"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"###;
    let example_lines: Vec<&str> = raw_str.lines().collect();
    // convert example lines to String
    let example_lines: Vec<String> =
        example_lines.iter().map(|s| s.to_string()).collect();

    //part1(example_lines);

    let input = std::fs::read_to_string("./inputs/day20.txt").unwrap();
    //// split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    //part1(input);

    // part 2
    let example_lines: Vec<&str> = raw_str.lines().collect();
    // convert example lines to String
    let example_lines: Vec<String> =
        example_lines.iter().map(|s| s.to_string()).collect();

    //part1(example_lines);

    let input = std::fs::read_to_string("./inputs/day20.txt").unwrap();
    //// split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    part2(input);
}

fn part1(lines: Vec<String>) {
    /*
    there are 2 types of modules:
    % - flip-flop, 'high' signal ignored, 'low' signal make flip the state:
         initial: 'off' (0)
         if state was 'off' then it becomes 'on' and vice versa
         off->on switch generates 'high' signal
         on->off switch generates 'low' signal
    & - conjunction, remembers all inputs signals
        initial: 'low' (0)
        all 'high' generates 'low' signal, otherwise 'high'
    broadcaster - sends signal from input to all outputs
        pressing the button sends 'low' to broadcaster

    all states are saved between presses
     */

    // our task is to count all 'high' and 'low' signals after 1000 presses

    // we need to create a graph of modules
    // for each conj modules we need to track all it's inputs
    // let's store type as char, outputs as Vec<String>
    // graph: HashMap<String, Vec<String>>
    // separate map for conj inputs memory, each conj module has it's own memory [(input_name -> input_value)]
    // let 'high' be 1, 'low' be 0
    // conj_memories: HashMap<String, HashMap<String, i32>>
    // we also need a memory for flip-flop modules
    // flip_flop_memories: HashMap<String, i32>
    // then we will do a BFS from broadcaster to all outputs
    // we will store the (signal, module_name) in a queue

    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    let mut conj_memories: HashMap<String, HashMap<String, i32>> = HashMap::new();
    let mut flip_flop_memories: HashMap<String, i32> = HashMap::new();
    let mut module_to_type: HashMap<String, char> = HashMap::new();
    /*
        r###"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"###;
     */
    for line in lines {
        let mut parts = line.split(" -> ");
        let from = parts.next().unwrap();
        let toos = parts.next().unwrap();
        let toos: Vec<String> = toos.split(", ").map(|s| s.to_string()).collect();
        let type_name = from.chars().next().unwrap();
        let from = from.chars().skip(1).collect::<String>();
        graph.insert(from.clone(), toos.clone());
        module_to_type.insert(from.clone(), type_name);
        if type_name == '&' {
            conj_memories.insert(from.clone(), HashMap::new());
        } else if type_name == '%' {
            flip_flop_memories.insert(from.clone(), 0); // insert as 'off'
        }
    }
    // now let's connect conj_memories
    for (from, toos) in &graph {
        // each signal in 'toos' that goes to conj module should be remembered
        for to in toos {
            /*
                r###"broadcaster -> a
        %a -> inv, con
        &inv -> b
        %b -> con
        &con -> output"###;
        // from b -> con
        // and from a -> con
        // so fill memories for con with values (a -> 0, b -> 0)
             */
            if graph.contains_key(&to.clone()) {
                //conj_memories: HashMap<String, HashMap<String, i32>>
                // check 'to' type
                let to_type = module_to_type.get(to).unwrap();
                if *to_type == '&' {
                    // this is a conj module
                    let memory = conj_memories.get_mut(to).unwrap();
                    memory.insert(from.clone(), 0);
                }
            }
        }
    }
    //println!("graph: {:?}", graph);

    let mut queue: Vec<(i32, String, String)> = Vec::new();
    let mut total_high_count = 0;
    let mut total_low_count = 0;
    let button_presses = 1000;
    for button_press in 1..=button_presses {
        let mut high_count = 0;
        let mut low_count = 0;
        queue.push((0, "roadcaster".to_string(), "".to_string()));
        while let Some((signal, module_name, from_name)) = queue.pop() {
            //println!("queue: {:?}", queue);
            //println!("signal: {}, module_name: {}, from_name: {}", signal, module_name, from_name);
            //println!("memories: {:?}", conj_memories);
            //println!("flip_flop_memories: {:?}", flip_flop_memories);
            if signal == 1 {
                high_count += 1;
            } else {
                low_count += 1;
            }

            let Some(type_name) = module_to_type.get(&module_name) else {
                continue;
            };
            //%a -> inv, con
            // first we need to process the current signal with current module
            // then we need to send processed signal to all outputs
            let mut out_signal = 0;
            match *type_name {
                '%' => {
                    /*
            % - flip-flop, 'high' signal ignored, 'low' signal make flip the state:
                initial: 'off' (0)
            if state was 'off' then it becomes 'on' and vice versa
            off->on switch generates 'high' signal
            on->off switch generates 'low' signal
             */
                    // if signal is 'high' then ignore it
                    if signal == 1 {
                        //println!("signal is 'high' then ignore it");
                        continue;
                    }
                    // if signal is 'low' then flip the state
                    let state = flip_flop_memories.get_mut(&module_name).unwrap();
                    if *state == 0 {
                        *state = 1;
                        out_signal = 1;
                    } else {
                        *state = 0;
                        out_signal = 0;
                    }
                    //println!("out_signal: {}", out_signal)
                }
                '&' => {
                    /*
            & - conjunction, remembers all inputs signals
                initial: 'low' (0)
                all 'high' generates 'low'(0) signal, otherwise 'high'(1)
             */
                    // this module must be processed automatically
                    // this is a conj module
                    let memory = conj_memories.get_mut(&module_name).unwrap();
                    memory.insert(from_name.clone(), signal);
                    //println!("new memory: {:?}", memory);
                    // now we need to check if all inputs are 'high'
                    let mut all_high = true;
                    for (_, value) in memory {
                        if *value == 0 {
                            all_high = false;
                            break;
                        }
                    }
                    out_signal = if all_high { 0 } else { 1 };
                }
                'b' => {
                    // send the same signal
                    out_signal = signal;
                }
                _ => panic!("unknown type_name: {}", type_name)
            }
            // send out_signal to all outputs
            if let Some(outputs) = graph.get(&module_name.clone()) {
                if *type_name == '&' {
                    for output in outputs {
                        // insert at front of queue
                        //println!("front of queue {} -{}- -> {}", module_name, out_signal, output);
                        queue.insert(0, (out_signal, output.clone(), module_name.clone()));
                    }
                } else {
                    for output in outputs {
                        //println!("back of queue {} -{}- -> {}", module_name, out_signal, output);
                        queue.push((out_signal, output.clone(), module_name.clone()));
                    }
                }
            }
        }
        total_high_count += high_count;
        total_low_count += low_count;
        println!("{} h={}, l={}", button_press, high_count, low_count);
    }

    println!("total: h= {}, l={}", total_high_count, total_low_count);
    println!("part1: {}", total_high_count * total_low_count);
}

fn part2(lines: Vec<String>) {
    /*
there are 2 types of modules:
% - flip-flop, 'high' signal ignored, 'low' signal make flip the state:
     initial: 'off' (0)
     if state was 'off' then it becomes 'on' and vice versa
     off->on switch generates 'high' signal
     on->off switch generates 'low' signal
& - conjunction, remembers all inputs signals
    initial: 'low' (0)
    all 'high' generates 'low' signal, otherwise 'high'
broadcaster - sends signal from input to all outputs
    pressing the button sends 'low' to broadcaster

all states are saved between presses
 */

    // our task is to count all 'high' and 'low' signals after 1000 presses

    // we need to create a graph of modules
    // for each conj modules we need to track all it's inputs
    // let's store type as char, outputs as Vec<String>
    // graph: HashMap<String, Vec<String>>
    // separate map for conj inputs memory, each conj module has it's own memory [(input_name -> input_value)]
    // let 'high' be 1, 'low' be 0
    // conj_memories: HashMap<String, HashMap<String, i32>>
    // we also need a memory for flip-flop modules
    // flip_flop_memories: HashMap<String, i32>
    // then we will do a BFS from broadcaster to all outputs
    // we will store the (signal, module_name) in a queue

    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    let mut conj_memories: HashMap<String, HashMap<String, i32>> = HashMap::new();
    let mut flip_flop_memories: HashMap<String, i32> = HashMap::new();
    let mut module_to_type: HashMap<String, char> = HashMap::new();
    /*
        r###"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"###;
     */
    for line in lines {
        let mut parts = line.split(" -> ");
        let from = parts.next().unwrap();
        let toos = parts.next().unwrap();
        let toos: Vec<String> = toos.split(", ").map(|s| s.to_string()).collect();
        let type_name = from.chars().next().unwrap();
        let from = from.chars().skip(1).collect::<String>();
        graph.insert(from.clone(), toos.clone());
        module_to_type.insert(from.clone(), type_name);
        if type_name == '&' {
            conj_memories.insert(from.clone(), HashMap::new());
        } else if type_name == '%' {
            flip_flop_memories.insert(from.clone(), 0); // insert as 'off'
        }
    }
    // now let's connect conj_memories
    for (from, toos) in &graph {
        // each signal in 'toos' that goes to conj module should be remembered
        for to in toos {
            /*
                r###"broadcaster -> a
        %a -> inv, con
        &inv -> b
        %b -> con
        &con -> output"###;
        // from b -> con
        // and from a -> con
        // so fill memories for con with values (a -> 0, b -> 0)
             */
            if graph.contains_key(&to.clone()) {
                //conj_memories: HashMap<String, HashMap<String, i32>>
                // check 'to' type
                let to_type = module_to_type.get(to).unwrap();
                if *to_type == '&' {
                    // this is a conj module
                    let memory = conj_memories.get_mut(to).unwrap();
                    memory.insert(from.clone(), 0);
                }
            }
        }
    }
    println!("graph: {:?}", graph);

    let mut queue: VecDeque<(i32, String, String)> = VecDeque::new();
    let mut total_high_count = 0;
    let mut total_low_count = 0;
    let mut kl = 0;
    let mut vm = 0;
    let mut kv = 0;
    let mut vb = 0;
    for button_press in 1.. {
        if button_press % 1_000_000 == 0 {
            println!("button_press: {}", button_press);
        }
        let mut high_count = 0;
        let mut low_count = 0;
        queue.push_back((0, "roadcaster".to_string(), "".to_string()));
        if kl != 0 && vm != 0 && kv != 0 && vb != 0 {
            break;
        }
        while let Some((signal, module_name, from_name)) = queue.pop_front() {
            if kl != 0 && vm != 0 && kv != 0 && vb != 0 {
                println!("kl: {}", kl);
                println!("vm: {}", vm);
                println!("kv: {}", kv);
                println!("vb: {}", vb);
                break;
            }
            /*
&ll -> rx

&vb -> ll
&kl -> ll
&vm -> ll
&kv -> ll

so to send low to rx all inputs of ll must be high

             */
            if kl == 0 && module_name == "ll" && from_name == "kl" && signal == 1 {
                kl = button_press;
                /*
kl: 3917
kl: 7834
kl: 11751
kl: 15668
kl: 19585
kl: 23502

                 */
            }
            if vm == 0 && module_name == "ll" && from_name == "vm" && signal == 1 {
                vm = button_press;
                /*
vm: 4032
vm: 4035
vm: 8038
vm: 8041
vm: 12044
vm: 12047
vm: 16050
vm: 16053
vm: 20056
vm: 20059
vm: 24062
                 */
            }
            if kv == 0 && module_name == "ll" && from_name == "kv" && signal == 1 {
                kv = button_press;
                /*
kv: 3968
kv: 3981
kv: 7898
kv: 7911
kv: 11828
kv: 11841
kv: 15758
kv: 15771
kv: 19688

                 */
            }
            if vb == 0 && module_name == "ll" && from_name == "vb" && signal == 1 {
                vb = button_press;
                /*
vb: 3793
vb: 7586
vb: 11379
vb: 15172
vb: 18965
vb: 22758
vb: 26551
vb: 30344
vb: 34137
                 */
            }
            //println!("queue: {:?}", queue);
            //println!("signal: {}, module_name: {}, from_name: {}", signal, module_name, from_name);
            //println!("memories: {:?}", conj_memories);
            //println!("flip_flop_memories: {:?}", flip_flop_memories);
            if signal == 1 {
                high_count += 1;
            } else {
                low_count += 1;
            }

            let Some(type_name) = module_to_type.get(&module_name) else {
                continue;
            };
            //%a -> inv, con
            // first we need to process the current signal with current module
            // then we need to send processed signal to all outputs
            let mut out_signal = 0;
            match *type_name {
                '%' => {
                    /*
            % - flip-flop, 'high' signal ignored, 'low' signal make flip the state:
                initial: 'off' (0)
            if state was 'off' then it becomes 'on' and vice versa
            off->on switch generates 'high' signal
            on->off switch generates 'low' signal
             */
                    // if signal is 'high' then ignore it
                    if signal == 1 {
                        //println!("signal is 'high' then ignore it");
                        continue;
                    }
                    // if signal is 'low' then flip the state
                    let state = flip_flop_memories.get_mut(&module_name).unwrap();
                    if *state == 0 {
                        *state = 1;
                        out_signal = 1;
                    } else {
                        *state = 0;
                        out_signal = 0;
                    }
                    //println!("out_signal: {}", out_signal)
                }
                '&' => {
                    /*
            & - conjunction, remembers all inputs signals
                initial: 'low' (0)
                all 'high' generates 'low'(0) signal, otherwise 'high'(1)
             */
                    // this module must be processed automatically
                    // this is a conj module
                    let memory = conj_memories.get_mut(&module_name).unwrap();
                    memory.insert(from_name.clone(), signal);
                    //println!("new memory: {:?}", memory);
                    // now we need to check if all inputs are 'high'
                    let mut all_high = true;
                    for (_, value) in memory {
                        if *value == 0 {
                            all_high = false;
                            break;
                        }
                    }
                    out_signal = if all_high { 0 } else { 1 };
                }
                'b' => {
                    // send the same signal
                    out_signal = signal;
                }
                _ => panic!("unknown type_name: {}", type_name)
            }
            // send out_signal to all outputs
            if let Some(outputs) = graph.get(&module_name.clone()) {
                for output in outputs {
                    //println!("back of queue {} -{}- -> {}", module_name, out_signal, output);
                    queue.push_back((out_signal, output.clone(), module_name.clone()));
                }
            }
        }
        total_high_count += high_count;
        total_low_count += low_count;
        if button_press == 1000 {
            println!("total: h= {}, l={}", total_high_count, total_low_count);
            println!("part1: {}", total_high_count as u128 * total_low_count as u128);
        }
    }
    /*
    kl: 3917
    vm: 4032
    kv: 3968
    vb: 3793
     */

    let answer:u128 = kl as u128 * vm as u128 * kv as u128 * vb as u128;
    println!("part2: {}", answer);
}

