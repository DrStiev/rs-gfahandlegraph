use crate::gfa::{gfa1::GFA, gfa2::GFA2};
use crate::hashgraph::HashGraph;
use crate::util::to_gfa::*;

use bstr::BString;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub enum ObjectType {
    GFABSTRING(GFA<BString>),
    GFAUSIZE(GFA<usize>),
    GFA2BSTRING(GFA2<BString>),
    GFA2USIZE(GFA2<usize>),
}

pub enum GraphType {
    FROMGFA1GRAPH(HashGraph),
    FROMGFA2GRAPH(HashGraph),
}

#[inline]
pub fn save_gfa_on_file(file: ObjectType, path: Option<String>) -> std::io::Result<()> {
    match file {
        ObjectType::GFAUSIZE(x) => {
            let path = path.unwrap_or_else(|| {
                String::from("./tests/output_files/default_path/file_usize.gfa")
            });
            let path = Path::new(&path);
            let mut file = File::create(path)?;
            file.write_all(format!("{}", x).as_bytes())?;
            file.sync_all()?;
            Ok(())
        }
        ObjectType::GFA2USIZE(x) => {
            let path = path.unwrap_or_else(|| {
                String::from("./tests/output_files/default_path/file_usize.gfa2")
            });
            let path = Path::new(&path);
            let mut file = File::create(path)?;
            file.write_all(format!("{}", x).as_bytes())?;
            file.sync_all()?;
            Ok(())
        }
        ObjectType::GFABSTRING(x) => {
            let path = path.unwrap_or_else(|| {
                String::from("./tests/output_files/default_path/file_bstring.gfa")
            });
            let path = Path::new(&path);
            let mut file = File::create(path)?;
            file.write_all(format!("{}", x).as_bytes())?;
            file.sync_all()?;
            Ok(())
        }
        ObjectType::GFA2BSTRING(x) => {
            let path = path.unwrap_or_else(|| {
                String::from("./tests/output_files/default_path/file_bstring.gfa2")
            });
            let path = Path::new(&path);
            let mut file = File::create(path)?;
            file.write_all(format!("{}", x).as_bytes())?;
            file.sync_all()?;
            Ok(())
        }
    }
}

/// Function that save a GFA1(2) object in a file
/// on a specific or default location
/// # Example
/// ```ignore
/// save_graph_to_file(GraphType::FROMGFA2GRAPH(graph), Some(String::from("./tests/output_files/gfa2_to_file.gfa")));
/// ```
#[inline]
pub fn save_graph_to_file(graph: GraphType, path: Option<String>) -> std::io::Result<()> {
    match graph {
        GraphType::FROMGFA1GRAPH(g) => {
            let path = path.unwrap_or_else(|| {
                String::from("./tests/output_files/default_path/file_graph.gfa")
            });
            let path = Path::new(&path);
            let mut file = File::create(path)?;
            let gfa_file: GFA<BString> = to_gfa(&g);
            file.write_all(format!("{}", gfa_file).as_bytes())?;
            file.sync_all()?;
            Ok(())
        }
        GraphType::FROMGFA2GRAPH(g) => {
            let path = path.unwrap_or_else(|| {
                String::from("./tests/output_files/default_path/file_graph.gfa2")
            });
            let path = Path::new(&path);
            let mut file = File::create(path)?;
            let gfa_file: GFA2<BString> = to_gfa2(&g);
            file.write_all(format!("{}", gfa_file).as_bytes())?;
            file.sync_all()?;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handle::*;
    use crate::mutablehandlegraph::{AdditiveHandleGraph, MutableHandleGraph};
    use crate::pathgraph::PathHandleGraph;
    use crate::util::GraphType;

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
        match graph.append_step(&path, h1) {
            Ok(_) => (),
            Err(why) => println!("Error: {}", why),
        };
        match graph.append_step(&path, h2) {
            Ok(_) => (),
            Err(why) => println!("Error: {}", why),
        };
        match graph.append_step(&path, h3) {
            Ok(_) => (),
            Err(why) => println!("Error: {}", why),
        };

        // save file on a specific path
        match save_graph_to_file(
            GraphType::FROMGFA2GRAPH(graph),
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
        match graph.append_step(&path, h1) {
            Ok(_) => (),
            Err(why) => println!("Error: {}", why),
        };
        match graph.append_step(&path, h2) {
            Ok(_) => (),
            Err(why) => println!("Error: {}", why),
        };
        match graph.append_step(&path, h3) {
            Ok(_) => (),
            Err(why) => println!("Error: {}", why),
        };

        // save file on a default path
        match save_graph_to_file(GraphType::FROMGFA2GRAPH(graph), None) {
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
        match graph.append_step(&path, h1) {
            Ok(_) => (),
            Err(why) => println!("Error: {}", why),
        };
        match graph.append_step(&path, h2) {
            Ok(_) => (),
            Err(why) => println!("Error: {}", why),
        };
        match graph.append_step(&path, h3) {
            Ok(_) => (),
            Err(why) => println!("Error: {}", why),
        };

        // save file on a specific path
        match save_graph_to_file(
            GraphType::FROMGFA1GRAPH(graph),
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
        match graph.append_step(&path, h1) {
            Ok(_) => (),
            Err(why) => println!("Error: {}", why),
        };
        match graph.append_step(&path, h2) {
            Ok(_) => (),
            Err(why) => println!("Error: {}", why),
        };
        match graph.append_step(&path, h3) {
            Ok(_) => (),
            Err(why) => println!("Error: {}", why),
        };

        // save file on a default path
        match save_graph_to_file(GraphType::FROMGFA1GRAPH(graph), None) {
            Ok(_) => println!("Handlegraph saved correctly!"),
            Err(why) => println!("Error: {}", why),
        };
    }
}
