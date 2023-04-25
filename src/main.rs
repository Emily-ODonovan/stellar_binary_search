use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;



fn main() {
    println!("Hello, {}!", "rusty");
    //Place each of the below into seperate rust modules (files)
    //generate reference points on first run --DONE!!!
    let star_at = file_to_stars();
    //genereates star_triples on second, file read can be excluded
    //--TODO generate_star_triples(star_at);
    //generates misc star data on third -> file
    let star_info = star_info_extractor(); 
}

fn file_to_stars() -> Vec<StarAt> {
    let mut star_at: Vec<StarAt> = Vec::new();
    //if result is Ok, opens the file and puts it into buffer "lines"
    if let Ok(lines) = read_lines("asuNoHeader.tsv") {
        // consumes lines and iterates over each line
        for line in lines {
            //if line sucessfully covertts to string it is consumed
            if let Ok(ip) = line {
                println!("{}", ip);
                let mut data = ip.split(";"); //returns a mutable iterator
                let star = StarAt {
                    bright_star_num: match data.nth(0) {
                        Some(x) => x.parse().unwrap(), //returns the bright_star_num
                        None => break, //breaks the loop if there is no data
                    },
                    galactic_long: match data.nth(13) {
                        Some(x) => x.parse().unwrap(), //returns the galactic_long
                        None => break, //breaks the loop if there is no data
                    },
                    galactic_lat: match data.nth(0) {
                        Some(x) => x.parse().unwrap(), //returns the galactic_long
                        None => break, //breaks the loop if there is no data
                    }
                };
                star_at.push(star);
            }
        }
    }
    star_at
}

fn star_info_extractor() -> Vec<Star> {  //TODO: set correct "nth" values
    let mut star_at: Vec<Star> = Vec::new();
    //if result is Ok, opens the file and puts it into buffer "lines"
    if let Ok(lines) = read_lines("asuNoHeader.tsv") {
        // consumes lines and iterates over each line
        for line in lines {
            //if line sucessfully covertts to string it is consumed
            if let Ok(ip) = line {
                println!("{}", ip);
                let mut data = ip.split(";"); //returns a mutable iterator
                let stars = Star {
                    bright_star_num: match data.nth(0) {
                        Some(x) => x.parse().unwrap(), //returns the bright_star_num
                        None => break, //breaks the loop if there is no data
                    },
                    name: match data.nth(0) {
                        Some(x) => x.parse().unwrap(), //returns the name
                        None => break, //breaks the loop if there is no data
                    },
                    durchmusterung: match data.nth(0) {
                        Some(x) => x.parse().unwrap(), //returns the durchmusterung
                        None => break, //breaks the loop if there is no data
                    },
                    sao: match data.nth(0) {
                        Some(x) => x.parse().unwrap(), //returns the sao
                        None => break, //breaks the loop if there is no data
                    },
                    fk5: match data.nth(0) {
                        Some(x) => x.parse().unwrap(), //returns the fk5
                        None => break, //breaks the loop if there is no data
                    },
                    visual_mag: match data.nth(0) {
                        Some(x) => x.parse().unwrap(), //returns the visual_mag
                        None => break, //breaks the loop if there is no data
                    },
                    visual_mag_code: match data.nth(0) {
                        Some(x) => x.parse().unwrap(), //returns the visual_mag
                        None => break, //breaks the loop if there is no data
                    }
                };
                star_at.push(stars);
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
    pub galactic_long: f64, //galactic longitude 5 bytes
    pub galactic_lat: f64 //galactic latitude
}


struct star_triple {
    pub bsm_1: u32, //bright star number 1
    pub bsm_2: u32, //bright star number 2
    pub bsm_3: u32, //bright star number 3
    pub angle: f64 //angle between the 3 stars
}