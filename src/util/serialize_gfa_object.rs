use crate::gfa::{gfa1::GFA, gfa2::GFA2};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// creates JSON or BINCODE OBject by serializing data structures
pub enum GFAType {
    GFA(GFA),
    GFA2(GFA2),
}

/// Function that convert a
/// [`GFA`](file:///D:/GitHub/rs-gfahandlegraph/target/doc/gfahandlegraph/gfa/gfa1/struct.GFA.html),
/// [`GFA2`](file:///D:/GitHub/rs-gfahandlegraph/target/doc/gfahandlegraph/gfa/gfa2/struct.GFA2.html),
/// object into a [`JSON`](https://docs.serde.rs/serde_json/) file
pub fn to_json(gfa: GFAType) -> serde_json::Result<String> {
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
pub fn to_bincode(gfa: GFAType) -> bincode::Result<Vec<u8>> {
    match gfa {
        GFAType::GFA(g) => {
            let bincode_file: Vec<u8> = bincode::serialize(&g)?;
            Ok(bincode_file)
        }
        GFAType::GFA2(g) => {
            let bincode_file: Vec<u8> = bincode::serialize(&g)?;
            Ok(bincode_file)
        }
    }
}

/// Function that convert a
/// [`GFA`](file:///D:/GitHub/rs-gfahandlegraph/target/doc/gfahandlegraph/gfa/gfa1/struct.GFA.html),
/// [`GFA2`](file:///D:/GitHub/rs-gfahandlegraph/target/doc/gfahandlegraph/gfa/gfa2/struct.GFA2.html),
/// object into a  [`YAML`](https://docs.serde.rs/serde_yaml/) file
pub fn to_yaml(gfa: GFAType) -> serde_yaml::Result<String> {
    match gfa {
        GFAType::GFA(g) => {
            let yaml_file: String = serde_yaml::to_string(&g)?;
            Ok(yaml_file)
        }
        GFAType::GFA2(g) => {
            let yaml_file: String = serde_yaml::to_string(&g)?;
            Ok(yaml_file)
        }
    }
}

pub fn to_hash(gfa: GFAType) -> Result<DefaultHasher, String> {
    let mut hasher = DefaultHasher::new();
    match gfa {
        GFAType::GFA(g) => {
            g.hash(&mut hasher);
            Ok(hasher)
        }
        GFAType::GFA2(g) => {
            g.hash(&mut hasher);
            Ok(hasher)
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

    #[test]
    fn simple_serializator() {
        match parse_file_to_graph("./tests/gfa2_files/spec_q7.gfa2") {
            Ok(g) => {
                g.print_graph();
                let file = to_gfa2(&g);
                match to_json(GFAType::GFA2(file.clone())) {
                    Ok(j) => println!("{}", j),
                    Err(why) => println!("Error: {}", why),
                }
                match to_yaml(GFAType::GFA2(file.clone())) {
                    Ok(y) => println!("{}", y),
                    Err(why) => println!("Error: {}", why),
                }
                match to_hash(GFAType::GFA2(file)) {
                    Ok(h) => println!("{:X}", h.finish()),
                    Err(why) => println!("Error: {}", why),
                }
            }
            Err(why) => println!("Error: {}", why),
        }
    }

    #[test]
    #[ignore]
    fn big_files() {
        convert_big_gfa_to_bincode();
        convert_big_gfa_to_json();
        convert_big_gfa_to_yaml();
    }

    #[test]
    #[ignore]
    fn medium_files() {
        convert_medium_gfa_to_bincode();
        convert_medium_gfa_to_json();
        convert_medium_gfa_to_yaml();
    }

    fn convert_big_gfa_to_json() {
        /* NOT GOOD FOR BIG FILES, the dimension skyrockets and the conversion it's too slow
        Create graph from file: Duration { seconds: 510, nanoseconds: 82235500 }
        Convert graph to GFAObject: Duration { seconds: 149, nanoseconds: 505934600 }
        Convert GFAObject to JSONObject: Duration { seconds: 894, nanoseconds: 650416500 }
        Save JSONObject to file: Duration { seconds: 68, nanoseconds: 859425000 }
        */
        let start = Instant::now();
        match parse_file_to_graph("./tests/big_files/ape-4-0.10b.gfa2") {
            Ok(g) => {
                println!("Create graph from file: {:?}", start.elapsed());
                let start = Instant::now();
                let gfa2 = to_gfa2(&g);
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
        match parse_file_to_graph("./tests/big_files/ape-4-0.10b.gfa2") {
            Ok(g) => {
                println!("Create graph from file: {:?}", start.elapsed());
                let start = Instant::now();
                let gfa2 = to_gfa2(&g);
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

    fn convert_big_gfa_to_yaml() {
        /*
        memory allocation of 1761336 bytes failed
        error: test failed, to rerun pass '-p gfahandlegraph --lib'

        Caused by:
          process didn't exit successfully:
          `D:\GitHub\rs-gfahandlegraph\target\debug\deps\gfahandlegraph-7d060bad315ba7d5.exe
          util::serialize_gfa_object::tests::big_files --exact -Z unstable-options --format=json --show-output`
          (exit code: 0xc0000409, STATUS_STACK_BUFFER_OVERRUN)

        Process finished with exit code -1073740791 (0xC0000409)
         */
        let start = Instant::now();
        match parse_file_to_graph("./tests/big_files/ape-4-0.10b.gfa2") {
            Ok(g) => {
                println!("Create graph from file: {:?}", start.elapsed());
                let start = Instant::now();
                let gfa2 = to_gfa2(&g);
                println!("Convert graph to GFAObject: {:?}", start.elapsed());
                let start = Instant::now();
                match to_yaml(GFAType::GFA2(gfa2)) {
                    Ok(b) => {
                        println!(
                            "Convert GFAObject to YAMLObject: {:?}",
                            start.elapsed()
                        );
                        let start = Instant::now();
                        match save_on_file(
                            ObjectType::YAML(b),
                            Some(
                                "./tests/output_files/ape-4-0.10b.gfa2.yaml"
                                    .to_string(),
                            ),
                        ) {
                            Ok(_) => println!(
                                "Save YAMLObject to file: {:?}",
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

    fn convert_medium_gfa_to_json() {
        /*
        Create graph from file: Duration { seconds: 1, nanoseconds: 149224700 }
        Convert graph to GFAObject: Duration { seconds: 0, nanoseconds: 162454600 }
        Convert GFAObject to JSONObject: Duration { seconds: 0, nanoseconds: 128758100 }
        Save JSONObject to file: Duration { seconds: 0, nanoseconds: 25843000 }
        */
        let start = Instant::now();
        match parse_file_to_graph("./tests/big_files/test.gfa2") {
            Ok(g) => {
                println!("Create graph from file: {:?}", start.elapsed());
                let start = Instant::now();
                let gfa2 = to_gfa2(&g);
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

    fn convert_medium_gfa_to_bincode() {
        /*
        Create graph from file: Duration { seconds: 1, nanoseconds: 140760200 }
        Convert graph to GFAObject: Duration { seconds: 0, nanoseconds: 161618800 }
        Convert GFAObject to BINCODEObject: Duration { seconds: 0, nanoseconds: 21763800 }
        Save BINCODEObject to file: Duration { seconds: 0, nanoseconds: 50598900 }
        */
        let start = Instant::now();
        match parse_file_to_graph("./tests/big_files/test.gfa2") {
            Ok(g) => {
                println!("Create graph from file: {:?}", start.elapsed());
                let start = Instant::now();
                let gfa2 = to_gfa2(&g);
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

    fn convert_medium_gfa_to_yaml() {
        /*
        Create graph from file: Duration { seconds: 1, nanoseconds: 133443700 }
        Convert graph to GFAObject: Duration { seconds: 0, nanoseconds: 161258600 }
        Convert GFAObject to YAMLObject: Duration { seconds: 0, nanoseconds: 134835900 }
        Save YAMLObject to file: Duration { seconds: 0, nanoseconds: 34949700 }
        */
        let start = Instant::now();
        match parse_file_to_graph("./tests/big_files/test.gfa2") {
            Ok(g) => {
                println!("Create graph from file: {:?}", start.elapsed());
                let start = Instant::now();
                let gfa2 = to_gfa2(&g);
                println!("Convert graph to GFAObject: {:?}", start.elapsed());
                let start = Instant::now();
                match to_yaml(GFAType::GFA2(gfa2)) {
                    Ok(b) => {
                        println!(
                            "Convert GFAObject to YAMLObject: {:?}",
                            start.elapsed()
                        );
                        let start = Instant::now();
                        match save_on_file(ObjectType::YAML(b), None) {
                            Ok(_) => println!(
                                "Save YAMLObject to file: {:?}",
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
