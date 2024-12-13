use std::collections::VecDeque;
use std::collections::HashMap;

pub struct Day09 {
    free_list: VecDeque<usize>,
    file_list: HashMap<usize, Vec<usize>>,
    sector_list: Vec<Option<usize>>,
}

fn determine_num_blocks(a_char: char) -> Option<usize> {
    match a_char {
        '0' => Some(0),
        '1' => Some(1),
        '2' => Some(2),
        '3' => Some(3),
        '4' => Some(4),
        '5' => Some(5),
        '6' => Some(6),
        '7' => Some(7),
        '8' => Some(8),
        '9' => Some(9),
        _ => None
    }
}

impl Day09 {
    pub fn new(line: &str) -> Self {
        let mut file_id: usize = 0;
        let mut disk_sector: usize = 0;
        let mut free_list = VecDeque::<usize>::new();
        let mut file_list = HashMap::<usize, Vec<usize>>::new();
        let mut sector_list = Vec::<Option<usize>>::new();

        for (index, a_char) in line.chars().enumerate() {
            let num_blocks = determine_num_blocks(a_char).unwrap();
            if index % 2 == 0 {
                let mut file_sector_list = Vec::<usize>::new();
                for sector_id in disk_sector..disk_sector+num_blocks {
                    file_sector_list.push(sector_id);
                }
                file_list.insert(file_id, file_sector_list);
                for _i in 0..num_blocks {
                    sector_list.push(Some(file_id));
                }
                file_id += 1;
                disk_sector += num_blocks;
            }
            else {
                for sector_id in disk_sector..disk_sector+num_blocks {
                    free_list.push_back(sector_id);
                    sector_list.push(None);
                }
                disk_sector += num_blocks;
            }
            assert_eq!(disk_sector, sector_list.len());
        }
        Day09{free_list, file_list, sector_list}
    }

    fn get_back_file_sector_id(sector_list: &mut Vec<Option<usize>>, free_list: &mut VecDeque<usize>) -> Option<(usize, usize)> {
        loop {
            let last_sector = sector_list.len() - 1;

            match sector_list.pop() {
                None => { return None; }
                Some(None) => {
                    let back_free = free_list.pop_back().unwrap();
                    assert_eq!(back_free, last_sector);
                },
                Some(Some(possible_file_id)) => {
                    return Some((possible_file_id, last_sector));
                }
            }
        }
    }
    fn find_sector_and_replace(sectors: &mut Vec<usize>, old_sector_id: usize, new_sector_id: usize) {
        for elem in sectors.iter_mut() {
            if *elem == old_sector_id { *elem = new_sector_id; }
        }
    }
    fn update_file_list(file_list: &mut HashMap<usize, Vec<usize>>, file_id: usize, old_sector_id: usize, new_sector_id: usize) {
        file_list.entry(file_id).and_modify(|v| { Self::find_sector_and_replace(v, old_sector_id, new_sector_id); });
    }

    pub fn part1(&self) -> usize {
        let mut free_list = self.free_list.clone();
        let mut sector_list = self.sector_list.clone();
        let mut file_list = self.file_list.clone();

        loop {
            match free_list.pop_front() {
                None => { break; }
                Some(sector_id) => {
                    assert_eq!(sector_list[sector_id], None);
                    let (back_sector_file_id, old_sector_id) = Self::get_back_file_sector_id(&mut sector_list, &mut free_list).unwrap();
                    Self::update_file_list(&mut file_list, back_sector_file_id, old_sector_id, sector_id);
                    sector_list[sector_id] = Some(back_sector_file_id);
                }
            }
        }

        let mut checksum = 0;
        for (index, sector) in sector_list.into_iter().enumerate() {
            if let Some(file_id) = sector {
                checksum += file_id * index;
            }
        }
        checksum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_LINE: &str = "2333133121414131402";
    #[test]
    fn sample_input_part1_is_1928() {
        let day = Day09::new(SAMPLE_LINE);
        assert_eq!(1928, day.part1());
    }
}
