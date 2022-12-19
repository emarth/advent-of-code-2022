use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn priority(ch : u8) -> i32 {
    if 65 <= ch && ch <= 90 {
        return (ch - 38).into();
    } else {
        return (ch - 96).into();
    }
}
fn quicksort (arr : &mut [i32]) {
    if arr.len() >= 2 {
        // partition
        let pivot = arr[arr.len()-1];
        let mut i = 0;

        for j in 0..arr.len() {
            if arr[j] <= pivot {
                i += 1;
                let temp = arr[i-1];
                arr[i-1] = arr[j];
                arr[j] = temp;
            } 
        } // the -1 shift on i is because one wishes to start a position before the slice begins
        quicksort(&mut arr[..i-1]);
        quicksort(&mut arr[i..]);
    }
}

fn binary_search (arr : &mut [i32], target : i32) -> i32 {
    if arr.len() == 0 {
        return 0;
    }
    let mid = arr.len()/2;
    if arr[mid] < target {
        return binary_search(&mut arr[mid+1..], target);
    } if arr[mid] > target {
        return binary_search(&mut arr[..mid], target);
    } target
} // gives 0 if not found, target value if found

pub fn part1 (filename : &str) -> i32 {
    if let Ok(lines) = read_lines(filename) {
        let mut sum = 0;
        for line in lines {
            let bytes = line.unwrap().into_bytes(); //characters in line as bytes
            let half = bytes.len()/2;

            let mut comp1 : Vec<i32> = Vec::new();
            let mut comp2 : Vec<i32> = Vec::new();
            for i in 0..half {
                comp1.push(priority(bytes[i]));
            } for i in half..bytes.len() {
                comp2.push(priority(bytes[i]));
            } //translate to priorities

            let comp2 = &mut comp2[..];
            quicksort(comp2);
            for p in comp1 {
                let search = binary_search(comp2, p);
                if search != 0 {
                    sum += search;
                    break;
                }
            }
        }
        return sum;
    } 0
}

pub fn part2 (filename : &str) -> i32 {
    if let Ok(lines) = read_lines(filename) {
        let mut first : Vec<i32> = Vec::new();
        let mut middle : Vec<i32> = Vec::new();
        let mut last : Vec<i32> = Vec::new();
        let mut sum = 0;
        let mut modulus: u8 = 0;

        for line in lines { 
            let bytes = line.unwrap().into_bytes();
            if modulus == 0 {
                let l = &mut last[..];
                quicksort(l);
                let m = &mut middle[..];
                quicksort(m);
                // update sum
                for i in 0..first.len() {
                    let result_l = binary_search(l, first[i]);
                    let result_m = binary_search(m, first[i]);
                    if result_l != 0 && result_m != 0 {
                        sum += result_l;
                        break;
                    }
                }
                first.clear();
                middle.clear();
                last.clear();

                for byte in bytes {
                    first.push(priority(byte));
                }
            } else if modulus == 1 {
                for byte in bytes {
                    middle.push(priority(byte));
                }
            } else {
                for byte in bytes {
                    last.push(priority(byte));
                }
            }
            modulus = (modulus + 1)%3;
        }
        return sum;
    } 0
}