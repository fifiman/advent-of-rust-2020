use regex::Regex;
use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

fn main() {
    let re = Regex::new(r"(\d+)-(\d+) (\w): (\w*)").expect("Bad regex");

    let mut valid_passwords_part_1 = 0;
    let mut valid_passwords_part_2 = 0;


    if let Ok(lines) = read_lines("./src/input.in") {
        for line in lines {
            if let Ok(line) = line {
                for cap in re.captures_iter(&line) {
                    let lower_count : usize = cap[1].parse().unwrap();
                    let upper_count : usize = cap[2].parse().unwrap();
                    let c : char = cap[3].parse().unwrap();

                    // ================= PART 1 =====================

                    let mut c_count = 0;

                    for ci in cap[4].chars() {
                        if c == ci {
                            c_count = c_count + 1;
                        }
                    }
                    if lower_count <= c_count && c_count <= upper_count {
                        valid_passwords_part_1 = valid_passwords_part_1 + 1;
                    }

                    // ================= PART 2 =====================

                    let char_at_lower = cap[4].chars().nth(lower_count - 1).unwrap();
                    let char_at_upper = cap[4].chars().nth(upper_count - 1).unwrap();

                    if (c == char_at_lower) ^ (c == char_at_upper) {
                        valid_passwords_part_2 = valid_passwords_part_2 + 1;
                    }
                }
            }
        }
    }

    println! ("Valid passwords for part 1: {}", valid_passwords_part_1);
    println! ("Valid passwords for part 2: {}", valid_passwords_part_2);

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
