use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
struct DiskFile {
    id: u64,
    size: u64
}

#[derive(Debug, Clone)]
struct DiskBlock {
    files: Vec<DiskFile>,
    free_space: u64
}

#[derive(Debug)]
struct DiskMap {
    blocks: Vec<DiskBlock>,
    fit_map: [Vec<usize>; 10]
}

impl DiskFile {
    fn checksum_serialize(&self) -> impl Iterator<Item = u64> + '_ {
        (0..self.size).map(|_| self.id)
    }
}

impl Display for DiskBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for file in self.files.iter() {
            let identifier = format!("{}", file.id % 10);
            write!(f, "{}", identifier.repeat(file.size as usize))?
        }
        let blank = ".".repeat(self.free_space as usize);
        write!(f, "{}", blank)
    }
}

impl DiskBlock {
    fn insert(&mut self, file: DiskFile) {
        self.files.push(file);
        self.free_space -= file.size;
    }

    fn remove(&mut self) -> DiskFile {
        let file = self.files.remove(0);
        self.free_space += file.size;
        file
    }

    fn is_free(&self) -> bool {
        self.files.is_empty()
    }

    fn get_size(&self) -> usize {
        self.files.iter().fold(0, |x, y| x + y.size) as usize
    }

    fn checksum_serialize(&self) -> impl Iterator<Item = u64> + '_ {
        return self.files.iter().flat_map(|v| v.checksum_serialize()).chain((0..self.free_space).map(|_| 0));
    }
}

impl Display for DiskMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for block in self.blocks.iter() {
            write!(f, "{}", block)?
        }

        write!(f, "")
    }
}

impl DiskMap {
    fn compact(&mut self) {
        let mut source_pointer = self.blocks.len() - 1;

        while source_pointer > 0 {
            let source_block = &self.blocks[source_pointer];

            if !source_block.is_free() {
                let maybe_dest_pointer = self.fit_map[source_block.get_size() as usize].last();

                if maybe_dest_pointer.is_some() {
                    let &dest_pointer = maybe_dest_pointer.unwrap();

                    if dest_pointer < source_pointer {
                        self.transfer(source_pointer, dest_pointer);
                    }
                }
            }

            source_pointer -= 1;
        }
    }

    fn transfer(&mut self, source_pointer: usize, dest_pointer: usize) {
        let original_free_space = self.blocks[dest_pointer].free_space;
        let file = self.blocks[source_pointer as usize].remove();
        self.blocks[dest_pointer].insert(file);
        let new_free_space = self.blocks[dest_pointer].free_space;

        for fit_bucket_index in (new_free_space + 1)..=original_free_space {
            let clear_index = self.fit_map[fit_bucket_index as usize].iter().enumerate().rev().find(|(_, &v)| v == dest_pointer).unwrap().0;
            self.fit_map[fit_bucket_index as usize].remove(clear_index);
        }
    }

    fn checksum(&self) -> u64 {
        self.blocks.iter().flat_map(|v| v.checksum_serialize()).enumerate().map(|(i, id)| i as u64 * id).fold(0, |a, b| a + b)
    }
}

pub fn run(input_path: &str, part: i32) {
    if part <= 1 {
        part1(input_path);
    }
    else {
        part2(input_path);
    }
}

fn part1(input_path: &str) {
    let input_string = std::fs::read_to_string(input_path).unwrap();
    let mut disk_map = parse_input(input_string.as_str(), true);
    disk_map.compact();
    println!("{}", disk_map.checksum());
}

fn part2(input_path: &str) {
    let input_string = std::fs::read_to_string(input_path).unwrap();
    let mut disk_map = parse_input(input_string.as_str(), false);
    disk_map.compact();
    println!("{}", disk_map.checksum());
}

fn parse_input(input_string: &str, split_files: bool) -> DiskMap {
    let mut fit_map: [Vec<usize>; 10] = [vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![]];
    let mut blocks = vec![];

    for (i, c) in input_string.chars().enumerate() {
        let size = c.to_digit(10).unwrap() as u64;
        
        if i % 2 == 0 {
            if split_files {
                for _ in 0..size {
                    let file = DiskFile {
                        id: i as u64 / 2,
                        size: 1
                    };
                    blocks.push(DiskBlock {
                        files: vec![file],
                        free_space: 0
                    });
                }
            } else {
                let file = DiskFile {
                    id: i as u64 / 2,
                    size
                };
                blocks.push(DiskBlock {
                    files: vec![file],
                    free_space: 0
                });
            }
        }
        else {
            if split_files {
                for _ in 0..size {
                    blocks.push(DiskBlock {
                        files: vec![],
                        free_space: 1
                    }); 

                    fit_map[1].push(blocks.len() - 1);
                }        
            } else {
                blocks.push(DiskBlock {
                    files: vec![],
                    free_space: size
                });

                for fit_bucket in 1..=size {
                    fit_map[fit_bucket as usize].push(blocks.len() - 1);
                }
            }
        }
    }

    for fit_bucket in 1..10 {
        fit_map[fit_bucket].reverse();
    }

    DiskMap {
        blocks,
        fit_map
    }
}