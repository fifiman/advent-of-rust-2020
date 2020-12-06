use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

fn main() {
    let mut vec : Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines("./input.in") {
        for line in lines {
            if let Ok(line) = line {
                let my_int : i32 = line.parse().unwrap();
                vec.push(my_int);
            }
        }
    }

    println! ("Vec len: {}", vec.len());

    for x in &vec {
        for y in &vec {
            for z in &vec {
                if x + y + z == 2020 {
                    println! ("X={},Y={},Z={}, X*Y*Z={}", x, y, z, x*y*z);
                }
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
