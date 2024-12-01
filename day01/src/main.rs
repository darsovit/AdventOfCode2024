use std::fs;
use day01::Day01;
use day01::Day01p2;

fn main() {
    const DATAFILE: &str = "input.txt";
    let file_content = fs::read_to_string(DATAFILE);
    match file_content {
        Ok(line_content) => {
            {
                let mut day01 = Day01::new(line_content.lines());
                println!("{}", day01.run());
            }
            {
                let day01p2 = Day01p2::new(line_content.lines());
                println!("{}", day01p2.run());
            }
        },
        Err(e) => { println!("Error reading file: {}, {:?}", DATAFILE, e); }
    }
}
