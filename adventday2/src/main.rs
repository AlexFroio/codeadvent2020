use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("No file");
    let buf = BufReader::new(file);
    buf.lines()
    .map(|l| l.expect("Could not parse line"))
    .collect()
}

fn check_letter_location(line: &str, loc: Vec<usize>, l2find: char) -> bool {
    let mut res = false;
    let mut y = 0;
    for ch in line.chars() {
        if ch == l2find && (y == loc[0] - 1 || y == loc[1] - 1) {
            res = !res;
        }
        y += 1;
    }
    return res;
}

fn main() {
    //Part one
    let lines = lines_from_file("C:/Users/alexa/Documents/Code Projects/CodeAdvent2020/adventday2/src/passwords.txt");
    let linescpy = lines.clone();
    let mut y = 0;
    for line in lines {
        let spline: Vec<String> = line.split(" ").map(|s| s.to_string()).collect();
        let lim: Vec<usize> = spline[0].split("-").map(|i| i.parse().unwrap()).collect();
        let l2find = spline[1].chars().next().unwrap();
        let lmatch = spline[2].matches(l2find).count();

        if lmatch >= lim[0] && lmatch <= lim[1] {
            y += 1;
        }
    }
    let mut z = 0;
    for line in linescpy {
        let spline: Vec<String> = line.split(" ").map(|s| s.to_string()).collect();
        let loc: Vec<usize> = spline[0].split("-").map(|i| i.parse().unwrap()).collect();
        let l2find = spline[1].chars().next().unwrap();
        if check_letter_location(&spline[2], loc, l2find){
            z += 1;
        }
    }
    println!("file contains {}, and {} acceptable passwords", y, z);
}
