use std::{fs, vec};

fn main() {
    let input = fs::read_to_string("inputs/day_07.txt")
        // let input = fs::read_to_string("inputs/day_07.txt")
        .expect("Could not read input file: inputs/day_07-example.txt");

    let mut lines = input.lines();

    let first_line = lines
        .next()
        .expect("Could not read the first line of input");

    let start_pos: usize = first_line
        .find("S")
        .expect("Could not find 'S' in the first line");

    let mut timelines = vec![0; first_line.len()];
    timelines[start_pos] = 1;
    let mut total_splits: u64 = 0;

    for line in lines {
        let mut new_beams = vec![0; first_line.len()];
        for (i, c) in line.chars().enumerate() {
            // timeline that is coming onto the character 'c'
            let timeline = timelines[i];
            if timeline == 0 {
                continue;
            }

            match c {
                '^' => {
                    total_splits += 1;
                    if i > 0 {
                        new_beams[i - 1] += timeline;
                    }
                    if i + 1 < first_line.len() {
                        new_beams[i + 1] += timeline;
                    }
                }
                // copy over the previous row
                '.' => new_beams[i] += timeline,

                _ => panic!("Unknown character '{}' in on the line {}", c, i + 1),
            }
        }
        for (i, beam) in new_beams.iter().enumerate() {
            let c = line.chars().nth(i).unwrap();
            print!("{}", if *beam > 0 { '|' } else { c })
        }
        println!();
        timelines = new_beams;
    }

    let possible_timelines: usize = timelines.iter().sum();
    println!("total splits: {}", total_splits);
    println!("possible_timelines: {}", possible_timelines);
}
