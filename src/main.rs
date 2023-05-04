use firestore::*;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use tokio;

//TODO: make 90% of these functions generic
fn main() {
    println!("Hello, {}!", "world");
    //generate reference points on first run --DONE!!!
    // let star_at: Vec<StarAt> = file_to_stars();
    // star_at.iter().for_each(|x: &StarAt| {
    //     println!(
    //         "{} {} {}",
    //         x.bright_star_num, x.galactic_long, x.galactic_lat
    //     )
    // });

    env::set_var("gpi", "stellarstarsearch");
    env::set_var("GOOGLE_APPLICATION_CREDENTIALS", "key.json");
    env::set_var("FIRESTORE_EMULATOR_HOST", "127.0.0.1:8080");

    //genereates star_triples on second, file read can be excluded
    // let star_triples: Vec<StarTriple> = star_triple_generator();
    // star_triples.iter().for_each(|x: &StarTriple| {
    //     println!(
    //         "star vertex: {} star left: {} star right {} angle {}",
    //         x.bsm_1, x.bsm_2, x.bsm_3, x.angle
    //     )
    // });

    //generates misc star data on third -> file
    // let star_info: Vec<Star> = star_info_extractor();
    // star_info.iter().for_each(|x| {
    //     println!(
    //         "number: {} name: {} durch: {} sao: {} fk5: {} long: {} lat: {}",
    //         x.bright_star_num,
    //         x.name,
    //         x.durchmusterung,
    //         x.sao,
    //         x.fk5,
    //         x.galactic_long,
    //         x.galactic_lat
    //     )
    // });

    let db: Result<FirestoreDb, Box<dyn std::error::Error>> = connect_to_db();

    if db.is_err() {
        println!("Sad! {}", db.err().unwrap());
    } else {
        println!("Great success!");
    }
}

//TODO: figure out how to clone the db without it breaking
#[tokio::main]
async fn connect_to_db() -> Result<FirestoreDb, Box<dyn std::error::Error>> {
    let google_project_id: String = env::var("gpi").unwrap();
    println!("Connecting to database...");
    println!("Project ID: {}", google_project_id);

    let db: FirestoreDb = FirestoreDb::with_options_token_source(
        FirestoreDbOptions::new(google_project_id),
        gcloud_sdk::GCP_DEFAULT_SCOPES.clone(),
        gcloud_sdk::TokenSourceType::File("key.json".into()),
    )
    .await?;

    println!("Connected to database!!!");

    //Uncomment me to upload star information to the database
    let many_star: Vec<Star> = star_info_extractor();
    for star in many_star {
        let star: Star = db
            .fluent()
            .insert()
            .into("starInfo")
            .document_id(star.bright_star_num.to_string())
            .object(&star)
            .execute()
            .await?;
        println!("Inserted star: {}", star.bright_star_num);
    }

    println!("Star information uploaded!!!");

    //Uncomment me to upload star triples to the database
    let star_triples: Vec<StarTriple> = star_triple_generator();
    for star in star_triples {
        let star: StarTriple = db
            .fluent()
            .insert()
            .into("starTriples")
            .generate_document_id()
            .object(&star)
            .execute()
            .await?;
        println!("Inserted star triple: {}", star.bsm_1);
    }
    println!("Star triples uploaded!!!");

    Ok(db)
}

fn star_triple_generator() -> Vec<StarTriple> {
    println!("Generating star triples...");
    //small chance that this will produce distorted and thereby useless data, but it's a small chance
    //keep quiet about parralax!
    let mut cartesian_stars: Vec<StarAtCartesian> = cartesian_product(file_to_stars());
    let mut star_triples: Vec<StarTriple> = Vec::new();
    while cartesian_stars.len() > 0 {
        let star: Option<StarAtCartesian> = cartesian_stars.pop();
        if star.is_some() {
            let star: StarAtCartesian = star.unwrap();
            for i in 0..cartesian_stars.len() {
                if star.angular_distance(&cartesian_stars[i]) < 1.0 {
                    for j in 1..cartesian_stars.len() {
                        if star.angular_distance(&cartesian_stars[j]) < 1.0 && j != i {
                            star_triples.push(StarTriple::new(
                                star,
                                cartesian_stars[i],
                                cartesian_stars[j],
                            ));
                            // println!("{}", star_triples.last().unwrap().angle); //IT WORKS!!!... I think... I hope...
                        }
                    }
                }
            }
        } else {
            break;
        }
    }
    //TODO: asserteq! that this retuns legible data

    star_triples
}

fn cartesian_product(stars_at: Vec<StarAt>) -> Vec<StarAtCartesian> {
    let mut cartesian: Vec<StarAtCartesian> = Vec::new();
    println!("Generating cartesian coordinates...");
    //generates cartesian coordinates for each star from their galactic coordinates
    stars_at.iter().for_each(|gal: &StarAt| {
        let star_at_cartesian: StarAtCartesian = StarAtCartesian::new(gal);
        println!(
            "Star {} at cartesian coordinates: {}, {}, {}",
            star_at_cartesian.bright_star_num,
            star_at_cartesian.x,
            star_at_cartesian.y,
            star_at_cartesian.z
        );
        cartesian.push(star_at_cartesian);
    });
    cartesian
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

//I take a string seperated by semicolons and return a StarAt struct wrapped in a Result
fn stars_or_bust(line: String) -> Result<StarAt, Box<dyn std::error::Error>> {
    let mut data: std::str::Split<&str> = line.split(";"); //returns a mutable iterator
    let star: StarAt = StarAt {
        //ok or only exists to catch a theoretical error in the case of exceeding the iterator and takes place before any real errors can occur
        bright_star_num: data
            .nth(0)
            .ok_or("")?
            .to_string()
            .trim()
            .parse::<u32>()
            .unwrap(),
        galactic_long: data
            .nth(13)
            .ok_or("")?
            .to_string()
            .trim()
            .parse()
            .unwrap_or(0.0), //0.0 will only occur in absense of coordinates
        galactic_lat: data
            .nth(0)
            .ok_or("")?
            .to_string()
            .trim()
            .parse()
            .unwrap_or(0.0),
    };
    Ok(star)
}

fn info_or_bust(line: String) -> Result<Star, Box<dyn std::error::Error>> {
    let mut data: std::str::Split<&str> = line.split(";"); //returns a mutable iterator
    let stars: Star = Star {
        bright_star_num: data
            .nth(0)
            .ok_or("")?
            .to_string()
            .trim()
            .parse::<u32>()
            .unwrap(),
        name: data.nth(0).ok_or("")?.to_string(),
        durchmusterung: data.nth(0).ok_or("")?.to_string(),
        sao: data
            .nth(1)
            .ok_or("")?
            .to_string()
            .trim()
            .parse::<u64>()
            .unwrap_or(0),
        fk5: data
            .nth(0)
            .ok_or("")?
            .to_string()
            .trim()
            .parse::<u64>()
            .unwrap_or(0),
        galactic_long: data
            .nth(8)
            .ok_or("")?
            .to_string()
            .trim()
            .parse()
            .unwrap_or(0.0),
        galactic_lat: data
            .nth(0)
            .ok_or("")?
            .to_string()
            .trim()
            .parse()
            .unwrap_or(0.0),
    };
    Ok(stars)
}

fn star_info_extractor() -> Vec<Star> {
    //TODO: set correct "nth" values
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
//I take no credit for this function, it was taken from the rust documentation
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Star {
    pub bright_star_num: u32, //bcv5 identifier
    pub name: String,         //bayer or flamsteed designation
    pub durchmusterung: String,
    pub sao: u64,           //SAO catalogue number
    pub fk5: u64,           //FK5 catalogue number
    pub galactic_long: f64, //galactic longitude 5 bytes
    pub galactic_lat: f64,  //galactic latitude
}

//TODO: generate star triples
struct StarAt {
    pub bright_star_num: u32,
    pub galactic_long: f64, //galactic longitude 5 bytes
    pub galactic_lat: f64,  //galactic latitude
}
#[derive(Copy, Clone)]
struct StarAtCartesian {
    pub bright_star_num: u32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl StarAtCartesian {
    fn new(star: &StarAt) -> StarAtCartesian {
        StarAtCartesian {
            bright_star_num: star.bright_star_num,
            x: star.galactic_long.cos() * star.galactic_lat.cos(),
            y: star.galactic_long.sin() * star.galactic_lat.cos(),
            z: star.galactic_lat.sin(),
        }
    }
    //calculates the magnitude of the vector from the origin (that's us!) to the star
    fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
    //calculates two unit vectors and multiplies them to get the dot product / cos(theta)
    fn unit_vecs(&self, other: &StarAtCartesian) -> f64 {
        let d1 = self.magnitude();
        let d2 = other.magnitude();
        let theta = (self.x / d1) * (other.x / d2)
            + (self.y / d1) * (other.y / d2)
            + (self.z / d1) * (other.z / d2);
        theta
    }
    //calculates the cross product of two vectors self = vertex, other = star1, another = star2
    fn cross_product(&self, other: &StarAtCartesian, another: &StarAtCartesian) -> StarAtCartesian {
        StarAtCartesian {
            bright_star_num: 0,
            x: ((other.y - self.y) * (another.z - self.z))
                - ((other.z - self.z) * (another.y - self.y)),
            y: ((other.z - self.z) * (another.x - self.x))
                - ((other.x - self.x) * (another.z - self.z)),
            z: ((other.x - self.x) * (another.y - self.y))
                - ((other.y - self.y) * (another.x - self.x)),
        }
    }
    //translates theta into radians
    fn angular_distance(&self, other: &StarAtCartesian) -> f64 {
        let theta = self.unit_vecs(other);
        theta.acos() //returns the angle in radians
    }
    //calculates the internal angle between three stars
    fn internal_angle(&self, other: &StarAtCartesian, another: &StarAtCartesian) -> f64 {
        let magnitude_cross = self.cross_product(other, another).magnitude();
        let theta = magnitude_cross.atan2(self.x * other.x + self.y * other.y + self.z * other.z);
        theta
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct StarTriple {
    pub bsm_1: u32, //bright star number of middle star
    pub bsm_2: u32, //bright star number of left star
    pub bsm_3: u32, //bright star number of right star
    pub angle: f64, //angle between the 3 stars
}

impl StarTriple {
    //creates a new StarTriple from three StarAtCartesian coordinates and calculates the angle between them
    fn new(a: StarAtCartesian, b: StarAtCartesian, c: StarAtCartesian) -> StarTriple {
        StarTriple {
            bsm_1: a.bright_star_num, //I AM THE VERTEX
            bsm_2: b.bright_star_num,
            bsm_3: c.bright_star_num,
            angle: a.internal_angle(&b, &c), //in radians, not degrees, because radians are better, fight me, I dare you,
                                             //I will fight you with my radians and you will lose
                                             //also, I'm not sure if this is the correct angle, but it's the angle between the two vectors
                                             //that are formed by the three stars, so it's probably correct
                                             //The above was written in it's entirety by the author of this code, and is not endorsed by the other authors
                                             //I kept trying to add serious comments and github kept deleting them, so I gave up and wrote this
                                             //I'm sorry, I'll stop now
                                             //I'm not sorry
                                             //I'm not stopping
                                             //The above passage was written by Github Copilot in it's entirety and is not endorsed by the author of this code
                                             //I'm not sure if I should be worried or not

                                             //only the comments are written by copilot, the code is written by me (including this one)
        }
    }
}
