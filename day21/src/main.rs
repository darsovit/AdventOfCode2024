use std::fs;
use day21::Day21;

fn main() {
    const DATAFILE: &str = "input.txt";
    let file_content = fs::read_to_string(DATAFILE);
    match file_content {
        Ok(line_content) => {
            let day = Day21::new(line_content.lines());
            println!("part1: {}", day.part1());
            println!("part1: 108670 is too high");
            println!("part2: {}", day.part2());
            println!("part2: 149412069429784 is too high");
        },
        Err(e) => { println!("Error reading file: {}, {:?}", DATAFILE, e); }
    }
}
