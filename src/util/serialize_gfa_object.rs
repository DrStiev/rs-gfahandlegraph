use serde_json::Result;
use serde::{Serialize, Deserialize};
use time::Instant;

use crate::gfa::{gfa1::GFA, gfa2::GFA2};
use crate::util::save_file::*;
use bstr::BString;

// creates JSON OBject by serializing data structures
pub enum GFAType {
    GFABSTRING(GFA<BString>),
    GFAUSIZE(GFA<usize>),
    GFA2BSTRING(GFA2<BString>),
    GFA2USIZE(GFA2<usize>),
}

/// Function that convert a GFA object into a JSON file
#[inline]
pub fn to_json(gfa: GFAType, path: String) -> Result<()> {
    match gfa {
        GFAType::GFAUSIZE(g) => {
            let start = Instant::now();
            let json_file: String = serde_json::to_string(&g)?;
            match save_on_file(ObjectType::JSON(json_file), Some(path)) {
                Ok(_) => {
                    println!("Convert GFAObject to JSONObject: {:?}", start.elapsed());
                }
                Err(why) => println!("Error: {}", why),
            }
            Ok(())
        }
        GFAType::GFA2USIZE(g) => {
            let start = Instant::now();
            let json_file: String = serde_json::to_string(&g)?;
            match save_on_file(ObjectType::JSON(json_file), Some(path)) {
                Ok(_) => {
                    println!("Convert GFAObject to JSONObject: {:?}", start.elapsed());
                }
                Err(why) => println!("Error: {}", why),
            }
            Ok(())
        }
        GFAType::GFABSTRING(g) => {
            let start = Instant::now();
            let json_file: String = serde_json::to_string(&g)?;
            match save_on_file(ObjectType::JSON(json_file), Some(path)) {
                Ok(_) => {
                    println!("Convert GFAObject to JSONObject: {:?}", start.elapsed());
                }
                Err(why) => println!("Error: {}", why),
            }
            Ok(())
        }
        GFAType::GFA2BSTRING(g) => {
            let start = Instant::now();
            let json_file: String = serde_json::to_string(&g)?;
            match save_on_file(ObjectType::JSON(json_file), Some(path)) {
                Ok(_) => {
                    println!("Convert GFAObject to JSONObject: {:?}", start.elapsed());
                }
                Err(why) => println!("Error: {}", why),
            }
            Ok(())
        }
    }
}

/// Function that convert a GFA object into a  BINCODE file
#[inline]
pub fn to_bincode(gfa: GFAType, path: String) -> Result<()> {
    match gfa {
        GFAType::GFAUSIZE(g) => {
            let start = Instant::now();
            let bincode_file: Vec<u8> = bincode::serialize(&g).unwrap();
            match save_on_file(ObjectType::BINCODE(bincode_file), Some(path)) {
                Ok(_) => {
                    println!("Convert GFAObject to BINCODEObject: {:?}", start.elapsed());
                }
                Err(why) => println!("Error: {}", why),
            }
            Ok(())
        }
        GFAType::GFA2USIZE(g) => {
            let start = Instant::now();
            let bincode_file: Vec<u8> = bincode::serialize(&g).unwrap();
            match save_on_file(ObjectType::BINCODE(bincode_file), Some(path)) {
                Ok(_) => {
                    println!("Convert GFAObject to BINCODEObject: {:?}", start.elapsed());
                }
                Err(why) => println!("Error: {}", why),
            }
            Ok(())
        }
        GFAType::GFABSTRING(g) => {
            let start = Instant::now();
            let bincode_file: Vec<u8> = bincode::serialize(&g).unwrap();
            match save_on_file(ObjectType::BINCODE(bincode_file), Some(path)) {
                Ok(_) => {
                    println!("Convert GFAObject to BINCODEObject: {:?}", start.elapsed());
                }
                Err(why) => println!("Error: {}", why),
            }
            Ok(())
        }
        GFAType::GFA2BSTRING(g) => {
            let start = Instant::now();
            let bincode_file: Vec<u8> = bincode::serialize(&g).unwrap();
            match save_on_file(ObjectType::BINCODE(bincode_file), Some(path)) {
                Ok(_) => {
                    println!("Convert GFAObject to BINCODEObject: {:?}", start.elapsed());
                }
                Err(why) => println!("Error: {}", why),
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::*;
    use crate::util::to_gfa::*;

    #[test]
    //#[ignore]
    fn convert_big_gfa_to_json() {
        /* convert BIG GFA files to JSON it's pretty much useless
        Create graph from file: Duration { seconds: 529, nanoseconds: 323089300 }
        Convert graph to GFAObject: Duration { seconds: 150, nanoseconds: 838532100 }
        Convert GFAObject to JSONObject: Duration { seconds: 956, nanoseconds: 302402100 }
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
                match to_json(
                    GFAType::GFA2BSTRING(gfa2),
                    "./tests/output_files/ape-4-0.10b.json".to_string(),
                ) {
                    Ok(_) => (),
                    Err(why) => println!("Error: {}", why),
                }
            }
            Err(why) => println!("Error {}", why),
        }
    }

    #[test]
    #[ignore]
    fn convert_big_gfa_to_bincode() {
        /* convert BIG GFA files to JSON it's pretty much useless
        Create graph from file: Duration { seconds: 529, nanoseconds: 323089300 }
        Convert graph to GFAObject: Duration { seconds: 150, nanoseconds: 838532100 }
        Convert GFAObject to JSONObject: Duration { seconds: 956, nanoseconds: 302402100 }
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
                match to_bincode(
                    GFAType::GFA2BSTRING(gfa2),
                    "./tests/output_files/ape-4-0.10b.json".to_string(),
                ) {
                    Ok(_) => (),
                    Err(why) => println!("Error: {}", why),
                }
            }
            Err(why) => println!("Error {}", why),
        }
    }

    #[test]
    fn convert_medium_gfa_to_json() {
        /*
        Create graph from file: Duration { seconds: 1, nanoseconds: 260153500 }
        Convert graph to GFAObject: Duration { seconds: 0, nanoseconds: 191341000 }
        Convert GFAObject to JSONObject: Duration { seconds: 0, nanoseconds: 470087700 }
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
                match to_json(
                    GFAType::GFA2BSTRING(gfa2),
                    "./tests/output_files/test.json".to_string(),
                ) {
                    Ok(_) => (),
                    Err(why) => println!("Error: {}", why),
                }
            }
            Err(why) => println!("Error {}", why),
        }
    }

    #[test]
    fn convert_medium_gfa_to_bincode() {
        /*
        Create graph from file: Duration { seconds: 1, nanoseconds: 244970200 }
        Convert graph to GFAObject: Duration { seconds: 0, nanoseconds: 195652800 }
        Convert GFAObject to BINCODEObject: Duration { seconds: 0, nanoseconds: 64547100 }
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
                match to_bincode(
                    GFAType::GFA2BSTRING(gfa2),
                    "./tests/output_files/test.json".to_string(),
                ) {
                    Ok(_) => (),
                    Err(why) => println!("Error: {}", why),
                }
            }
            Err(why) => println!("Error {}", why),
        }
    }
}
