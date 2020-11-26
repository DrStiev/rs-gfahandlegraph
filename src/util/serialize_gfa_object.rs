use crate::gfa::{gfa1::GFA, gfa2::GFA2};
use serde_json::Result;

// creates JSON or BINCODE OBject by serializing data structures
pub enum GFAType {
    GFA(GFA),
    GFA2(GFA2),
}

/// Function that convert a
/// [`GFA`](file:///D:/GitHub/rs-gfahandlegraph/target/doc/gfahandlegraph/gfa/gfa1/struct.GFA.html),
/// [`GFA2`](file:///D:/GitHub/rs-gfahandlegraph/target/doc/gfahandlegraph/gfa/gfa2/struct.GFA2.html),
/// object into a [`JSON`](https://docs.serde.rs/serde_json/) file
pub fn to_json(gfa: GFAType) -> Result<String> {
    match gfa {
        GFAType::GFA(g) => {
            let json_file: String = serde_json::to_string(&g)?;
            Ok(json_file)
        }
        GFAType::GFA2(g) => {
            let json_file: String = serde_json::to_string(&g)?;
            Ok(json_file)
        }
    }
}

/// Function that convert a
/// [`GFA`](file:///D:/GitHub/rs-gfahandlegraph/target/doc/gfahandlegraph/gfa/gfa1/struct.GFA.html),
/// [`GFA2`](file:///D:/GitHub/rs-gfahandlegraph/target/doc/gfahandlegraph/gfa/gfa2/struct.GFA2.html),
/// object into a  [`BINCODE`](https://docs.rs/bincode/1.3.1/bincode/) file
pub fn to_bincode(gfa: GFAType) -> Result<Vec<u8>> {
    match gfa {
        GFAType::GFA(g) => {
            let bincode_file: Vec<u8> = bincode::serialize(&g).unwrap();
            Ok(bincode_file)
        }
        GFAType::GFA2(g) => {
            let bincode_file: Vec<u8> = bincode::serialize(&g).unwrap();
            Ok(bincode_file)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_file_to_graph;
    use crate::util::save_file::*;
    use crate::util::to_gfa::*;
    use time::Instant;

    fn convert_big_gfa_to_json() {
        /* NOT GOOD FOR BIG FILES, the dimension skyrockets and the conversion it's too slow
        Create graph from file: Duration { seconds: 510, nanoseconds: 82235500 }
        Convert graph to GFAObject: Duration { seconds: 149, nanoseconds: 505934600 }
        Convert GFAObject to JSONObject: Duration { seconds: 894, nanoseconds: 650416500 }
        Save JSONObject to file: Duration { seconds: 68, nanoseconds: 859425000 }
        */
        let start = Instant::now();
        let mut gfa2: GFA2 = GFA2::new();
        match parse_file_to_graph("./tests/big_files/ape-4-0.10b.gfa2") {
            Ok(g) => {
                println!("Create graph from file: {:?}", start.elapsed());
                let start = Instant::now();
                gfa2 = to_gfa2(&g);
                println!("Convert graph to GFAObject: {:?}", start.elapsed());
                let start = Instant::now();
                match to_json(GFAType::GFA2(gfa2)) {
                    Ok(j) => {
                        println!(
                            "Convert GFAObject to JSONObject: {:?}",
                            start.elapsed()
                        );
                        let start = Instant::now();
                        match save_on_file(
                            ObjectType::JSON(j),
                            Some(
                                "./tests/output_files/ape-4-0.10b.gfa2.json"
                                    .to_string(),
                            ),
                        ) {
                            Ok(_) => println!(
                                "Save JSONObject to file: {:?}",
                                start.elapsed()
                            ),
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
        /* GOOD FOR BIG FILES, mantain the same dimension and the conversion it's very fast
        Create graph from file: Duration { seconds: 526, nanoseconds: 791021400 }
        Convert graph to GFAObject: Duration { seconds: 151, nanoseconds: 572864700 }
        Convert GFAObject to BINCODEObject: Duration { seconds: 5, nanoseconds: 665128800 }
        Save BINCODEObject to file: Duration { seconds: 21, nanoseconds: 500720900 }
        */
        let start = Instant::now();
        let mut gfa2: GFA2 = GFA2::new();
        match parse_file_to_graph("./tests/big_files/ape-4-0.10b.gfa2") {
            Ok(g) => {
                println!("Create graph from file: {:?}", start.elapsed());
                let start = Instant::now();
                gfa2 = to_gfa2(&g);
                println!("Convert graph to GFAObject: {:?}", start.elapsed());
                let start = Instant::now();
                match to_bincode(GFAType::GFA2(gfa2)) {
                    Ok(b) => {
                        println!(
                            "Convert GFAObject to BINCODEObject: {:?}",
                            start.elapsed()
                        );
                        let start = Instant::now();
                        match save_on_file(
                            ObjectType::BINCODE(b),
                            Some(
                                "./tests/output_files/ape-4-0.10b.gfa2.bin"
                                    .to_string(),
                            ),
                        ) {
                            Ok(_) => println!(
                                "Save BINCODEObject to file: {:?}",
                                start.elapsed()
                            ),
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
    #[ignore]
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
        let mut gfa2: GFA2 = GFA2::new();
        match parse_file_to_graph("./tests/big_files/test.gfa2") {
            Ok(g) => {
                println!("Create graph from file: {:?}", start.elapsed());
                let start = Instant::now();
                gfa2 = to_gfa2(&g);
                println!("Convert graph to GFAObject: {:?}", start.elapsed());
                let start = Instant::now();
                match to_json(GFAType::GFA2(gfa2)) {
                    Ok(j) => {
                        println!(
                            "Convert GFAObject to JSONObject: {:?}",
                            start.elapsed()
                        );
                        let start = Instant::now();
                        match save_on_file(ObjectType::JSON(j), None) {
                            Ok(_) => println!(
                                "Save JSONObject to file: {:?}",
                                start.elapsed()
                            ),
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
        let mut gfa2: GFA2 = GFA2::new();
        match parse_file_to_graph("./tests/big_files/test.gfa2") {
            Ok(g) => {
                println!("Create graph from file: {:?}", start.elapsed());
                let start = Instant::now();
                gfa2 = to_gfa2(&g);
                println!("Convert graph to GFAObject: {:?}", start.elapsed());
                let start = Instant::now();
                match to_bincode(GFAType::GFA2(gfa2)) {
                    Ok(b) => {
                        println!(
                            "Convert GFAObject to BINCODEObject: {:?}",
                            start.elapsed()
                        );
                        let start = Instant::now();
                        match save_on_file(ObjectType::BINCODE(b), None) {
                            Ok(_) => println!(
                                "Save BINCODEObject to file: {:?}",
                                start.elapsed()
                            ),
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
