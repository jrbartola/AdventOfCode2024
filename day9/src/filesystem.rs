pub struct FileSystem {
    pub space_allocation: Vec<i16>,
}

enum InsertionMode {
    Space,
    File,
}

impl FileSystem {
    pub fn new(line: &str) -> Self {
        Self {
            space_allocation: Self::expand_allocation(line.as_bytes()),
        }
    }

    pub fn move_files_left(&mut self) -> () {
        let mut left_ptr = 0usize;
        let mut right_ptr = self.space_allocation.len() - 1;

        while left_ptr < right_ptr {
            while self.space_allocation[left_ptr] != -1 {
                left_ptr += 1;
            }

            while self.space_allocation[right_ptr] == -1 {
                right_ptr -= 1;
            }

            if left_ptr >= right_ptr {
                break;
            }

            self.space_allocation.swap(left_ptr, right_ptr);
        }
    }

    pub fn move_whole_files_left(&mut self) -> () {
        let mut left_ptr = 0usize;
        let mut right_ptr = self.space_allocation.len() - 1;

        let mut file_id_to_move = self.space_allocation[right_ptr];
        let mut right_ptr_start = right_ptr;

        while left_ptr < right_ptr {
            right_ptr_start = right_ptr;
            while right_ptr_start > 0 && self.space_allocation[right_ptr_start] == file_id_to_move {
                right_ptr_start -= 1;
            }

            right_ptr_start += 1;

            if right_ptr < right_ptr_start {
                break;
            }

            let file_size = right_ptr - right_ptr_start + 1;

            let mut found_valid_size = false;
            let mut inner_left_ptr = left_ptr;
            let mut left_ptr_end = left_ptr;

            // Finds a contiguous region of space at least `file_size` long
            loop {
                if self.space_allocation[left_ptr_end] == -1 {
                    while self.space_allocation[left_ptr_end] == -1 {
                        left_ptr_end += 1;
                    }
                } else {
                    while self.space_allocation[inner_left_ptr] != -1 {
                        inner_left_ptr += 1;
                    }
                    left_ptr_end = inner_left_ptr;
                }

                if left_ptr_end - inner_left_ptr >= file_size {
                    found_valid_size = true;
                    break;
                } else if left_ptr_end >= right_ptr {
                    break;
                } else {
                    inner_left_ptr = left_ptr_end;
                }
            }

            if found_valid_size {
                // Swap contiguous regions
                for i in 0..file_size {
                    self.space_allocation
                        .swap(inner_left_ptr + i, right_ptr_start + i);
                }
            }

            right_ptr = right_ptr_start - 1;

            while right_ptr > 0 && self.space_allocation[right_ptr] == -1 {
                right_ptr -= 1;
            }

            file_id_to_move = self.space_allocation[right_ptr];
        }
    }

    pub fn get_checksum(&self) -> u64 {
        self.space_allocation
            .iter()
            .enumerate()
            .fold(0, |acc, (pos, &file_id)| {
                if file_id == -1 {
                    return acc;
                }

                acc + (pos as u64) * (file_id as u64)
            })
    }

    fn expand_allocation(chars: &[u8]) -> Vec<i16> {
        let mut allocation = Vec::new();
        let mut insertion_mode = InsertionMode::File;
        let mut file_id: i16 = 0;

        for c in chars {
            // Map ASCII value to actual number
            let actual_block_len = *c - b'0';

            match insertion_mode {
                InsertionMode::Space => {
                    for _ in 0..actual_block_len {
                        allocation.push(-1);
                    }
                    insertion_mode = InsertionMode::File;
                }
                InsertionMode::File => {
                    for _ in 0..actual_block_len {
                        allocation.push(file_id);
                    }

                    file_id += 1;
                    insertion_mode = InsertionMode::Space;
                }
            }
        }

        allocation
    }
}
