

fn main() {
    println!("Hello, {}!", "rusty");
    binary_interpretation();
}

fn binary_interpretation() {
    let bytes = std::fs::read("bsc5.dat.gz");
    //error handling
    let bytes = match bytes {
        Ok(bytes) => bytes,
        Err(e) => panic!("Error reading file: {}", e),
    };

    print_star(new_star(bytes));
    
}

fn new_star(bytes: Vec<u8>) -> Star {
    let  a_star = Star {
        harvard_star_num: u32::from_be_bytes(bytes[0..4].try_into().unwrap()),
        name: String::from_utf8_lossy(&bytes[5..14]).to_string(),
        durchmusterung: String::from_utf8_lossy(&bytes[15..25]).to_string(),
        hdcn: read_le_u64(&bytes[26..31]),
        sao: read_le_u64(&bytes[32..37]),
        fk5: u32::from_be_bytes(bytes[38..41].try_into().unwrap()),
        infra_flag: bytes[42] == 'I' as u8,
        flag_ref: bytes[43],
        ads: read_le_u64(&bytes[45..49]),
        ads_components: u16::from_be_bytes(bytes[50..51].try_into().unwrap()),
        var_id: u64::from_be_bytes(bytes[52..60].try_into().unwrap()),
        hours: u16::from_be_bytes(bytes[76..77].try_into().unwrap()),
        minutes: u16::from_be_bytes(bytes[78..79].try_into().unwrap()),
        seconds: u32::from_be_bytes(bytes[80..83].try_into().unwrap()),
        sign: bytes[84],
        degrees: u16::from_be_bytes(bytes[85..86].try_into().unwrap()),
        arcminutes: u16::from_be_bytes(bytes[87..88].try_into().unwrap()),
        arcseconds: u16::from_be_bytes(bytes[89..90].try_into().unwrap()),
        galactic_long: u64::from_be_bytes(bytes[91..96].try_into().unwrap()),
        galactic_lat: u64::from_be_bytes(bytes[97..102].try_into().unwrap()),
        visual_mag: u64::from_be_bytes(bytes[103..107].try_into().unwrap()),
        visual_mag_code: bytes[108],
    };
    return a_star;
}

fn print_star(inp: Star) {
    println!("Harvard Star Number: {}", inp.harvard_star_num);
    println!("Name: {}", inp.name);
    println!("Durchmusterung: {}", inp.durchmusterung);
    println!("Henry Draper Catalog Number: {}", inp.hdcn);
    println!("SAO catalogue number: {}", inp.sao);
    println!("FK5 catalogue number: {}", inp.fk5);
    println!("Infrared source flag: {}", inp.infra_flag);
    println!("Infrared flag reference: {}", inp.flag_ref);
    println!("ADS catalogue reference: {}", inp.ads);
    println!("ADS number of components: {}", inp.ads_components);
    println!("Variable star identification: {}", inp.var_id);
    println!("Hours: {}", inp.hours);
    println!("Minutes: {}", inp.minutes);
    println!("Seconds: {}", inp.seconds);
    println!("Sign: {}", inp.sign);
    println!("Degrees: {}", inp.degrees);
    println!("Arcminutes: {}", inp.arcminutes);
    println!("Arcseconds: {}", inp.arcseconds);
    println!("Galactic longitude: {}", inp.galactic_long);
    println!("Galactic latitude: {}", inp.galactic_lat);
    println!("Visual magnitude: {}", inp.visual_mag);
    println!("Visual magnitude code: {}", inp.visual_mag_code);
}

//Converts byte slices smaller than 8 bytes to u64 with native (Big) Endian
fn read_le_u64(bytes: &[u8]) -> u64 {
    let mut result: u64 = 0;
    for byte in bytes.iter() {
        result = (result << 8) + u64::from(*byte);
    }
    result
}
struct Star {
    pub harvard_star_num: u32, //4 bytes long
    pub name: String, //10 bytes long
    pub durchmusterung: String, //10 bytes long
    pub hdcn: u64, //Henry Draper Catalog Number
    pub sao: u64, //SAO catalogue number
    pub fk5: u32, //FK5 catalogue number
    pub infra_flag: bool, //infrared source flag
    pub flag_ref: u8, //infrared flag reference
    // pub multi: bool, //multiple star flag
    pub ads: u64, //ADS catalogue reference
    pub ads_components: u16, //ADS number of components
    pub var_id: u64, //variable star identification
    //stip bytes 61-75 to Epoch2000 coordinates
    pub hours: u16,
    pub minutes: u16,
    pub seconds: u32,
    pub sign: u8, //don't know what this is
    pub degrees: u16, //degrees
    pub arcminutes: u16, //arcminutes
    pub arcseconds: u16, //arcseconds
    pub galactic_long: u64, //galactic longitude 5 bytes
    pub galactic_lat: u64, //galactic latitude
    pub visual_mag: u64, //visual magnitude
    pub visual_mag_code: u8, //visual magnitude code
    //pub uncertaincy_flag: u8, //uncertaincy flag
    //ignore the next 88 bytes to reach the next star
}