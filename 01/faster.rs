use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use std::collections::HashSet;
use std::collections::HashMap;

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

    // ================= PART 1 =========================

    let mut map : HashSet<i32> = HashSet::new() ;
    for x in &vec {
        map.insert(*x);
    }

    for y in &vec {
        if map.contains(&(2020 - y)) {
            let x = 2020 - y;
            println! ("X={}, Y={}, X*Y={}", x, y, x*y);
            break;
        }
    }

    // ================= PART 2 =========================

    let mut pair_map : HashMap<i32, i32> = HashMap::new();

    for x in &vec {
        for y in &vec {
            pair_map.insert(x + y, x * y);
        }
    }
    for z in &vec {
        if pair_map.contains_key(&(2020 - z)) {
            let xy_sum = 2020 - z;
            let xy_prod = pair_map.get(&xy_sum).expect("[xy_sum] does not exist in [pair_map]");
            println! ("X+Y={}, Z = {}, X*Y={}, X*Y*Z={}", xy_sum, z, xy_prod, xy_prod*z);
            break;
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
