use std::{collections::HashSet, fs};

struct Problem {
    target: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<i64>,
}

fn parse_problem(line: &str) -> Problem {
    let (target_str, buttons_joltage_str) = line.split_once(' ').unwrap();

    let target: Vec<bool> = target_str
        .chars()
        .filter(|&c| c != '[' && c != ']')
        .map(|c| c == '#')
        .collect();
    for b in &target {
        print!("{}", if *b { '#' } else { '.' })
    }
    print!(" ");

    let (buttons_str, joltage_str) = buttons_joltage_str.split_once('{').unwrap();

    let buttons: Vec<Vec<usize>> = buttons_str
        .trim()
        .split_whitespace()
        .map(|btn_str| {
            btn_str[1..btn_str.len() - 1] // trim parenthesis
                .split(',')
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect();
    for button in &buttons {
        print!("(");
        for number in button {
            print!("{},", *number);
        }
        print!(") ");
    }
    let joltage: Vec<i64> = joltage_str[..joltage_str.len() - 1]
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();
    for button in &buttons {
        print!("{{");
        for number in button {
            print!("{},", *number);
        }
        print!("}} ");
    }
    println!("");

    return Problem {
        target,
        buttons,
        joltage,
    };
}

fn solve_problem(problem: &Problem) -> i64 {
    let target = &problem.target;
    let buttons = &problem.buttons;

    let mut unvisited: Vec<Vec<bool>> = vec![vec![false; target.len()]];
    let mut visited: HashSet<Vec<bool>> = HashSet::new();
    let mut depth: i64 = 0;

    while unvisited.len() > 0 {
        depth += 1;
        let mut new_unvisited: Vec<Vec<bool>> = vec![vec![false; target.len()]];

        for state in unvisited {
            visited.insert(state.clone());

            print!("Entering new state: ");
            for b in &state {
                print!("{}", if *b { '#' } else { '.' })
            }
            println!("");

            for button in buttons {
                let mut new_state = state.clone();
                for num in button {
                    new_state[*num] = !new_state[*num];
                }
                // let new_state = button.press(state);
                if !visited.contains(&new_state) {
                    new_unvisited.push(new_state.clone());
                };
                if new_state == *target {
                    println!("Part answer: {}", depth);
                    return depth;
                };
            }
        }
        unvisited = new_unvisited;
    }
    panic!("Unsolvable riddle");
}

fn main() {
    let input =
        fs::read_to_string("inputs/day_10.txt").expect("Should be able to read the input file");

    let mut answer = 0;

    for line in input.trim().lines() {
        println!("Parsing line: {}", line);
        let tline = line.trim();
        if tline.len() == 0 {
            continue;
        }

        let problem = parse_problem(tline);

        answer += solve_problem(&problem);
    }

    println!("Part 1 answer: {}", answer)
}
