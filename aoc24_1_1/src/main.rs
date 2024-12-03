use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
#[macro_use] extern crate scan_fmt;

fn main() {
    let mut v1:Vec<i32> = Vec::new();
    let mut v2:Vec<i32> = Vec::new();
    let mut v3:Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines("./aoc24_1_1-input.txt") {
        for line in lines.flatten() {
            if let Ok((l,r)) = scan_fmt!( &line, "{d}  {d}", i32, i32) {
                v1.push(l);
                v2.push(r);
            }
        }
        v1.sort();
        v2.sort();
        for p in v1.iter().zip(v2.iter()) {
            let (l,r) = p;
            v3.push((l-r).abs());
        }
        let dist:i32 = v3.iter().sum();
        println!("{}", dist);
    }
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
