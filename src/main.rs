use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;



fn main() {
    println!("Hello, {}!", "rusty");
    //Place each of the below into seperate rust modules (files)
    //generate reference points on first run --DONE!!!
    let star_at: Vec<StarAt> = file_to_stars();
    star_at.iter().for_each(|x| println!("{} {} {}", x.bright_star_num, x.galactic_long, x.galactic_lat));
    //genereates star_triples on second, file read can be excluded
    //--TODO generate_star_triples(star_at);
    //generates misc star data on third -> file
    let star_info = star_info_extractor();
    star_info.iter().for_each(|x| println!("{} {} {} {} {} {} {}", x.bright_star_num, x.name, x.durchmusterung, x.sao, x.fk5, x.visual_mag, x.visual_mag_code));
}

fn file_to_stars() -> Vec<StarAt> {
    let mut stars_at: Vec<StarAt> = Vec::new();
    //if result is Ok, opens the file and puts it into buffer "lines"
    if let Ok(lines) = read_lines("asuNoHeader.tsv") {
        // consumes lines and iterates over each line
        for line in lines {
            //will always run until end of file
            if let Ok(ip) = line {
                let star = stars_or_bust(ip);
                //catches any errors
                if star.is_ok() {
                    stars_at.push(star.unwrap());
                }
            }
        }
    }
    stars_at
}

fn stars_or_bust(line: String) -> Result<StarAt, Box<dyn std::error::Error>> {
    let mut data: std::str::Split<&str> = line.split(";"); //returns a mutable iterator
    let star: StarAt = StarAt { //ok or only exists to catch a theoretical error in the case of exceeding the iterator and takes place before any real errors can occur
        bright_star_num: data.nth(0).ok_or("")?.to_string().trim().parse::<u32>().unwrap(),
        galactic_long: data.nth(13).ok_or("")?.to_string().trim().parse().unwrap_or(0.0), //0.0 will only occur in absense of coordinates
        galactic_lat: data.nth(0).ok_or("")?.to_string().trim().parse().unwrap_or(0.0), 
    };
    Ok(star)
}

fn info_or_bust(line: String) -> Result<Star, Box<dyn std::error::Error>> {
    let mut data: std::str::Split<&str> = line.split(";"); //returns a mutable iterator
    let stars: Star = Star {
        bright_star_num: data.nth(0).ok_or("")?.to_string().trim().parse::<u32>().unwrap(),
        name: data.nth(0).ok_or("")?.to_string(),
        durchmusterung: data.nth(0).ok_or("")?.to_string(),
        sao: data.nth(1).ok_or("")?.to_string().trim().parse::<u64>().unwrap_or(0),
        fk5: data.nth(0).ok_or("")?.to_string().trim().parse::<u32>().unwrap_or(0),
        galactic_long: data.nth(8).ok_or("")?.to_string().trim().parse().unwrap_or(0.0),
        galactic_lat: data.nth(0).ok_or("")?.to_string().trim().parse().unwrap_or(0.0),
        visual_mag: data.nth(0).ok_or("")?.to_string().trim().parse::<u64>().unwrap_or(0),
        visual_mag_code: data.nth(0).ok_or("")?.to_string().trim().parse::<u8>().unwrap_or(0)
    };
    Ok(stars)
}

fn star_info_extractor() -> Vec<Star> {  //TODO: set correct "nth" values
    let mut star_at: Vec<Star> = Vec::new();
    //if result is Ok, opens the file and puts it into buffer "lines"
    if let Ok(lines) = read_lines("asuNoHeader.tsv") {
        // consumes lines and iterates over each line
        for line in lines {
            //if line sucessfully covertts to string it is consumed
            if let Ok(ip) = line {
                let star = info_or_bust(ip);
                if star.is_ok() {
                    star_at.push(star.unwrap());
                }
            }
        }
    }
    star_at
}


// Returns an Iterator wrapped in a result to the Reader of the lines of the file.
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
    pub visual_mag_code: u8, //visual magnitude code
    pub galactic_long: f64, //galactic longitude 5 bytes
    pub galactic_lat: f64 //galactic latitude

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