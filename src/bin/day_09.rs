use std::{collections::HashSet, fs};

#[derive(Eq, Hash, PartialEq)]
struct Tile {
    x: u64,
    y: u64,
}

fn area(tile_a: &Tile, tile_b: &Tile) -> u64 {
    let x_diff = tile_a.x.max(tile_b.x) - tile_a.x.min(tile_b.x);
    let y_diff = tile_a.y.max(tile_b.y) - tile_a.y.min(tile_b.y);

    return (x_diff + 1) * (y_diff + 1);
}

fn main() {
    // let input = fs::read_to_string("inputs/day_09-example.txt").expect("Could not read input file");
    let input = fs::read_to_string("inputs/day_09.txt").expect("Could not read input file");

    let mut red_tiles: HashSet<Tile> = HashSet::new();
    for line in input.lines() {
        let (x_str, y_str) = line.split_once(",").unwrap();

        let tile = Tile {
            x: x_str.parse().unwrap(),
            y: y_str.parse().unwrap(),
        };

        red_tiles.insert(tile);
    }

    let mut area_largest = 0;
    for tile_a in &red_tiles {
        for tile_b in &red_tiles {
            if tile_a == tile_b {
                continue;
            }

            let area = area(tile_a, tile_b);

            if area > area_largest {
                area_largest = area;
            }
        }
    }

    println!("Largest area: {}", area_largest)
}
