from typing import List
import re

def main():
    data_path  = "../input/input.txt"
    lines = parser(data_path)
    result = conditional_parse(lines)
    print(f"result: {result}")

def parser(data_path: str) -> List[str]:
    lines = []
    with open(data_path, "r") as f:
        for line in f.readlines():
            lines.append(line.strip())

    return lines

def conditional_parse(lines: List[str]) -> int:
    product = 0
    megaline = ""
    for line in lines:
        megaline += line

    # catch until first don't() match
    pattern_do = r"don't\(\)"
    split_start = ""
    for match in re.finditer(pattern_do, megaline):
        split_start = megaline[:match.span()[1]]
        break
    product += process_mul(split_start)

    # catch do()..don't() pairs
    split_rest = megaline[match.span()[1]:]
    pattern_do_dont = r"do\(\)(.*?(?=don't\(\)|$))"
    matches = []
    
    for match in re.finditer(pattern_do_dont, split_rest):
        matches.append(split_rest[match.span()[0]:match.span()[1]])
        
    for match in matches:
        product += process_mul(match)

    return product

def process_mul(s: str) -> int:
    pattern_mul = r"mul\([0-9]+,[0-9]+\)"
    result = 0

    for match in re.findall(pattern_mul, s):
        product = 1
        for num in re.findall(r"\d+", match):
            product *= int(num)
        result += product

    return result

if __name__ == "__main__":
    main()