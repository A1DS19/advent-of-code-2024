use std::fs;

#[derive(Debug)]
struct DiskBlock {
    file_id: Option<usize>,
    position: usize,
}

fn parse_input(input: &str) -> Vec<u32> {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn create_disk_map(lengths: &[u32]) -> Vec<DiskBlock> {
    let mut disk = Vec::new();
    let mut current_pos = 0;
    let mut file_id = 0;

    for (i, &length) in lengths.iter().enumerate() {
        if i % 2 == 0 {
            // File blocks
            for _ in 0..length {
                disk.push(DiskBlock {
                    file_id: Some(file_id),
                    position: current_pos,
                });
                current_pos += 1;
            }
            file_id += 1;
        } else {
            // Free space
            for _ in 0..length {
                disk.push(DiskBlock {
                    file_id: None,
                    position: current_pos,
                });
                current_pos += 1;
            }
        }
    }
    disk
}

fn defragment(disk: &mut Vec<DiskBlock>) {
    let len = disk.len();
    for i in 0..len {
        if disk[i].file_id.is_none() {
            // Found free space, look for rightmost file to move
            for j in (i + 1..len).rev() {
                if let Some(id) = disk[j].file_id {
                    disk[i].file_id = Some(id);
                    disk[j].file_id = None;
                    break;
                }
            }
        }
    }
}

fn calculate_checksum(disk: &[DiskBlock]) -> usize {
    disk.iter()
        .enumerate()
        .filter_map(|(pos, block)| block.file_id.map(|id| pos * id))
        .sum()
}

// Add these new functions:

fn get_file_blocks(disk: &[DiskBlock], file_id: usize) -> Vec<usize> {
    disk.iter()
        .enumerate()
        .filter(|(_, block)| block.file_id == Some(file_id))
        .map(|(pos, _)| pos)
        .collect()
}

fn find_free_space(disk: &[DiskBlock], size_needed: usize) -> Option<usize> {
    let mut current_free = 0;
    let mut start_pos = 0;

    for (pos, block) in disk.iter().enumerate() {
        if block.file_id.is_none() {
            if current_free == 0 {
                start_pos = pos;
            }
            current_free += 1;
            if current_free >= size_needed {
                return Some(start_pos);
            }
        } else {
            current_free = 0;
        }
    }
    None
}

fn defragment_whole_files(disk: &mut Vec<DiskBlock>) {
    // Get highest file ID
    let max_file_id = disk
        .iter()
        .filter_map(|block| block.file_id)
        .max()
        .unwrap_or(0);

    // Process files in descending order
    for file_id in (0..=max_file_id).rev() {
        // Get positions of all blocks in this file
        let file_blocks = get_file_blocks(disk, file_id);
        if file_blocks.is_empty() {
            continue;
        }

        // Only look for free space before the first block of the file
        let first_block_pos = file_blocks[0];
        let file_size = file_blocks.len();

        // Find leftmost suitable free space
        if let Some(target_pos) = find_free_space(&disk[..first_block_pos], file_size) {
            // Move the entire file
            for (i, &old_pos) in file_blocks.iter().enumerate() {
                disk[target_pos + i].file_id = Some(file_id);
                disk[old_pos].file_id = None;
            }
        }
    }
}

// Modify main() to use new defragmentation:
fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let lengths = parse_input(&input);
    let mut disk = create_disk_map(&lengths);
    defragment_whole_files(&mut disk); // Changed this line
    let checksum = calculate_checksum(&disk);
    println!("Filesystem checksum: {}", checksum);
}
