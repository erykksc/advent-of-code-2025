use std::fs;

#[derive(Debug, PartialEq)]
enum Direction {
    L, // Left
    R, // Right
}

#[derive(Debug)]
struct Instruction {
    dir: Direction,
    steps: i32,
}

fn parse_line(s: &str) -> Instruction {
    let dir_str = s.chars().next().unwrap();
    let dir: Direction = match dir_str {
        'L' => Direction::L,
        'R' => Direction::R,
        _ => panic!("Invalid direction: {}", dir_str),
    };
    let steps_str = s.get(1..).unwrap();
    let steps: i32 = steps_str.parse().unwrap_or_default();

    return Instruction {
        dir: dir,
        steps: steps,
    };
}

fn main() {
    let input_path = "inputs/day_01.txt";

    // Read the file content into a string.
    let input = fs::read_to_string(input_path).expect("Should have been able to read the file");

    let mut dial: i32 = 50;
    let mut password: u32 = 0;

    // Part 1
    println!("=============== PART 1 ===============");
    for (i, line) in input.lines().enumerate() {
        let instr = parse_line(line);
        println!("{}: {:?}", line, instr);

        dial = match instr.dir {
            Direction::L => dial - instr.steps,
            Direction::R => dial + instr.steps,
        };
        dial = dial.rem_euclid(100);

        if dial == 0 {
            password += 1;
        }
        println!("after step {}: dial={}, password={}", i + 1, dial, password);
    }
    println!("final password: {}", password);

    // Part 2
    println!("=============== PART 2 ===============");
    dial = 50;
    password = 0;
    for (i, line) in input.lines().enumerate() {
        let instr = parse_line(line);
        println!("{}: {:?}", line, instr);

        let mut steps = instr.steps;
        while steps > 0 {
            steps -= 1;
            dial = match instr.dir {
                Direction::L => dial - 1,
                Direction::R => dial + 1,
            }
            .rem_euclid(100);

            if dial == 0 {
                password += 1
            }
        }

        println!("after step {}: dial={}, password={}", i + 1, dial, password);
    }
    println!("final password: {}", password);
}
