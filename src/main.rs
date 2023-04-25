use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;



fn main() {
    println!("Hello, {}!", "rusty");
    //Place each of the below into seperate rust modules (files)
    //generate reference points on first run
    let star_at = binary_interpretation();
    //genereates star_triples on second, file read can be excluded
    //generates misc star data on third
    
}

fn binary_interpretation() -> Vec<StarAt> {
    let star_at: Vec<StarAt> = Vec::new();
    //if result is Ok, opens the file and puts it into buffer "lines"
    if let Ok(lines) = read_lines("asuNoHeader.tsv") {
        // consumes lines and iterates over each line
        for line in lines {
            //if line sucessfully covertts to string it is consumed
            if let Ok(ip) = line {
                println!("{}", ip);
                ip.split(";");
            }
        }
    }
    star_at
}


//// Returns an Iterator wrapped in a result to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}



struct Star {
    pub bright_star_num: u32, 
    pub name: String, //bayer or flamsteed designation
    pub durchmusterung: String, 
    pub sao: u64, //SAO catalogue number
    pub fk5: u32, //FK5 catalogue number
    pub visual_mag: u64, //visual magnitude
    pub visual_mag_code: u8 //visual magnitude code
}

struct StarAt {
    pub bright_star_num: u32, 
    pub galactic_long: u64, //galactic longitude 5 bytes
    pub galactic_lat: u64 //galactic latitude
}


struct star_triple {
    pub bsm_1: u32, //bright star number 1
    pub bsm_2: u32, //bright star number 2
    pub bsm_3: u32, //bright star number 3
    pub angle: u64 //angle between the 3 stars
}