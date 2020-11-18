use serde_json::Result;
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
            let json_file = serde_json::to_string(&g)?;
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
            let json_file = serde_json::to_string(&g)?;
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
            let json_file = serde_json::to_string(&g)?;
            println!("Convert GFAObject to JSONObject: {:?}", start.elapsed());
            let start = Instant::now();
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
            let json_file = serde_json::to_string(&g)?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::*;
    use crate::util::to_gfa::*;

    #[test]
    #[ignore]
    fn convert_big_gfa_to_object() {
        /* convert BIG GFA files to JSON it's pretty much useless
        Create graph from file: Duration { seconds: 485, nanoseconds: 486852300 }
        Convert graph to GFAObject: Duration { seconds: 150, nanoseconds: 748793200 }
        Convert GFAObject to JSONObject: Duration { seconds: 960, nanoseconds: 804706000 }
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
    fn convert_medium_gfa_to_object() {
        /* convert GFA to JSON sucks!
        Create graph from file: Duration { seconds: 1, nanoseconds: 232650900 }
        Convert graph to GFAObject: Duration { seconds: 0, nanoseconds: 191577100 }
        Convert GFAObject to JSONObject: Duration { seconds: 0, nanoseconds: 258884800 }
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
}
