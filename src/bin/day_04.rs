use std::{collections::HashSet, fs};

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

fn get_adj(p: &Point, grid_x_size: usize, grid_y_size: usize) -> HashSet<Point> {
    let mut res: HashSet<Point> = HashSet::new();

    // top row
    if p.x > 0 && p.y > 0 {
        res.insert(Point {
            x: p.x - 1,
            y: p.y - 1,
        });
    }
    if p.y > 0 {
        res.insert(Point { x: p.x, y: p.y - 1 });
    }
    if p.x < grid_x_size - 1 && p.y > 0 {
        res.insert(Point {
            x: p.x + 1,
            y: p.y - 1,
        });
    }

    // middle row
    if p.x > 0 {
        res.insert(Point { x: p.x - 1, y: p.y });
    }
    if p.x < grid_x_size - 1 {
        res.insert(Point { x: p.x + 1, y: p.y });
    }

    // bottom row
    if p.y < grid_y_size - 1 && p.x > 0 {
        res.insert(Point {
            x: p.x - 1,
            y: p.y + 1,
        });
    }
    if p.y < grid_y_size - 1 {
        res.insert(Point { x: p.x, y: p.y + 1 });
    }
    if p.y < grid_y_size - 1 && p.x < grid_x_size - 1 {
        res.insert(Point {
            x: p.x + 1,
            y: p.y + 1,
        });
    }

    return res;
}

fn is_paper(c: char) -> bool {
    return c == '@';
}

fn main() {
    let input = fs::read_to_string("inputs/day_04.txt")
        .expect("The input file day_04.txt should be accessible");
    // let input = r#"
    // ..@@.@@@@.
    // @@@.@.@.@@
    // @@@@@.@.@@
    // @.@@@@..@.
    // @@.@@@@.@@
    // .@@@@@@@.@
    // .@.@.@.@@@
    // @.@@@.@@@@
    // .@@@@@@@@.
    // @.@.@@@.@.
    // "#;

    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut grid_x_size: usize = 0;
    let mut grid_y_size: usize = 0;
    // parse the grid
    for (y, line) in input.trim().lines().enumerate() {
        grid.push(Vec::new());

        if grid_x_size == 0 {
            grid_x_size = line.trim().len();
        }
        grid_y_size += 1;

        for (_x, c) in line.trim().chars().enumerate() {
            grid[y].push(c);
        }
    }

    println!("=============== PART 1 ===============");
    let mut moveable_papers: u64 = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            let point = Point { x, y };
            if !is_paper(*c) {
                continue;
            }
            let mut adj_papers: u64 = 0;
            for point_adj in get_adj(&point, grid_x_size, grid_y_size) {
                if is_paper(grid[point_adj.y][point_adj.x]) {
                    adj_papers += 1;
                }
            }
            if adj_papers < 4 {
                moveable_papers += 1;
                println!("point={:?} adj_papers={}", point, adj_papers);
            }
        }
    }
    println!("moveable_papers={}", moveable_papers);

    println!("=============== PART 2 ===============");
    let mut removed_papers: u64 = 0;
    let mut loop_count: u32 = 0;
    loop {
        println!("Entering loop {}", loop_count);
        loop_count += 1;

        let mut to_be_removed: Vec<Point> = Vec::new();

        for (y, row) in grid.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                let point = Point { x, y };
                if !is_paper(*c) {
                    continue;
                }
                let mut adj_papers: u64 = 0;
                for point_adj in get_adj(&point, grid_x_size, grid_y_size) {
                    if is_paper(grid[point_adj.y][point_adj.x]) {
                        adj_papers += 1;
                    }
                }
                if adj_papers < 4 {
                    to_be_removed.push(point);
                    println!("point={:?} adj_papers={}", point, adj_papers);
                }
            }
        }
        if to_be_removed.len() == 0 {
            break;
        }
        while let Some(point) = to_be_removed.pop() {
            grid[point.y][point.x] = '.';
            removed_papers += 1;
        }
    }
    println!("total_removed_papers={}", removed_papers);
}
