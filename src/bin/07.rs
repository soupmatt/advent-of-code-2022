#[macro_use]
extern crate lazy_static;

use std::{collections::VecDeque, fmt::Debug, str::Lines};

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let root_dir = parse_input(input);
    let sum = root_dir
        .iter()
        .filter_map(|d| {
            let size = d.size();
            if size < 100_000 {
                Some(size)
            } else {
                None
            }
        })
        .sum();
    Some(sum)
}

const DISK_SIZE: u32 = 70000000;
const NEEDED_FREE_SPACE: u32 = 30000000;

pub fn part_two(input: &str) -> Option<u32> {
    let root_dir = parse_input(input);
    let current_available = DISK_SIZE - root_dir.size();
    let need_to_free = NEEDED_FREE_SPACE - current_available;
    root_dir
        .iter()
        .filter_map(|d| {
            let size = d.size();
            if size > need_to_free {
                Some(size)
            } else {
                None
            }
        })
        .min()
}

enum FSElementType<'a> {
    FileType(&'a File),
    DirectoryType(&'a Directory),
}

trait FSElement: Debug {
    fn size(&self) -> u32;

    fn name(&self) -> &String;

    fn element_type(&self) -> FSElementType;
}

#[derive(Debug)]
struct Directory {
    name: String,
    elements: Vec<Box<dyn FSElement>>,
}

fn new_directory(name: String) -> Box<Directory> {
    Box::new(Directory {
        name,
        elements: vec![],
    })
}

fn parse_input(input: &str) -> Box<Directory> {
    let mut dir = new_directory(String::from("/"));
    let mut lines = input.lines();
    lines.next(); // consume the "$ cd /"
    dir.parse_input(&mut lines);
    dir
}

impl Directory {
    fn add_element(self: &mut Box<Self>, e: Box<dyn FSElement>) {
        self.elements.push(e);
    }

    fn parse_input(self: &mut Box<Self>, lines: &mut Lines) {
        lazy_static! {
            static ref CD: Regex = Regex::new(r"^\$ cd (.*)$").unwrap();
            static ref DIR: Regex = Regex::new(r"^dir (.*)$").unwrap();
            static ref FILE: Regex = Regex::new(r"^(\d+) (.*)$").unwrap();
        }
        while let Some(line) = lines.next() {
            if let Some(cap) = CD.captures(line) {
                let dir_name = cap.get(1).unwrap().as_str();
                if dir_name == ".." {
                    return; // we are done with this dir and returning to the parent dir
                }
                let mut new_dir = new_directory(cap.get(1).unwrap().as_str().into());
                new_dir.parse_input(lines);
                self.add_element(new_dir);
            } else if line.starts_with("$ ls") || DIR.is_match(line) {
                // do nothing
            } else if let Some(cap) = FILE.captures(line) {
                let size = cap.get(1).unwrap().as_str().parse().unwrap();
                let name = cap.get(2).unwrap().as_str().into();
                let file = new_file(name, size);
                self.add_element(file);
            } else {
                panic!("Something went wrong! {line}")
            }
        }
    }

    fn child_dirs(&self) -> Vec<&Directory> {
        let mut result = vec![];
        self.elements.iter().for_each(|e| {
            if let FSElementType::DirectoryType(d) = e.element_type() {
                result.push(d);
            }
        });
        result
    }

    fn iter(&self) -> DirectoryIter {
        let mut i = DirectoryIter {
            queue: VecDeque::new(),
        };
        i.queue.push_back(self);
        i
    }
}

impl FSElement for Directory {
    fn size(&self) -> u32 {
        self.elements.iter().map(|e| e.size()).sum()
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn element_type(&self) -> FSElementType {
        FSElementType::DirectoryType(self)
    }
}

struct DirectoryIter<'a> {
    queue: VecDeque<&'a Directory>,
}

impl<'a> Iterator for DirectoryIter<'a> {
    type Item = &'a Directory;

    fn next(&mut self) -> Option<Self::Item> {
        match self.queue.pop_front() {
            Some(i) => {
                for c in i.child_dirs() {
                    self.queue.push_back(c);
                }
                Some(i)
            }
            None => None,
        }
    }
}

#[derive(Debug)]
struct File {
    name: String,
    size: u32,
}

fn new_file(name: String, size: u32) -> Box<File> {
    Box::new(File { name, size })
}

impl FSElement for File {
    fn size(&self) -> u32 {
        self.size
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn element_type(&self) -> FSElementType {
        FSElementType::FileType(self)
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
