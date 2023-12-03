pub(crate) fn day2() {
    println!("Hello, world!");
    /*
            example lines are:
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
     */
    // let's print example lines one by one
    let example_lines =
        vec!["Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
             "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
             "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
             "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
             "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"];
    // convert example lines to String
    let example_lines: Vec<String> =
        example_lines.iter().map(|s| s.to_string()).collect();

    part1(example_lines);

    // read lines from ./day1/day1.txt
    let input = std::fs::read_to_string("./inputs/day2.txt").unwrap();
    // split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    part1(input);

    let example_lines =
        vec!["Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
             "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
             "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
             "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
             "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"];
    // convert example lines to String
    let example_lines: Vec<String> =
        example_lines.iter().map(|s| s.to_string()).collect();

    part2(example_lines);

    // read lines from ./day1/day1.txt
    let input = std::fs::read_to_string("./inputs/day2.txt").unwrap();
    // split input into lines
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    part2(input);
}

fn part1(lines: Vec<String>) {
    /*
            example lines are:
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
     */
    // only 12 red cubes, 13 green cubes, and 14 blue cubes
    /*
    Let's parse by template:
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    Game {id}:{round1};{round2};...;{roundN}
     */
    let mut sum = 0;
    lines.iter().for_each(|line| {
        // split line by ":"
        let mut line = line.split(":");
        // get game id
        let game_str = line.next().unwrap();
        let game_id_str = game_str.split(" ").last().unwrap();
        let game_id = game_id_str.parse::<u32>().unwrap();
        // get rounds
        let rounds_str = line.next().unwrap();
        let rounds_str = rounds_str.split(";").collect::<Vec<&str>>();
        // check each round for cubes, no more than 12 red, 13 green, 14 blue
        // iterate over rounds
        let mut round_valid = true;
        rounds_str.iter().for_each(|round_str| {
            // parse cubes
            // split round by ","
            let cubes_str = round_str.split(",").collect::<Vec<&str>>();
            // iterate over cubes and match with red, green and blue
            cubes_str.iter().for_each(|cube_str| {
                // split cube by " "
                let cube_str = cube_str.trim().split(" ").collect::<Vec<&str>>();
                // get cube color
                let cube_color = cube_str[1];
                // get cube count
                let cube_count_str = cube_str[0];
                let cube_count = cube_count_str.parse::<u32>().unwrap();
                // check cube color
                match cube_color {
                    "red" => {
                        if cube_count > 12 {
                            round_valid = false;
                        }
                    }
                    "green" => {
                        if cube_count > 13 {
                            round_valid = false;
                        }
                    }
                    "blue" => {
                        if cube_count > 14 {
                            round_valid = false;
                        }
                    }
                    _ => {}
                }
            })
        });

        if round_valid {
            sum += game_id;
        }

    });

    println!("sum part 1 = {}", sum);
}

fn part2(lines: Vec<String>) {
    /*
            example lines are:
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
     */
    /*
    Let's parse by template:
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    Game {id}:{round1};{round2};...;{roundN}
     */
    let mut sum = 0;
    lines.iter().for_each(|line| {
        // split line by ":"
        let mut line = line.split(":");
        // get game id
        let game_str = line.next().unwrap();
        let game_id_str = game_str.split(" ").last().unwrap();
        let game_id = game_id_str.parse::<u32>().unwrap();
        // get rounds
        let rounds_str = line.next().unwrap();
        let rounds_str = rounds_str.split(";").collect::<Vec<&str>>();
        // check each round for cubes, and compute the power,
        // which is max(blue)*max(green)*max(red)
        // iterate over rounds
        let mut max_blue = 1;
        let mut max_green = 1;
        let mut max_red = 1;
        rounds_str.iter().for_each(|round_str| {
            // parse cubes
            // split round by ","
            let cubes_str = round_str.split(",").collect::<Vec<&str>>();
            // iterate over cubes and match with red, green and blue
            cubes_str.iter().for_each(|cube_str| {
                // split cube by " "
                let cube_str = cube_str.trim().split(" ").collect::<Vec<&str>>();
                // get cube color
                let cube_color = cube_str[1];
                // get cube count
                let cube_count_str = cube_str[0];
                let cube_count = cube_count_str.parse::<u32>().unwrap();
                // check cube color
                match cube_color {
                    "red" => {
                        max_red = max_red.max(cube_count);
                    }
                    "green" => {
                        max_green = max_green.max(cube_count);
                    }
                    "blue" => {
                        max_blue = max_blue.max(cube_count);
                    }
                    _ => {}
                }
            })
        });

        sum += max_blue * max_green * max_red;

    });

    println!("sum part 2 = {}", sum);
}


