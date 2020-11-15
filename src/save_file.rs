use crate::gfa::{gfa1::GFA, gfa2::GFA2};
use crate::hashgraph::to_gfa::*;
use crate::hashgraph::HashGraph;

use bstr::BString;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// Function that save a GFA2 object in a file
/// on a specific or default location
/// # Example
/// ```ignore
/// save_as_gfa2_file(&graph, Some(String::from("./tests/output_files/gfa2_to_file.gfa")));
/// ```
pub fn save_as_gfa2_file(graph: &HashGraph, path: Option<String>) -> Result<(), std::io::Error> {
    let path =
        path.unwrap_or_else(|| String::from("./tests/output_files/default_path/file_gfa2.gfa2"));
    let path = Path::new(&path);
    let mut file = File::create(path)?;
    let gfa_file: GFA2<BString> = to_gfa2(&graph);
    file.write_all(format!("{}", gfa_file).as_bytes())?;
    file.sync_all()?;
    Ok(())
}

/// Function that save a GFA2 object in a file
/// on a specific or default location
/// # Example
/// ```ignore
/// save_gfa2_file(&gfa2_file, Some(String::from("./tests/output_files/gfa2_to_file.gfa")));
/// ```
pub fn save_gfa2_file(gfa2: GFA2<usize>, path: Option<String>) -> Result<(), std::io::Error> {
    let path =
        path.unwrap_or_else(|| String::from("./tests/output_files/default_path/file_gfa2.gfa2"));
    let path = Path::new(&path);
    let mut file = File::create(path)?;
    file.write_all(format!("{}", gfa2).as_bytes())?;
    file.sync_all()?;
    Ok(())
}

/// Function that save a GFA1 object in a file
/// on a specific or default location
/// # Example
/// ```ignore
/// save_as_gfa1_file(&graph, Some(String::from("./tests/output_files/gfa2_to_file.gfa")));
/// ```
pub fn save_as_gfa1_file(graph: &HashGraph, path: Option<String>) -> Result<(), std::io::Error> {
    let path =
        path.unwrap_or_else(|| String::from("./tests/output_files/default_path/file_gfa1.gfa"));
    let path = Path::new(&path);
    let mut file = File::create(path)?;
    let gfa_file: GFA<BString> = to_gfa(&graph);
    file.write_all(format!("{}", gfa_file).as_bytes())?;
    file.sync_all()?;
    Ok(())
}

/// Function that save a GFA2 object in a file
/// on a specific or default location
/// # Example
/// ```ignore
/// save_gfa1_file(&gfa1_file, Some(String::from("./tests/output_files/gfa2_to_file.gfa")));
/// ```
pub fn save_gfa1_file(gfa1: GFA<usize>, path: Option<String>) -> Result<(), std::io::Error> {
    let path =
        path.unwrap_or_else(|| String::from("./tests/output_files/default_path/file_gfa2.gfa2"));
    let path = Path::new(&path);
    let mut file = File::create(path)?;
    file.write_all(format!("{}", gfa1).as_bytes())?;
    file.sync_all()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handle::*;
    use crate::mutablehandlegraph::{AdditiveHandleGraph, MutableHandleGraph};
    use crate::pathgraph::PathHandleGraph;

    #[test]
    fn can_save_handlegraph_as_gfa2_file() {
        let mut graph = HashGraph::new();
        let h1 = graph.create_handle(b"ACCTT", 11).unwrap();
        let h2 = graph.create_handle(b"TCAAGG", 12).unwrap();
        let h3 = graph.create_handle(b"CTTGATT", 13).unwrap();

        // use .flip() to apply reverse complement to the node
        graph.apply_orientation(h2.flip());

        match graph.create_edge(Edge(h1, h2)) {
            Ok(_) => (),
            Err(why) => println!("Error: {}", why),
        };
        match graph.create_edge(Edge(h2, h3)) {
            Ok(_) => (),
            Err(why) => println!("Error: {}", why),
        };
        match graph.create_edge(Edge(h1, h3)) {
            Ok(_) => (),
            Err(why) => println!("Error: {}", why),
        };

        let path = graph.create_path_handle(b"1", false);
        // path: 1 -> 2 -> 3
        graph.append_step(&path, h1);
        graph.append_step(&path, h2);
        graph.append_step(&path, h3);

        // save file on a specific path
        match save_as_gfa2_file(
            &graph,
            Some(String::from("./tests/output_files/file_gfa2.gfa2")),
        ) {
            Ok(_) => println!("Handlegraph saved correctly!"),
            Err(why) => println!("Error: {}", why),
        };
    }

    #[test]
    fn can_save_handlegraph_as_gfa2_file_default_path() {
        let mut graph = HashGraph::new();
        let h1 = graph.create_handle(b"ACCTT", 11).unwrap();
        let h2 = graph.create_handle(b"TCAAGG", 12).unwrap();
        let h3 = graph.create_handle(b"CTTGATT", 13).unwrap();

        // use .flip() to apply reverse complement to the node
        graph.apply_orientation(h2.flip());

        match graph.create_edge(Edge(h1, h2)) {
            Ok(_) => (),
            Err(why) => println!("Error: {}", why),
        };
        match graph.create_edge(Edge(h2, h3)) {
            Ok(_) => (),
            Err(why) => println!("Error: {}", why),
        };
        match graph.create_edge(Edge(h1, h3)) {
            Ok(_) => (),
            Err(why) => println!("Error: {}", why),
        };

        let path = graph.create_path_handle(b"1", false);
        // path: 1 -> 2 -> 3
        graph.append_step(&path, h1);
        graph.append_step(&path, h2);
        graph.append_step(&path, h3);

        // save file on a default path
        match save_as_gfa2_file(&graph, None) {
            Ok(_) => println!("Handlegraph saved correctly!"),
            Err(why) => println!("Error: {}", why),
        };
    }

    #[test]
    fn can_save_handlegraph_as_gfa1_file() {
        let mut graph = HashGraph::new();
        let h1 = graph.create_handle(b"ACCTT", 11).unwrap();
        let h2 = graph.create_handle(b"TCAAGG", 12).unwrap();
        let h3 = graph.create_handle(b"CTTGATT", 13).unwrap();

        // use .flip() to apply reverse complement to the node
        graph.apply_orientation(h2.flip());

        match graph.create_edge(Edge(h1, h2)) {
            Ok(_) => (),
            Err(why) => println!("Error: {}", why),
        };
        match graph.create_edge(Edge(h2, h3)) {
            Ok(_) => (),
            Err(why) => println!("Error: {}", why),
        };
        match graph.create_edge(Edge(h1, h3)) {
            Ok(_) => (),
            Err(why) => println!("Error: {}", why),
        };

        let path = graph.create_path_handle(b"1", false);
        // path: 1 -> 2 -> 3
        graph.append_step(&path, h1);
        graph.append_step(&path, h2);
        graph.append_step(&path, h3);

        // save file on a specific path
        match save_as_gfa1_file(
            &graph,
            Some(String::from("./tests/output_files/file_gfa1.gfa")),
        ) {
            Ok(_) => println!("Handlegraph saved correctly!"),
            Err(why) => println!("Error: {}", why),
        };
    }

    #[test]
    fn can_save_handlegraph_as_gfa1_file_default_path() {
        let mut graph = HashGraph::new();
        let h1 = graph.create_handle(b"ACCTT", 11).unwrap();
        let h2 = graph.create_handle(b"TCAAGG", 12).unwrap();
        let h3 = graph.create_handle(b"CTTGATT", 13).unwrap();

        // use .flip() to apply reverse complement to the node
        graph.apply_orientation(h2.flip());

        match graph.create_edge(Edge(h1, h2)) {
            Ok(_) => (),
            Err(why) => println!("Error: {}", why),
        };
        match graph.create_edge(Edge(h2, h3)) {
            Ok(_) => (),
            Err(why) => println!("Error: {}", why),
        };
        match graph.create_edge(Edge(h1, h3)) {
            Ok(_) => (),
            Err(why) => println!("Error: {}", why),
        };

        let path = graph.create_path_handle(b"1", false);
        // path: 1 -> 2 -> 3
        graph.append_step(&path, h1);
        graph.append_step(&path, h2);
        graph.append_step(&path, h3);

        // save file on a default path
        match save_as_gfa1_file(&graph, None) {
            Ok(_) => println!("Handlegraph saved correctly!"),
            Err(why) => println!("Error: {}", why),
        };
    }
}
