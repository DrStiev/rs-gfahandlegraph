use crate::{handle::Edge, handlegraph::*, hashgraph::HashGraph, pathgraph::PathHandleGraph};

use crate::gfa::{
    gfa1::{Header as Header1, Link, Path, Segment as Segment1, GFA},
    gfa2::{Edge as GFA2Edge, GroupO, Header, Segment, GFA2},
    orientation::Orientation,
    segment_id::*,
};
use bstr::BString;

/// Function that takes a HashGraph object as input and return a GFA2 object
/// # Example
/// ```ignore
/// let gfa_out: GFA2<BString> = to_gfa2(&graph);
/// /* hashgraph to gfa2:
/// H   VN:Z:2.0
/// S   13  7   CTTGATT
/// S   12  6   TCAAGG
/// S   11  5   ACCTT
/// E   42   12- 13+ 0   0$  0   0$  0M
/// E   42   11+ 12- 0   0$  0   0$  0M
/// E   42   11+ 13+ 0   0$  0   0$  0M
/// O   14  11+ 12- 13+
/// */
///
/// /* original gfa2:
/// H   VN:Z:2.0
/// H
/// S   11   5   ACCTT
/// S   12   6   TCAAGG
/// S   13   7   CTTGATT
/// E   *   11+   12-   1   5$   2   6$   4M
/// E   *   12-   13+   0   5   0   5   5M
/// E   *   11+   13+   2   5$   0   3   3M
/// O   14   11+ 12- 13+
/// */
/// ```
pub fn to_gfa2(graph: &HashGraph) -> GFA2 {
    let mut file: GFA2 = GFA2::default();

    // default header
    let header = Header {
        version: "VN:Z:2.0".into(),
        tag: BString::from(""),
    };
    file.headers.push(header);

    for handle in graph.all_handles() {
        let seq_id = usize::from(handle.id());
        let sequence: BString = graph.sequence_iter(handle.forward()).collect();
        let len: BString = BString::from(sequence.len().to_string());

        let segment = Segment {
            id: seq_id,
            len,
            sequence,
            tag: BString::from(""),
        };
        file.segments.push(segment);
    }

    let orient = |rev: bool| {
        if rev {
            45 as usize
        } else {
            43 as usize
        }
    };

    for edge in graph.all_edges() {
        let Edge(left, right) = edge;

        let sid1_id: String = left.id().to_string();
        let sid1_orient = orient(left.is_reverse());
        let sid1 = format!("{}{}", sid1_id, sid1_orient)
            .parse::<usize>()
            .unwrap();

        let sid2_id: String = right.id().to_string();
        let sid2_orient = orient(right.is_reverse());
        let sid2 = format!("{}{}", sid2_id, sid2_orient)
            .parse::<usize>()
            .unwrap();

        let edge = GFA2Edge {
            // placeholder id
            id: convert_to_usize(b"*").unwrap(),
            sid1,
            sid2,
            beg1: "0".into(),  // placeholder value
            end1: "0$".into(), // placeholder value
            beg2: "0".into(),  // placeholder value
            end2: "0$".into(), // placeholder value
            alignment: "0M".into(),
            tag: BString::from(""),
        };
        file.edges.push(edge);
    }

    let o_orient = |rev: bool| {
        if rev {
            "-"
        } else {
            "+"
        }
    };

    for path_id in graph.paths_iter() {
        let path_name: BString = graph.path_handle_to_name(path_id).into();
        let mut segment_names: Vec<String> = Vec::new();

        for step in graph.steps_iter(path_id) {
            let handle = graph.handle_of_step(&step).unwrap();
            let segment: String = handle.id().to_string();
            let orientation = o_orient(handle.is_reverse());

            segment_names.push(segment);
            segment_names.push(orientation.to_string());
            segment_names.push(" ".to_string());
        }

        let mut segment_names: String = segment_names
            .iter()
            .fold(String::new(), |acc, str| acc + &str.to_string());

        // remove the last whitespace " "
        segment_names.pop();
        let ogroup: GroupO = GroupO::new(path_name, BString::from(segment_names), b"");
        file.groups_o.push(ogroup);
    }
    file
}

/// Function that takes a HashGraph object as input and return a GFA object
/// # Example
/// ```ignore
/// let gfa_out: GFA<BString> = to_gfa(&graph);
///  
/// /* hashgraph to gfa:
/// H   VN:Z:1.0
/// S   13  CTTGATT
/// S   12  TCAAGG
/// S   11  ACCTT
/// L   12  -   13  +   0M
/// L   11  +   12  -   0M
/// L   11  +   13  +   0M
/// P   14  11+ 12- 13+ 0M
/// */
///
/// /* original gfa:
/// H   VN:Z:1.0
/// S   13  CTTGATT
/// S   12  TCAAGG
/// S   11  ACCTT
/// L   12  -   13  +   0M
/// L   11  +   12  -   0M
/// L   11  +   13  +   0M
/// P   14  11+ 12- 13+ 0M
/// */
/// ```
pub fn to_gfa(graph: &HashGraph) -> GFA {
    let mut gfa: GFA = GFA::default();

    let header = Header1 {
        version: "VN:Z:1.0".into(),
        optional: BString::from(""),
    };
    gfa.headers.push(header);

    for handle in graph.all_handles() {
        let name = usize::from(handle.id());
        let sequence: BString = graph.sequence_iter(handle.forward()).collect();

        let segment = Segment1 {
            name,
            sequence,
            optional: BString::from(""),
        };
        gfa.segments.push(segment);
    }

    let orient = |rev: bool| {
        if rev {
            Orientation::Backward
        } else {
            Orientation::Forward
        }
    };

    for edge in graph.all_edges() {
        let Edge(left, right) = edge;
        let from_segment: usize = usize::from(left.id());
        let from_orient = orient(left.is_reverse());
        let to_segment: usize = usize::from(right.id());
        let to_orient = orient(right.is_reverse());
        let overlap = BString::from("0M");

        let link = Link {
            from_segment,
            from_orient,
            to_segment,
            to_orient,
            overlap,
            optional: BString::from(""),
        };
        gfa.links.push(link);
    }

    for path_id in graph.paths_iter() {
        let path_name: BString = graph.path_handle_to_name(path_id).into();
        let mut segment_names: Vec<String> = Vec::new();
        for step in graph.steps_iter(path_id) {
            let handle = graph.handle_of_step(&step).unwrap();
            let segment: String = handle.id().to_string();
            let orientation = orient(handle.is_reverse());

            segment_names.push(segment);
            segment_names.push(orientation.to_string());
            segment_names.push(",".into());
        }
        let mut segment_names: String = segment_names
            .iter()
            .fold(String::new(), |acc, str| acc + &str.to_string());

        // remove the last comma "," otherwise it will produce an error
        // that could break everything (overflow and other bad stuff)
        segment_names.pop();
        let path: Path = Path::new(path_name, BString::from(segment_names), "0M".into(), b"");

        gfa.paths.push(path);
    }
    gfa
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
                g.print_graph();
                let mut _file: GFA = GFA::new();
                _file = to_gfa(&g);
                println!("{}", _file);
            }
            Err(why) => println!("Error {}", why),
        }
    }

    #[test]
    fn can_convert_graph_to_gfa2() {
        match parse_file_to_graph("./tests/gfa2_files/spec_q7.gfa2") {
            Ok(g) => {
                g.print_graph();
                let mut _file: GFA2 = GFA2::new();
                _file = to_gfa2(&g);
                println!("{}", _file);
            }
            Err(why) => println!("Error {}", why),
        }
    }

    #[test]
    fn can_convert_medium_graph_to_gfa2() {
        // Convert graph to GFA2: Duration { seconds: 0, nanoseconds: 209741800 }
        match parse_file_to_graph("./tests/big_files/test.gfa2") {
            Ok(g) => {
                let start = Instant::now();
                let mut _file: GFA2 = GFA2::new();
                _file = to_gfa2(&g);
                println!("Convert graph to GFA2: {:?}", start.elapsed());
            }
            Err(why) => println!("Error {}", why),
        }
    }

    #[test]
    #[ignore]
    fn can_convert_big_graph_to_gfa() {
        // Convert graph to GFA: Duration { seconds: 147, nanoseconds: 274328400 }
        match parse_file_to_graph("./tests/big_files/ape-4-0.10b.gfa2") {
            Ok(g) => {
                let start = Instant::now();
                let mut _file: GFA = GFA::new();
                _file = to_gfa(&g);
                println!("Convert graph to GFA: {:?}", start.elapsed());
            }
            Err(why) => println!("Error {}", why),
        }
    }
}
