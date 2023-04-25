fn main() {
    println!("Hello, {}!", "rusty");
    binary_interpretation();
}

fn binary_interpretation() {
    //read file line by line
    //extract relevant data
    
    //per star generate reference points, and store in a vector
    //use galactic coordinates and geometry to relate stars to each other within an arbitrary angle
    //store the angular relationship between groups of 3 stars along their bright_star_number


    //format for firebase
}

//  fn new_star(bytes: Vec<u8>) -> Star {

// }

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