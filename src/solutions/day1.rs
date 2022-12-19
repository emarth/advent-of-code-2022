use std::collections::BinaryHeap;
use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn make_list(filename: &str, list: &mut Vec<i32>) {

    if let Ok(lines) = read_lines(filename) {
        let mut sum = 0;
        for line in lines {
            let l = line.unwrap();
            if l.len() >= 1 {
                sum = sum + l.parse::<i32>().unwrap(); // adds up calories for each elf
            } else {
                list.push(sum);
                sum = 0;
            }
        }
    }
}


pub fn part1(filename: &str) -> i32 {
    let mut list = vec![];
    make_list(filename, &mut list);
    let heap = BinaryHeap::from(list);
    *heap.peek().unwrap()
}

pub fn part2(filename: &str) -> i32 {
    let mut list = vec![];
    make_list(filename, &mut list);
    let mut heap = BinaryHeap::from(list);
    heap.pop().unwrap() + heap.pop().unwrap() + heap.pop().unwrap()
    // This is a special case k = 3. The heap is good here because in general Part 2 is O(n) + O(k*log n)
}