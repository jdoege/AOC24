use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
#[macro_use] extern crate scan_fmt;

fn main() {
    let mut v1:Vec<i32> = Vec::new();
    let mut v2:Vec<i32> = Vec::new();
    let mut cmap: HashMap<i32,i32> = HashMap::new();
    if let Ok(lines) = read_lines("./aoc24_1_2-input.txt") {
        for line in lines.flatten() {
            if let Ok((l,r)) = scan_fmt!( &line, "{d}  {d}", i32, i32) {
                v1.push(l);
                v2.push(r);
            }
        }
        v1.sort();
        v2.sort();

        for p in v2.iter() {
            let c = cmap.entry(*p).or_insert(0);
            *c += 1;
        }
        let mut sim:i32  = 0;
        for p in v1.iter() {
            if cmap.contains_key(p) {
                sim += cmap.get(p).unwrap()*p;
            }
        }
        println!("{}", sim);
    }
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
