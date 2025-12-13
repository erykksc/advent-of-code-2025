use std::fs;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Battery {
    idx: usize, // index in bank/input-line
    joltage: u32,
}

fn main() {
    let input =
        fs::read_to_string("inputs/day_03.txt").expect("Should have been able to read the file");
    // let input = r#"
    // 987654321111111
    // 811111111111119
    // 234234234234278
    // 818181911112111
    // 482332323232824
    // "#;

    let mut answer: u64 = 0;
    println!("=============== PART 1 ===============");
    for bank in input.trim().lines() {
        println!("'{}'", bank);
        // let digits2 = parse_fixed_width_digits(bank)
        let digits: Vec<u32> = bank.chars().filter_map(|c| c.to_digit(10)).collect();

        let mut batteries: Vec<Battery> = digits
            .iter()
            .enumerate()
            .map(|(i, d)| Battery {
                idx: i,
                joltage: *d,
            })
            .collect();

        batteries.sort_by(|a, b| b.joltage.cmp(&a.joltage).then_with(|| a.idx.cmp(&b.idx)));

        let first_battery: &Battery;
        let second_battery: &Battery;

        // the biggest digit is at the end
        if batteries.get(0).unwrap().idx == batteries.len() - 1 {
            first_battery = batteries.get(1).unwrap();
            second_battery = batteries.get(0).unwrap();
        } else {
            first_battery = batteries.get(0).unwrap();
            let bank_rest: Vec<&Battery> = batteries
                .iter()
                .filter(|battery| battery.idx > first_battery.idx)
                .collect();
            second_battery = bank_rest.get(0).unwrap();
        }

        println!(
            "first={} second={}",
            first_battery.joltage, second_battery.joltage
        );

        let bank_joltage = format!("{}{}", first_battery.joltage, second_battery.joltage);
        let bank_joltage_num: u64 = bank_joltage.parse().unwrap();
        answer += bank_joltage_num;
    }
    println!("Final answer: {}", answer);

    // PART 2
    println!("=============== PART 2 ===============");
    answer = 0;
    for bank in input.trim().lines() {
        println!("'{}'", bank);
        let digits: Vec<u32> = bank.chars().filter_map(|c| c.to_digit(10)).collect();

        // parse line into batteries
        let mut batteries: Vec<Battery> = digits
            .iter()
            .enumerate()
            .map(|(i, d)| Battery {
                idx: i,
                joltage: *d,
            })
            .collect();

        // get batteries sorted by joltage (descending), idx (ascending)
        batteries.sort_by(|a, b| b.joltage.cmp(&a.joltage).then_with(|| a.idx.cmp(&b.idx)));

        let mut batteries_chosen: Vec<&Battery> = Vec::new();
        let mut last_battery_chosed_idx = 0; // highest index of chosen battery
        for battery2chose_offset in (1..=12).rev() {
            let batteries_available: Vec<&Battery> = batteries
                .iter()
                // can choose batteries only in order, from left to right
                .filter(|b| batteries_chosen.len() == 0 || b.idx > last_battery_chosed_idx)
                // there needs to be enough batteries to choose from (remove x last batteries)
                .filter(|b| b.idx <= batteries.len() - battery2chose_offset)
                .collect();

            // choose battery with highest joltage and lowest idx
            let battery_chosen = batteries_available.get(0).unwrap();

            batteries_chosen.push(battery_chosen);
            last_battery_chosed_idx = battery_chosen.idx;
        }

        // read combined joltage of the batteries
        let bank_joltage: String = batteries_chosen
            .iter()
            .map(|b| b.joltage.to_string())
            .collect();
        println!("choice: {}", bank_joltage);
        let bank_joltage_num: u64 = bank_joltage.parse().unwrap();

        answer += bank_joltage_num;
    }
    println!("Final answer: {}", answer);
}
