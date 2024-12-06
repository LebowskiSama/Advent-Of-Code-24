// Day 1 - Historian Hysteria https://adventofcode.com/2024/day/1
// figure out the total distance between two given lists
// part - 1 distance = distance(left_large. right_large) + distance(left_second_largest, right_second_largest) + ... + distance(left_n_large, right_n_large)
// part - 2 similarity score = sum(each number in left list * frequency of occurence in right list)

use std::collections::HashMap;

fn main() {
    // parse inputs
    let data_path: String = String::from("./input/input.txt");
    let lines = parser(&data_path);
    // convert to ints
    let (left, right) = split_cast(&lines);

    // part one
    // calculate distance as sum of absolute differences in sorted pairs
    let distance = calc_distance(&left, &right);

    // part two
    // calculate similarity score
    let similarity_score = calc_similarity(&left, &right);

    println!("distance: {}", distance);
    println!("similarity_score: {}", similarity_score);
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

fn calc_similarity(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    let mut similarity_score: i32 = 0;
    let mut frequency_map: HashMap<i32, i32> = HashMap::new();
    for left_ele in left.iter() {
        if frequency_map.contains_key(left_ele) {
            similarity_score += left_ele * frequency_map.get(left_ele).unwrap();
        } else {
            // calculate frequency
            let frequency = binary_search_freq(*left_ele, right);
            // update similarity score
            similarity_score += left_ele * frequency;
            // push new frequency for ele into frequency map for future repetitions of same left_ele
            frequency_map.entry(*left_ele).or_insert(frequency);
        }
    }
    similarity_score
}

fn binary_search_freq(target: i32, nums: &Vec<i32>) -> i32 {
    // sorted list => binary search
    let mut left: usize = 0;
    let mut right: usize = nums.len();
    let mut freq: usize = 0;

    while left < right {
        let mid: usize = left + ((right - left) / 2);
        if nums[mid] == target {
            // increment frequency for mid match
            freq += 1;
            // dual pointer spread search
            let mut a = mid - 1;
            let mut b = mid + 1;
            while nums[a] == target && a > 0 {
                // increment frequency for left match
                freq += 1;
                a -= 1;
            }
            while nums[b] == target && b < nums.len(){
                // increment frequency for right match
                freq += 1;
                b += 1;
            }
            return freq as i32
        } else if target > nums[mid] {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    return 0
}