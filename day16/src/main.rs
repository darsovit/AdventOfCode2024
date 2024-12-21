use std::fs;
use day16::Day16;

fn main() {
    const DATAFILE: &str = "input.txt";
    let file_content = fs::read_to_string(DATAFILE);
    match file_content {
        Ok(line_content) => {
            let day = Day16::new(line_content.lines());
            println!("part1: {}", day16::part1(&day));
            //println!("part2: {}", day.part2());
        },
        Err(e) => { println!("Error reading file: {}, {:?}", DATAFILE, e); }
    }
}
