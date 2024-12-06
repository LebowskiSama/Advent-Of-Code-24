// Day 1 - Historian Hysteria https://adventofcode.com/2024/day/1
// figure out the total distance between two given lists
// distance = distance(left_large. right_large) + distance(left_second_largest, right_second_largest) + ... + distance(left_n_large, right_n_large)

fn main() {
    // parse inputs
    let data_path: String = String::from("./input/input.txt");
    let lines = parser(&data_path);
    // convert to ints
    let (left, right) = split_cast(&lines);
    // calculate distance as sum of absolute differences in sorted pairs
    let distance = calc_distance(&left, &right);
    
    println!("{}", distance);
}

fn parser(path_to_data: &str) -> Vec<String> {
    // read lines
    std::fs::read_to_string(path_to_data)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn split_cast(lines: &Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let mut left: Vec<i32> =  Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for s in lines.iter() {
        // split by 3 spaces per line
        let split: Vec<&str> = s.split("   ").collect();
        left.push(split[0].parse().expect("Can't parse string to i32, check inputs"));
        right.push(split[1].parse().expect("Can't parse string to i32, check inputs"));
    }
    // there is no need to reverse into descending as the final objective is to find total distance between sorted pairs
    left.sort_unstable();
    right.sort_unstable();
    (left, right)
}

fn calc_distance(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    // take in sorted left and right lists and return sum total distance between pairs
    let mut distance: i32 = 0;
    
    for (ref_left, ref_right) in left.iter().zip(right.iter()) {
        distance += (ref_left - ref_right).abs();
    }

    distance   
}