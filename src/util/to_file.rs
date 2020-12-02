use crate::{
    handle::Edge, handlegraph::*, hashgraph::HashGraph,
    pathgraph::PathHandleGraph,
};

use bstr::BString;
use rayon::iter::ParallelIterator;
use std::fs::File;
use std::io::Write;
use std::sync::Mutex;

/// take an HashGraph and create a GFA1 or GFA2 file from it and save that file on a specific
/// location or on a default one
pub fn to_gfa(
    graph: &HashGraph,
    format: String,
    path: Option<String>,
) -> std::io::Result<()> {
    match format.to_uppercase().as_str() {
        "GFA2" => {
            let path = path.unwrap_or_else(|| {
                String::from(
                    "./tests/output_files/default_path/converted_hashgraph.gfa",
                )
            });
            let file = Mutex::new(File::create(path)?);
            file.lock()
                .unwrap()
                .write(b"H\tVN:Z:2.0\n")
                .expect("Unable to write File");

            graph.handles_par().for_each(|h| {
                let id = usize::from(h.id());
                let sequence: BString =
                    graph.sequence_iter(h.forward()).collect();
                let len: BString = BString::from(sequence.len().to_string());

                file.lock()
                    .unwrap()
                    .write(
                        format!("S\t{}\t{}\t{}\n", id, len, sequence)
                            .as_bytes(),
                    )
                    .expect("Unable to write File");
            });

            let orient = |rev: bool| {
                if rev {
                    "-"
                } else {
                    "+"
                }
            };

            graph.edges_par().for_each(|e| {
                let Edge(left, right) = e;

                let sid1_id: String = left.id().to_string();
                let sid1_orient = orient(left.is_reverse());
                let sid1 = format!("{}{}", sid1_id, sid1_orient);

                let sid2_id: String = right.id().to_string();
                let sid2_orient = orient(right.is_reverse());
                let sid2 = format!("{}{}", sid2_id, sid2_orient);

                file.lock()
                    .unwrap()
                    .write(
                        format!(
                            "E\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
                            "*", sid1, sid2, "0", "0$", "0", "0$", "0M"
                        )
                        .as_bytes(),
                    )
                    .expect("Unable to write File");
            });

            graph.paths().for_each(|p| {
                let id: BString = graph.path_handle_to_name(p).into();
                let mut segment_names: Vec<String> = Vec::new();

                graph.steps(p).for_each(|s| {
                    let handle = graph.handle_of_step(&s).unwrap();
                    let segment: String = handle.id().to_string();
                    let orientation = orient(handle.is_reverse());

                    segment_names.push(segment);
                    segment_names.push(orientation.to_string());
                    segment_names.push(" ".to_string());
                });

                let mut segment_names: String = segment_names
                    .iter()
                    .fold(String::new(), |acc, str| acc + &str.to_string());

                // remove the last whitespace " "
                segment_names.pop();
                file.lock()
                    .unwrap()
                    .write(format!("O\t{}\t{}\n", id, segment_names).as_bytes())
                    .expect("Unable to write File");
            });
            file.lock().unwrap().sync_all()?;
            Ok(())
        }
        "GFA" => {
            let path = path.unwrap_or_else(|| String::from("./tests/output_files/default_path/converted_hashgraph.gfa2"));
            let file = Mutex::new(File::create(path)?);
            file.lock()
                .unwrap()
                .write(b"H\tVN:Z:1.0\n")
                .expect("Unable to write File");

            graph.handles_par().for_each(|h| {
                let id = usize::from(h.id());
                let sequence: BString =
                    graph.sequence_iter(h.forward()).collect();

                file.lock()
                    .unwrap()
                    .write(format!("S\t{}\t{}\n", id, sequence).as_bytes())
                    .expect("Unable to write File");
            });

            let orient = |rev: bool| {
                if rev {
                    "-"
                } else {
                    "+"
                }
            };

            graph.edges_par().for_each(|e| {
                let Edge(left, right) = e;

                let sid1_id: String = left.id().to_string();
                let sid1_orient = orient(left.is_reverse());

                let sid2_id: String = right.id().to_string();
                let sid2_orient = orient(right.is_reverse());

                file.lock()
                    .unwrap()
                    .write(
                        format!(
                            "L\t{}\t{}\t{}\t{}\t{}\n",
                            sid1_id, sid1_orient, sid2_id, sid2_orient, "0M"
                        )
                        .as_bytes(),
                    )
                    .expect("Unable to write File");
            });

            graph.paths().for_each(|p| {
                let id: BString = graph.path_handle_to_name(p).into();
                let mut segment_names: Vec<String> = Vec::new();

                graph.steps(p).for_each(|s| {
                    let handle = graph.handle_of_step(&s).unwrap();
                    let segment: String = handle.id().to_string();
                    let orientation = orient(handle.is_reverse());

                    segment_names.push(segment);
                    segment_names.push(orientation.to_string());
                    segment_names.push(" ".to_string());
                });

                let mut segment_names: String = segment_names
                    .iter()
                    .fold(String::new(), |acc, str| acc + &str.to_string());
                // remove the last whitespace " "
                segment_names.pop();

                file.lock()
                    .unwrap()
                    .write(
                        format!("P\t{}\t{}\t{}\n", id, segment_names, "0M")
                            .as_bytes(),
                    )
                    .expect("Unable to write File");
            });
            file.lock().unwrap().sync_all()?;
            Ok(())
        }
        _ => panic!("Error the format it's not correct!"),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::parse_file_to_graph;
    use time::Instant;

    #[test]
    fn can_convert_graph_to_gfa() {
        match parse_file_to_graph("./tests/gfa1_files/lil.gfa") {
            Ok(g) => {
                println!("{}", g); //g.print_graph(),
                match to_gfa(&g, "gfa".to_string(), None) {
                    Err(why) => println!("Error: {}", why),
                    _ => (),
                }
            }
            Err(why) => println!("Error {}", why),
        }
    }

    #[test]
    fn can_convert_graph_to_gfa2() {
        match parse_file_to_graph("./tests/gfa2_files/spec_q7.gfa2") {
            Ok(g) => {
                println!("{}", g); //g.print_graph(),
                match to_gfa(&g, "gfa2".to_string(), None) {
                    Err(why) => println!("Error: {}", why),
                    _ => (),
                }
            }
            Err(why) => println!("Error {}", why),
        }
    }

    #[test]
    fn can_convert_medium_graph_to_gfa2() {
        // Convert graph to GFA2: Duration { seconds: 0, nanoseconds: 299785900 }
        match parse_file_to_graph("./tests/big_files/test.gfa2") {
            Ok(g) => {
                let start = Instant::now();
                match to_gfa(&g, "gfa2".to_string(), None) {
                    Err(why) => println!("Error: {}", why),
                    _ => {
                        println!("Convert graph to GFA2: {:?}", start.elapsed())
                    }
                }
            }
            Err(why) => println!("Error {}", why),
        }
    }

    #[test]
    #[ignore]
    fn can_convert_big_graph_to_gfa() {
        // Convert graph to GFA2: Duration { seconds: 28, nanoseconds: 111079100 }
        match parse_file_to_graph("./tests/big_files/ape-4-0.10b.gfa") {
            Ok(g) => {
                let start = Instant::now();
                match to_gfa(&g, "gfa".to_string(), None) {
                    Err(why) => println!("Error: {}", why),
                    _ => {
                        println!("Convert graph to GFA2: {:?}", start.elapsed())
                    }
                }
            }
            Err(why) => println!("Error {}", why),
        }
    }
}
