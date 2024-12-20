// main.rs
// Day 3 - Mull It Over
// part 1 - find strict occurences of mul(X, Y) from string inputs and give out sum of such multiplication instructions
// part 2 - find and calculate mul() values only between instances of do() and don't(), always consider instances up until first don't() from start of string

use regex::Regex;
fn main() {
    let data_path = "./input/input.txt";
    let lines = parser(&data_path);
    let result = process(&lines);
    println!("sum of multiplications: {}", result);
    let result_conditional = process_conditional(&lines);
    println!("sum of conditional multiplications: {}", result_conditional);
}

fn parser(data_path: &str) -> Vec<String> {
    std::fs::read_to_string(data_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn process(lines: &Vec<String>) -> i64 {
    // pattern to match mul(X, Y)
    let pattern = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    // pattern to match numerical values
    let pattern_inner = Regex::new(r"[0-9]+").unwrap();
    let mut result: i64 = 0;

    for line in lines.iter() {
        // match for mul(X, Y) pattern
        for capture in pattern.captures_iter(line) {
            let mut product = 1;
            // integer match within mul(X, Y) string
            for num in pattern_inner.captures_iter(&capture[0]) {
                product = product * &num[0].parse::<i64>().unwrap();
            }
            result += product;
        }
    }
    result
}

fn process_conditional(lines: &Vec<String>) -> i64 {
    let mut product: i64 = 0;
    let dont = Regex::new(r"don't\(\)").unwrap();
    let mut mega_line: String = String::new();
    for line in lines {
        mega_line += line;
    }

    // find first don't() occurence
    let dont_ends: Vec<usize> = dont.find_iter(&mega_line)
    .map(|c| c.end())
    .collect();
    
    // process portion of string till first don't
    let split_start = &mega_line[..dont_ends[0]];
    product += process_string(split_start);

    // process slices (do() - don't()) of strings from first don't till last don't
    let do_dont = Regex::new(r"do\(\).*?don't\(\)").unwrap();
    let split_mid: &str = &mega_line[dont_ends[0]..];
    let do_dont_matches: Vec<(usize, usize)> = do_dont.find_iter(split_mid)
            .map(|c| (c.start(),c.end()))
            .collect();
    product += do_dont_matches.iter()
                    .map(|v| process_string(&split_mid[v.0..v.1]))
                    .sum::<i64>();

    // process end portion of string from last do()
    let split_end = &split_mid[do_dont_matches.last().unwrap().1..];
    // process any string after do() in the last split
    let do_matches: Vec<usize> = Regex::new(r"do\(\)").unwrap()
                    .find_iter(split_end)
                    .map(|c| c.start())
                    .collect();
    
    if do_matches.len() > 0 {
        let split = do_matches.last().unwrap();
        product += process_string(&split_end[*split..]);
    }

    product
}

fn process_string(s: &str) -> i64 {
    // pattern to match mul(X, Y)
    let pattern = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    // sum
    let mut result: i64 = 0;
    // pattern to match numerical values
    let pattern_inner = Regex::new(r"[0-9]+").unwrap();

    for capture in pattern.captures_iter(s) {
        // product
        let mut product = 1;
        // integer match within mul(X, Y) string
        for num in pattern_inner.captures_iter(&capture[0]) {
            product = product * &num[0].parse::<i64>().unwrap();
        }
        result += product;
    }
    
    result
}
