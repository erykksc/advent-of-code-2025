use std::fs;

#[derive(Debug)]
struct Problem {
    numbers: Vec<i64>,
    operation: char,
}

impl Problem {
    fn solve(&self) -> i64 {
        if self.operation == '*' {
            return self
                .numbers
                .iter()
                .map(|&n| n)
                .reduce(|acc, number| acc * number)
                .unwrap();
        }
        // otherwise sum up
        self.numbers.iter().sum()
    }
}

fn get_char(s: &str, x: usize, y: usize) -> Option<char> {
    s.lines().nth(y)?.chars().nth(x)
}
fn get_column(s: &str, x: usize, column_len: usize) -> String {
    let mut res: String = String::new();
    for y in 0..column_len {
        let c = get_char(s, x, y).unwrap();
        res = format!("{}{}", res, c);
    }
    return res;
}

fn main() {
    let input =
        fs::read_to_string("inputs/day_06.txt").expect("Should be able to read the input file");

    //     let input = r#"123 328  51 64
    //  45 64  387 23
    //   6 98  215 314
    // *   +   *   +
    // "#;

    println!("=============== PART 1 ===============");
    let mut problems: Vec<Problem> = Vec::new();
    let mut problems_initialized = false;
    for line in input.trim().lines() {
        let line_elems = line.split_whitespace();

        for (i, elem) in line_elems.enumerate() {
            if !problems_initialized {
                problems.push(Problem {
                    numbers: vec![],
                    operation: '.',
                });
            }
            if elem == "*" || elem == "+" {
                problems[i].operation = elem.chars().next().unwrap();
            } else {
                problems[i].numbers.push(elem.parse().unwrap());
            }
        }
        problems_initialized = true;
    }

    let answer = problems.iter().fold(0, |acc, p| acc + p.solve());
    println!("Answer: {}", answer);

    println!("=============== PART 2 ===============");
    let x_size: usize = input.lines().next().unwrap().len();
    let y_size: usize = input.lines().fold(0, |acc, _| acc + 1);

    let mut new_problems: Vec<Problem> = Vec::new();
    let mut new_problem = true;
    let mut problem_current: Problem = Problem {
        numbers: vec![],
        operation: ' ',
    };
    for x in 0..x_size {
        let rcol = get_column(&input, x, y_size - 1);
        let col = rcol.trim();
        if col.trim().len() == 0 {
            println!("Adding problem {:?}", problem_current);
            new_problems.push(problem_current);
            problem_current = Problem {
                numbers: vec![],
                operation: ' ',
            };
            new_problem = true;
            continue;
        }
        println!("Parsing column: '{}'", col);
        let number: i64 = col.parse().unwrap();

        if new_problem {
            let operation = get_char(&input, x, y_size - 1).unwrap();
            problem_current.operation = operation;
            new_problem = false;
        }
        problem_current.numbers.push(number);
    }
    // add the final problem
    new_problems.push(problem_current);

    let answer = new_problems.iter().fold(0, |acc, p| acc + p.solve());
    println!("Answer: {}", answer);
}
