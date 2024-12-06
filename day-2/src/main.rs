// Day 1 - Red-Nosed Reports
// part 1 - figure out if values are either strictly increasing or decreasing and inter integer distance is at least 1 and utmost 3

fn main() {
    // parse input data
    let data_path: String = String::from("./input/input.txt");
    let reports = parser(&data_path);
    // calculate total safe reports present
    let n_safe: i32 = calc_safety(&reports);
    println!("number of safe reports: {}", n_safe);
}

fn parser(data_path: &str) -> Vec<Vec<i32>> {
    std::fs::read_to_string(data_path)
        .unwrap()
        .lines()
        .map(|x| split_cast(String::from(x)))
        .collect()
}

fn split_cast(x: String) -> Vec<i32> {
    let mut reports: Vec<i32> = Vec::new();
    // split and cast string
    for val in x.split(" ") {
        reports.push(val.parse().expect("Can't parse string to i32, check inputs"));
    }

    reports
}

fn calc_safety(reports: &Vec<Vec<i32>>) -> i32 {
    // a report is considered safe if and only if
    // 1. the progression is strictly ascending or descending
    // 2. the difference between two consecutive readings is at least 1 and utmost 3

    let mut n_safe = 0;

    for report in reports.iter() {
        let mut i: usize= 1;
        let n: usize = report.len();
        // calculate progression of first two elements
        let asc_desc = (report[1] - report[0]).signum();
        
        while i < n && asc_desc != 0 {
            // calculate absolute difference between elements
            let diff = report[i] - report[i - 1];
            if diff.signum() == asc_desc && diff.abs() >= 1 && diff.abs() <= 3 {
                i += 1;
            } else {
                break;
            }
            if i == n {
                n_safe += 1;
            }
        }
    }
    
    n_safe
}