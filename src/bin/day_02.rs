use std::fs;

use std::collections::HashSet;

fn split_equal_substr(s: &str, chunk_size: usize) -> Vec<String> {
    let mut result = Vec::new();
    let mut chars = s.chars();

    loop {
        let chunk: String = chars.by_ref().take(chunk_size).collect();
        if chunk.is_empty() {
            break;
        }
        result.push(chunk);
    }

    return result;
}

fn main() {
    let input_path = "inputs/day_02.txt";
    let input = fs::read_to_string(input_path).expect("Should have been able to read the file");

    // let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    let mut answer: u64 = 0;

    println!("=============== PART 1 ===============");
    for range in input.trim().split(",") {
        println!("{}", range);

        // Parse range
        let (start_str, end_str) = range.split_once("-").unwrap();
        let start: u64 = start_str.parse().unwrap();
        let end: u64 = end_str.parse().unwrap();

        for id in start..end + 1 {
            let id_str = id.to_string();
            if id_str.len() % 2 != 0 {
                continue;
            }

            let mid = id_str.len() / 2;
            let first_half = id_str.get(0..mid).unwrap();
            let second_half = id_str.get(mid..id_str.len()).unwrap();

            let is_valid = first_half != second_half;

            if !is_valid {
                answer += id;
                println!(
                    "i: id={} first_half={} second_half={} valid={}",
                    id, first_half, second_half, is_valid
                );
            }
        }
    }
    println!("answer: {}", answer);

    println!("=============== PART 2 ===============");
    let mut invalid_ids: HashSet<u64> = HashSet::new();
    for range in input.trim().split(",") {
        println!("{}", range);

        // Parse range
        let (start_str, end_str) = range.split_once("-").unwrap();
        let start: u64 = start_str.parse().unwrap();
        let end: u64 = end_str.parse().unwrap();

        for id in start..end + 1 {
            let id_str = id.to_string();

            let longest_pattern = id_str.len() / 2;

            for pattern_len in 1..longest_pattern + 1 {
                if id_str.len() % pattern_len != 0 {
                    continue;
                }
                let chunks = split_equal_substr(&id_str, pattern_len);
                // println!("chunks: {:?}", chunks);

                // check if all chunks are the same
                let chunk_1 = chunks.get(0).expect("At least one chunk");

                let mut is_valid = false;

                let rest_of_chunks = &chunks[1..];
                for c in rest_of_chunks {
                    if chunk_1 != c {
                        is_valid = true;
                        break;
                    }
                }

                if !is_valid {
                    invalid_ids.insert(id);
                    println!("i: id={} chunk={} valid={}", id, chunk_1, is_valid);
                }
            }
        }
    }
    answer = invalid_ids.iter().sum();
    println!("answer: {}", answer);
}
