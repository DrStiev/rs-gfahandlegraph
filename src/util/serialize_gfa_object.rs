use crate::gfa::{gfa1::GFA, gfa2::GFA2};
use bstr::BString;
use serde_json::Result;

// creates JSON or BINCODE OBject by serializing data structures
pub enum GFAType {
    GFABSTRING(GFA<BString>),
    GFAUSIZE(GFA<usize>),
    GFA2BSTRING(GFA2<BString>),
    GFA2USIZE(GFA2<usize>),
}

/// Function that convert a GFA object into a JSON file
#[inline]
pub fn to_json(gfa: GFAType) -> Result<String> {
    match gfa {
        GFAType::GFAUSIZE(g) => {
            let json_file: String = serde_json::to_string(&g)?;
            Ok(json_file)
        }
        GFAType::GFA2USIZE(g) => {
            let json_file: String = serde_json::to_string(&g)?;
            Ok(json_file)
        }
        GFAType::GFABSTRING(g) => {
            let json_file: String = serde_json::to_string(&g)?;
            Ok(json_file)
        }
        GFAType::GFA2BSTRING(g) => {
            let json_file: String = serde_json::to_string(&g)?;
            Ok(json_file)
        }
    }
}

/// Function that convert a GFA object into a  BINCODE file
#[inline]
pub fn to_bincode(gfa: GFAType) -> Result<Vec<u8>> {
    match gfa {
        GFAType::GFAUSIZE(g) => {
            let bincode_file: Vec<u8> = bincode::serialize(&g).unwrap();
            Ok(bincode_file)
        }
        GFAType::GFA2USIZE(g) => {
            let bincode_file: Vec<u8> = bincode::serialize(&g).unwrap();
            Ok(bincode_file)
        }
        GFAType::GFABSTRING(g) => {
            let bincode_file: Vec<u8> = bincode::serialize(&g).unwrap();
            Ok(bincode_file)
        }
        GFAType::GFA2BSTRING(g) => {
            let bincode_file: Vec<u8> = bincode::serialize(&g).unwrap();
            Ok(bincode_file)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::*;
    use crate::util::save_file::*;
    use crate::util::to_gfa::*;
    use time::Instant;

    fn convert_big_gfa_to_json() {
        /*
         */
        let start = Instant::now();
        let parser: Parser = Parser::new();
        let mut gfa2: GFA2<BString> = GFA2::new();
        match parser.parse_file_to_graph("./tests/big_files/ape-4-0.10b.gfa2") {
            Ok(g) => {
                println!("Create graph from file: {:?}", start.elapsed());
                let start = Instant::now();
                gfa2 = to_gfa2(&g);
                println!("Convert graph to GFAObject: {:?}", start.elapsed());
                let start = Instant::now();
                match to_json(GFAType::GFA2BSTRING(gfa2)) {
                    Ok(j) => {
                        println!("Convert GFAObject to JSONObject: {:?}", start.elapsed());
                        let start = Instant::now();
                        match save_on_file(
                            ObjectType::JSON(j),
                            Some("./tests/output_files/ape-4-0.10b.gfa2.json".to_string()),
                        ) {
                            Ok(_) => println!("Save JSONObject to file: {:?}", start.elapsed()),
                            Err(why) => println!("Error: {}", why),
                        }
                    }
                    Err(why) => println!("Error: {}", why),
                }
            }
            Err(why) => println!("Error {}", why),
        }
    }

    fn convert_big_gfa_to_bincode() {
        /*
         */
        let start = Instant::now();
        let parser: Parser = Parser::new();
        let mut gfa2: GFA2<BString> = GFA2::new();
        match parser.parse_file_to_graph("./tests/big_files/ape-4-0.10b.gfa2") {
            Ok(g) => {
                println!("Create graph from file: {:?}", start.elapsed());
                let start = Instant::now();
                gfa2 = to_gfa2(&g);
                println!("Convert graph to GFAObject: {:?}", start.elapsed());
                let start = Instant::now();
                match to_bincode(GFAType::GFA2BSTRING(gfa2)) {
                    Ok(b) => {
                        println!("Convert GFAObject to BINCODEObject: {:?}", start.elapsed());
                        let start = Instant::now();
                        match save_on_file(
                            ObjectType::BINCODE(b),
                            Some("./tests/output_files/ape-4-0.10b.gfa2.txt".to_string()),
                        ) {
                            Ok(_) => println!("Save BINCODEObject to file: {:?}", start.elapsed()),
                            Err(why) => println!("Error: {}", why),
                        }
                    }
                    Err(why) => println!("Error: {}", why),
                }
            }
            Err(why) => println!("Error {}", why),
        }
    }

    #[test]
    //#[ignore]
    fn big_files() {
        convert_big_gfa_to_bincode();
        convert_big_gfa_to_json();
    }

    #[test]
    fn convert_medium_gfa_to_json() {
        /*
        Create graph from file: Duration { seconds: 1, nanoseconds: 243468200 }
        Convert graph to GFAObject: Duration { seconds: 0, nanoseconds: 192942500 }
        Convert GFAObject to JSONObject: Duration { seconds: 0, nanoseconds: 212035100 }
        Save JSONObject to file: Duration { seconds: 0, nanoseconds: 49159200 }
        */
        let start = Instant::now();
        let parser: Parser = Parser::new();
        let mut gfa2: GFA2<BString> = GFA2::new();
        match parser.parse_file_to_graph("./tests/big_files/test.gfa2") {
            Ok(g) => {
                println!("Create graph from file: {:?}", start.elapsed());
                let start = Instant::now();
                gfa2 = to_gfa2(&g);
                println!("Convert graph to GFAObject: {:?}", start.elapsed());
                let start = Instant::now();
                match to_json(GFAType::GFA2BSTRING(gfa2)) {
                    Ok(j) => {
                        println!("Convert GFAObject to JSONObject: {:?}", start.elapsed());
                        let start = Instant::now();
                        match save_on_file(ObjectType::JSON(j), None) {
                            Ok(_) => println!("Save JSONObject to file: {:?}", start.elapsed()),
                            Err(why) => println!("Error: {}", why),
                        }
                    }
                    Err(why) => println!("Error: {}", why),
                }
            }
            Err(why) => println!("Error {}", why),
        }
    }

    #[test]
    fn convert_medium_gfa_to_bincode() {
        /*
        Create graph from file: Duration { seconds: 1, nanoseconds: 234540300 }
        Convert graph to GFAObject: Duration { seconds: 0, nanoseconds: 193081300 }
        Convert GFAObject to BINCODEObject: Duration { seconds: 0, nanoseconds: 33020400 }
        Save BINCODEObject to file: Duration { seconds: 0, nanoseconds: 179276200 }
        */
        let start = Instant::now();
        let parser: Parser = Parser::new();
        let mut gfa2: GFA2<BString> = GFA2::new();
        match parser.parse_file_to_graph("./tests/big_files/test.gfa2") {
            Ok(g) => {
                println!("Create graph from file: {:?}", start.elapsed());
                let start = Instant::now();
                gfa2 = to_gfa2(&g);
                println!("Convert graph to GFAObject: {:?}", start.elapsed());
                let start = Instant::now();
                match to_bincode(GFAType::GFA2BSTRING(gfa2)) {
                    Ok(b) => {
                        println!("Convert GFAObject to BINCODEObject: {:?}", start.elapsed());
                        let start = Instant::now();
                        match save_on_file(ObjectType::BINCODE(b), None) {
                            Ok(_) => println!("Save BINCODEObject to file: {:?}", start.elapsed()),
                            Err(why) => println!("Error: {}", why),
                        }
                    }
                    Err(why) => println!("Error: {}", why),
                }
            }
            Err(why) => println!("Error {}", why),
        }
    }
}
