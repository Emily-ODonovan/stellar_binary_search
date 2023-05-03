use firestore::*;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use tokio;

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

    env::set_var("FIRESTORE_EMULATOR_HOST","127.0.0.1:8080");


    //genereates star_triples on second, file read can be excluded
    //--TODO generate_star_triples(star_at);

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

    let my_star: Star = Star {
        bright_star_num: 1,
        name: "test".to_string(),
        durchmusterung: "test".to_string(),
        sao: 1,
        fk5: 1,
        galactic_long: 1.0,
        galactic_lat: 1.0,
    };

    let db = connect_to_db(my_star.clone());
}

//TODO send a goofy collection of data to the database
#[tokio::main]
async fn connect_to_db(my_star: Star) -> Result<FirestoreDb, Box<dyn std::error::Error>> {
    let google_project_id: String = env::var("gpi").unwrap();
    println!("Connecting to database...");
    println!("Project ID: {}", google_project_id);

    // let db = FirestoreDb::new(google_project_id).await?;
    let db = FirestoreDb::with_options_token_source(
        FirestoreDbOptions::new(google_project_id),
        gcloud_sdk::GCP_DEFAULT_SCOPES.clone(),
        gcloud_sdk::TokenSourceType::File("key.json".into())
    ).await?;

    println!("Connected to database!!!");

    let object_returned: Star = db
        .fluent()
        .insert()
        .into("testCollection")
        .document_id(&my_star.bright_star_num.to_string())
        .object(&my_star)
        .execute()
        .await?;

    println!("Inserted object: {:?}", object_returned);

    Ok(db)
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
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Star {
    pub bright_star_num: u32,
    pub name: String, //bayer or flamsteed designation
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

#[derive(Serialize, Deserialize, Debug, Clone)]
struct StarTriple {
    pub bsm_1: u32, //bright star number 1
    pub bsm_2: u32, //bright star number 2
    pub bsm_3: u32, //bright star number 3
    pub angle: f64, //angle between the 3 stars
}
