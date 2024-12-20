use std::fs;
use day15::Day15;

fn main() {
    const DATAFILE: &str = "input.txt";
    let file_content = fs::read_to_string(DATAFILE);
    match file_content {
        Ok(line_content) => {
            let day = Day15::new(line_content.lines());
            println!("part1: {}", day.part1());
            let day = Day15::new_part2(line_content.lines());
            println!("part2: {}", day.part2());
            println!("part2: 1457559 is too low");
        },
        Err(e) => { println!("Error reading file: {}, {:?}", DATAFILE, e); }
    }
}
