use std::collections::VecDeque;
use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

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

pub struct Day09p2 {
    free_list: HashMap<usize, BinaryHeap<Reverse<usize>>>,
    file_list: HashMap<usize, Vec<usize>>,
    sector_list: Vec<Option<usize>>,
    largest_file_id: usize,
}

impl Day09p2 {
    pub fn new(line: &str) -> Self {
        let mut file_id: usize = 0;
        let mut disk_sector: usize = 0;
        let mut free_list = HashMap::<usize, BinaryHeap::<Reverse<usize>>>::new();
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
                free_list.entry(num_blocks).or_insert(BinaryHeap::new()).push(Reverse(disk_sector));
                for _i in 0..num_blocks {
                    sector_list.push(None);
                }
                disk_sector += num_blocks;
            }
            assert_eq!(disk_sector, sector_list.len());
        }
        Day09p2{free_list, file_list, sector_list, largest_file_id: file_id}
    }

    fn find_best_free_space(space_needed: usize, file_sectors_start: usize, free_list_sizes: &mut HashMap<usize, BinaryHeap<Reverse<usize>>>) -> Option<usize> {
        let mut furthest_left_fit_list_size: Option<(usize, usize)> = None;
        for spaces in space_needed..10 {
            if let Some(free_list) = free_list_sizes.get(&spaces) {
                if let Some(Reverse(left_most_of_size)) = free_list.peek() {
                    match furthest_left_fit_list_size {
                        None => { furthest_left_fit_list_size = Some((*left_most_of_size, spaces)); }
                        Some((left_spot, _)) => {
                            if *left_most_of_size < left_spot {
                                furthest_left_fit_list_size = Some((*left_most_of_size, spaces));
                            }
                        }
                    }
                    
                }
            }
        }
        match furthest_left_fit_list_size {
            None => None,
            Some((left_spot, size_free_list)) => { if file_sectors_start > left_spot { Some(size_free_list) } else { None } }
        }
    }
    fn try_move_file(file_id: usize, file_sectors: &mut Vec<usize>, sector_list: &mut Vec<Option<usize>>, free_list: &mut HashMap<usize, BinaryHeap<Reverse<usize>>>) {
        if let Some(free_entry_size) = Self::find_best_free_space(file_sectors.len(), file_sectors[0], free_list) {
            if let Some(Reverse(free_list_start)) = free_list.get_mut(&free_entry_size).unwrap().pop() {
                for sector in &file_sectors[..] {
                    if let Some(sector_file_id) = sector_list[*sector] {
                        assert_eq!(sector_file_id, file_id);
                        sector_list[*sector] = None;
                    }
                }
                for (index, sector) in file_sectors.iter_mut().enumerate() {
                    *sector = free_list_start + index;
                }
                if free_entry_size > file_sectors.len() {
                    let free_entry_left = free_entry_size - file_sectors.len();
                    let new_free_list_start = free_list_start + file_sectors.len();
                    free_list.entry(free_entry_left).or_insert(BinaryHeap::new()).push(Reverse(new_free_list_start));
                }
            }
        }
    }

    pub fn part2(&self) -> usize {
        let mut free_list = self.free_list.clone();
        let mut sector_list = self.sector_list.clone();
        let mut file_list = self.file_list.clone();

        for id in (1..self.largest_file_id).rev() {
            Self::try_move_file(id, &mut file_list.get_mut(&id).unwrap(), &mut sector_list, &mut free_list);
        }

        let mut checksum = 0;
        for file in file_list {
            let mut sector_id_sum = 0;
            for sector_id in file.1 {
                sector_id_sum += sector_id;
            }
            checksum += file.0 * sector_id_sum;
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

    #[test]
    fn sample_input_part2_is_2858() {
        let day = Day09p2::new(SAMPLE_LINE);
        assert_eq!(2858, day.part2());
    }
}
