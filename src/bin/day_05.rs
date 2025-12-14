use core::fmt;
use std::fs;

#[derive(Clone, Copy)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn contains(&self, id: usize) -> bool {
        return self.start <= id && id <= self.end;
    }

    fn total_ids(&self) -> usize {
        return self.end - self.start + 1;
    }
}

impl fmt::Display for Range {
    // The required method is 'fmt'
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use write! macro to send the desired output to the formatter
        write!(f, "({}-{})", self.start, self.end)
    }
}

fn main() {
    let input =
        fs::read_to_string("inputs/day_05.txt").expect("Should be able to read the input file");
    // let input = r#"3-5
    // 10-14
    // 16-20
    // 12-18
    //
    // 1
    // 5
    // 8
    // 11
    // 17
    // 32"#;

    let mut scanning_ranges = true;
    let mut fresh_ingredient_ranges: Vec<Range> = Vec::new();
    let mut available_ingredients: Vec<usize> = Vec::new();

    for line in input.trim().lines() {
        println!("Parsing line: {}", line);
        let tline = line.trim();

        if tline.len() == 0 {
            scanning_ranges = false;
            continue;
        }

        if scanning_ranges {
            // scanning fresh_ingredients
            let (start_str, end_str) = tline.split_once("-").unwrap();
            fresh_ingredient_ranges.push(Range {
                start: start_str.parse().unwrap(),
                end: end_str.parse().unwrap(),
            });
        } else {
            // scanning available_ingredients
            let available_id: usize = tline.parse().unwrap();
            available_ingredients.push(available_id);
        }
    }

    for range in &fresh_ingredient_ranges {
        println!("Range: {}", range);
    }
    for id in &available_ingredients {
        println!("ID: {}", id);
    }

    println!("=============== PART 1 ===============");
    let mut available_fresh_id_count = 0;
    for id in available_ingredients {
        for range in &fresh_ingredient_ranges {
            if range.contains(id) {
                available_fresh_id_count += 1;
                break;
            }
        }
    }
    println!("available_fresh_id_count: {}", available_fresh_id_count);

    println!("=============== PART 2 ===============");
    // merge ranges to avoid duplicates
    fresh_ingredient_ranges.sort_by_key(|range| range.start);
    let mut merged_ranges: Vec<Range> = Vec::new();
    let mut range_current = fresh_ingredient_ranges[0];
    for range_i in fresh_ingredient_ranges {
        // basic situation, no overlap
        if range_current.end < range_i.start {
            merged_ranges.push(range_current);
            range_current = range_i;
            continue;
        } else {
            // assume range_current.end >= range_i.start, merge required
            // check which range ends later
            if range_current.end > range_i.end {
                // skip range_i as it is included in the range_current
                continue;
            } else {
                // merge range_current and range_i by extending the end
                range_current.end = range_i.end;
                continue;
            }
        }
    }
    // add the final range
    merged_ranges.push(range_current);

    let mut total_available_fresh_id_count = 0;
    for range in &merged_ranges {
        total_available_fresh_id_count += range.total_ids();
    }
    println!(
        "total_available_fresh_id_count: {}",
        total_available_fresh_id_count
    );
}
