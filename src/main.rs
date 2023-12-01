fn main() {
    println!("Hello, world!");
    /*
            example lines are:
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
     */
    // let's print example lines one by one
    let example_lines =
        vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
    // convert example lines to String
    let example_lines: Vec<String> =
        example_lines.iter().map(|s| s.to_string()).collect();

    solve(example_lines);

    // read lines from ./day1/input.txt
    let input = std::fs::read_to_string("./day1/input.txt").unwrap();
    // split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    solve(input);

    // part two
    /*
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
     */
    let example_lines: Vec<String> = vec![
        "two1nine",
        "eightwothree",
        "abcone2threexyz",
        "xtwone3four",
        "4nineeightseven2",
        "zoneight234",
        "7pqrstsixteen",
    ].iter().map(|s| s.to_string()).collect();
    /*
    set of numbers as strings:
        one
        two
        three
        four
        five
        six
        seven
        eight
        nine
     */
    let numbers: Vec<String> = vec![
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
    ].iter().map(|s| s.to_string()).collect();
    solve_part_two(example_lines, numbers);
    let input = std::fs::read_to_string("./day1/input.txt").unwrap();
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    let numbers: Vec<String> = vec![
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
    ].iter().map(|s| s.to_string()).collect();
    solve_part_two(input, numbers);

}

fn solve_part_two(example_lines: Vec<String>, numbers: Vec<String>) {
    // convert numbers to HashMap: one -> 1, two -> 2, ...
    let mut numbers_map = std::collections::HashMap::new();
    for (i, number) in numbers.iter().enumerate() {
        numbers_map.insert(number.clone(), i + 1);
    }
    // analyze each line
    let mut sum = 0;
    for line in example_lines {
        // find first and last digits in line
        let mut first_digit = -1;
        let mut last_digit = -1;
        // iterate over characters in line,
        // if c is digit, use it
        // else take chars and search in O(n^2) for number
        // if found, use it and adjust the index
        let mut ind = 0;
        while ind < line.len() {
            let c = line.chars().nth(ind).unwrap();
            if (c.is_digit(10)) {
                if first_digit == -1 {
                    first_digit = c.to_digit(10).unwrap() as i32;
                }
                last_digit = c.to_digit(10).unwrap() as i32;
                ind += 1;
            } else {
                // c is not digit
                // find number in numbers_map
                let mut found = false;
                // search from ind to min(line.len(), next_digit_pos - 1)
                let mut next_digit_pos = line.len();

                let mut to = ind;
                while to < line.len() {
                    let substr = &line[ind..=to];
                    if let Some(number) = numbers_map.get(substr) {
                        // if found, set 'fl' number to number
                        let digit = *number as i32;
                        if first_digit == -1 {
                            first_digit = digit;
                        }
                        last_digit = digit;
                        found = true;
                        next_digit_pos = to;
                        break;
                    }
                    to += 1;
                }

                if found {
                    ind = next_digit_pos;
                } else {
                    ind += 1;
                }
            }
        }

        // add 'fl' number to sum
        sum += first_digit * 10 + last_digit;
    }
    // print sum
    println!("sum part 2 = {}", sum);
}


fn solve(example_lines: Vec<String>) {
    let mut sum = 0;
    // analyze each line
    for line in example_lines {
        // find first and last digits in line
        let mut first_digit = -1;
        let mut last_digit = -1;
        for (i, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                if first_digit == -1 {
                    first_digit = c.to_digit(10).unwrap() as i32;
                    last_digit = c.to_digit(10).unwrap() as i32;
                }
                last_digit = c.to_digit(10).unwrap() as i32;
            }
        }
        // convert 'f' and 'l' digits to 'fl' number
        // add 'fl' number to sum
        sum += first_digit * 10 + last_digit;
    }
    // print sum
    println!("sum = {}", sum);
}
