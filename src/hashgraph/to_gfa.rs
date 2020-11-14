use crate::{handle::Edge, handlegraph::*, hashgraph::HashGraph, pathgraph::PathHandleGraph};

use crate::gfa::{
    gfa1::{Header as Header1, Link, Path, Segment as Segment1, GFA},
    gfa2::{Edge as GFA2Edge, GroupO, Header, Segment, GFA2},
    orientation::Orientation,
};
use bstr::BString;

/// Function that takes a HashGraph object as input and return a GFA2 object
/// This function is still ```Work In Progress``` so it's not perfect.\
/// Sometimes can leads to unexpected bugs.
/// # Example
/// ```ignore
/// let gfa_out: GFA2<BString> = handlegraph2::conversion::to_gfa2(&graph);
/// /* hashgraph to gfa2:
/// H   VN:Z:2.0
/// S   13  0   CTTGATT
/// S   12  0   TCAAGG
/// S   11  0   ACCTT
/// E   *   12- 13+ 0   0$  0   0$  0M
/// E   *   11+ 12- 0   0$  0   0$  0M
/// E   *   11+ 13+ 0   0$  0   0$  0M
/// O   14  11+ 12- 13+
/// */
///
/// /* original gfa2:
/// H	VN:Z:2.0
/// H
/// S	11	5	ACCTT
/// S	12	6	TCAAGG
/// S	13	7	CTTGATT
/// E	1	11+	12-	1	5$	2	6$	4M
/// E	1	12-	13+	0	5	0	5	5M
/// E	1	11+	13+	2	5$	0	3	3M
/// O	14	11+ 12- 13+
/// */
/// ```
pub fn to_gfa2(graph: &HashGraph) -> GFA2<BString> {
    /*
    // Provide a custom bar style
    let pb_seg = ProgressBar::new(1000);
    pb_seg.set_style(ProgressStyle::default_bar().template(
        "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] [{pos}/{pos}]",
    ));
    // Provide a custom bar style
    let pb_link = ProgressBar::new(1000);
    pb_link.set_style(ProgressStyle::default_bar().template(
        "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] [{pos}/{pos}]",
    ));
    // Provide a custom bar style
    let pb_path = ProgressBar::new(1000);
    pb_path.set_style(ProgressStyle::default_bar().template(
        "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] [{pos}/{pos}]",
    ));
    */

    // I think it can be more efficient but for now it's good
    let mut file: GFA2<BString> = GFA2::new();

    // default header
    let header = Header {
        version: Some("VN:Z:2.0".into()),
        tag: "".into(),
    };
    file.headers.push(header);

    for handle in graph.all_handles()
    /*.progress_with(pb_seg)*/
    {
        let seq_id = BString::from(handle.id().to_string());
        let sequence: BString = graph.sequence_iter(handle.forward()).collect();
        let len: BString = BString::from(sequence.len().to_string());

        let segment = Segment {
            id: seq_id,
            len,
            sequence,
            tag: "".into(),
        };
        file.segments.push(segment);
    }

    let orient = |rev: bool| {
        if rev {
            "-"
        } else {
            "+"
        }
    };

    for edge in graph.all_edges()
    /*.progress_with(pb_link)*/
    {
        let Edge(left, right) = edge;

        let sid1_id: String = left.id().to_string();
        let sid1_orient = orient(left.is_reverse());
        let sid1: BString = BString::from(format!("{}{}", sid1_id, sid1_orient));

        let sid2_id: String = right.id().to_string();
        let sid2_orient = orient(right.is_reverse());
        let sid2: BString = BString::from(format!("{}{}", sid2_id, sid2_orient));

        let edge = GFA2Edge {
            // placeholder id
            id: "*".into(),
            sid1,
            sid2,
            beg1: "0".into(),  // placeholder value
            end1: "0$".into(), // placeholder value
            beg2: "0".into(),  // placeholder value
            end2: "0$".into(), // placeholder value
            alignment: "0M".into(),
            tag: "".into(),
        };
        file.edges.push(edge);
    }

    for path_id in graph.paths_iter()
    /*.progress_with(pb_path)*/
    {
        let path_name: BString = graph.path_handle_to_name(path_id).into();
        let mut segment_names: Vec<String> = Vec::new();

        for step in graph.steps_iter(path_id) {
            let handle = graph.handle_of_step(&step).unwrap();
            let segment: String = handle.id().to_string();
            let orientation = orient(handle.is_reverse());

            segment_names.push(segment);
            segment_names.push(orientation.to_string());
            segment_names.push(" ".to_string());
        }

        let mut segment_names: String = segment_names
            .iter()
            .fold(String::new(), |acc, str| acc + &str.to_string());

        // remove the last whitespace " "
        segment_names.pop();
        let ogroup: GroupO<BString> =
            GroupO::new(path_name, BString::from(segment_names), "".into());
        file.groups_o.push(ogroup);
    }

    file
}

/// Function that takes a HashGraph object as input and return a GFA object
/// This function is still so it's not perfect.\
/// Sometimes can leads to unexpected bugs.
/// # Example
/// ```ignore
/// let gfa_out: GFA<BString> = handlegraph2::conversion::to_gfa(&graph);
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
pub fn to_gfa(graph: &HashGraph) -> GFA<BString> {
    /*
    // Provide a custom bar style
    let pb_seg = ProgressBar::new(1000);
    pb_seg.set_style(ProgressStyle::default_bar().template(
        "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] [{pos}/{pos}]",
    ));
    // Provide a custom bar style
    let pb_link = ProgressBar::new(1000);
    pb_link.set_style(ProgressStyle::default_bar().template(
        "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] [{pos}/{pos}]",
    ));
    // Provide a custom bar style
    let pb_path = ProgressBar::new(1000);
    pb_path.set_style(ProgressStyle::default_bar().template(
        "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] [{pos}/{pos}]",
    ));
    */

    let mut gfa: GFA<BString> = GFA::new();

    // default header
    let header = Header1 {
        version: Some("VN:Z:1.0".into()),
        optional: "".into(),
    };
    gfa.headers.push(header);

    for handle in graph.all_handles()
    /*.progress_with(pb_seg)*/
    {
        let name = BString::from(handle.id().to_string());
        let sequence: BString = graph.sequence_iter(handle.forward()).collect();

        let segment = Segment1 {
            name,
            sequence,
            optional: "".into(),
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

    for edge in graph.all_edges()
    /*.progress_with(pb_link)*/
    {
        let Edge(left, right) = edge;
        let from_segment: BString = BString::from(left.id().to_string());
        let from_orient = orient(left.is_reverse());
        let to_segment: BString = BString::from(right.id().to_string());
        let to_orient = orient(right.is_reverse());
        let overlap = BString::from("0M");

        let link = Link {
            from_segment,
            from_orient,
            to_segment,
            to_orient,
            overlap,
            optional: "".into(),
        };

        gfa.links.push(link);
    }

    for path_id in graph.paths_iter()
    /*.progress_with(pb_path)*/
    {
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
        let path: Path<BString> = Path::new(
            path_name,
            BString::from(segment_names),
            "0M".into(),
            "".into(),
        );

        gfa.paths.push(path);
    }

    gfa
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::parser::Parser;

    #[test]
    fn can_convert_graph_to_gfa() {
        let parser: Parser<usize> = Parser::new();
        match parser.parse_file_to_graph("./tests/gfa1_files/lil.gfa") {
            Ok(g) => {
                g.print_graph();
                let mut _file: GFA<BString> = GFA::new();
                _file = to_gfa(&g);
                println!("{}", _file);
            }
            Err(why) => println!("Error {}", why),
        }
    }

    #[test]
    fn can_convert_graph_to_gfa2() {
        let parser: Parser<usize> = Parser::new();
        match parser.parse_file_to_graph("./tests/gfa2_files/spec_q7.gfa2") {
            Ok(g) => {
                g.print_graph();
                let mut _file: GFA2<BString> = GFA2::new();
                _file = to_gfa2(&g);
                println!("{}", _file);
            }
            Err(why) => println!("Error {}", why),
        }
    }
}
