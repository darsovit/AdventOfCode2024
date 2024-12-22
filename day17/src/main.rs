use std::fs;
use day17::Day17;

fn main() {
    const DATAFILE: &str = "input.txt";
    let file_content = fs::read_to_string(DATAFILE);
    match file_content {
        Ok(line_content) => {
            let day = Day17::new(line_content.lines());
            println!("part1: {}", day.part1());
            println!("part2: {}", day.part2());
        },
        Err(e) => { println!("Error reading file: {}, {:?}", DATAFILE, e); }
    }
}

//    2nd octet can be 000, 001,
// Part 2                              A = 24 (011 010)
// 0  BST 4  => A % 8 -> B             B = 2 (010)
// 1  BXL 5  => B ^ 5 (101) -> B       B = 7 (111)
// 2  CDV 5  => A >> B -> C            C = 0 (000)
// 3  BXL 6  => B ^ 6 (110) -> B       B = 0 (110)
// 4  ADV 3  => A >> 3 -> A            A = 3
// 5  BXC 3  => B ^ C -> B             B = 0 (011)
// 6  OUT 5  => OUT(B%8)                   3                              ,0   (A=0,B=0,C=?), A!=0,B=3/11/19/...,
// 7  JNZ 0

// 16 loops through the program, on 16th one, A is 0, A is only changed against itself at step 4, always dividing by 8 (with trunc)
// At start of each iteration (backwards)
//
// A = 3, [24,25], [8,64), [64,512), []
// B =
//    (011)

// A (2nd round) = [24,32)

// (A % 8) ^ 011 ^ (A >> ((A % 8) ^ 101))
// 101 ^ 011 = 110
