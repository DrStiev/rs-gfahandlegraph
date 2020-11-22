use bstr::{BString, ByteSlice};
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::gfa::gfa2::{Edge, GroupO, Header, Segment, GFA2};
use crate::gfa::segment_id::*;

/// Very BASIC converter from GFA1 format to GFA2 format.\
/// For now it consider only S-, L- and P- lines.
/// ignoring all the others.
/// WIP
pub fn gfa_file_to_gfa2(path: String) -> GFA2 {
    let mut gfa2 = GFA2::default();
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        let mut line_split = line.split_whitespace();
        let prefix = line_split.next().unwrap();

        match prefix {
            "H" => {
                let mut tag = line_split.next();
                let mut opt_fields: Vec<&[u8]> = vec![];
                let mut version: BString = BString::from("");
                while tag.is_some() {
                    if tag.unwrap() == "VN:Z:1.0" {
                        version = BString::from("VN:Z:2.0");
                    } else {
                        opt_fields.push(tag.unwrap().as_bytes());
                    }
                    tag = line_split.next();
                }
                let tag = opt_fields
                    .into_iter()
                    .map(BString::from)
                    .collect::<BString>();

                let header = Header { version, tag };
                gfa2.headers.push(header);
            }
            "S" => {
                let id = convert_to_usize(line_split.next().unwrap().as_bytes()).unwrap();
                let sequence = BString::from(line_split.next().unwrap());
                let len = BString::from(sequence.len().to_string());

                let mut tag = line_split.next();
                let mut opt_fields: Vec<&[u8]> = vec![];
                while tag.is_some() {
                    opt_fields.push(tag.unwrap().as_bytes());
                    tag = line_split.next();
                }
                let tag = opt_fields
                    .into_iter()
                    .map(BString::from)
                    .collect::<BString>();

                let segment = Segment {
                    id,
                    len,
                    sequence,
                    tag,
                };
                gfa2.segments.push(segment);
            }
            "L" => {
                // placeholder value
                let id = convert_to_usize(b"*").unwrap();

                let from_node = convert_to_usize(line_split.next().unwrap().as_bytes()).unwrap();
                let from_node_orient = convert_to_usize(line_split.next().unwrap().as_bytes()).unwrap();
                let to_node = convert_to_usize(line_split.next().unwrap().as_bytes()).unwrap();
                let to_node_orient = convert_to_usize(line_split.next().unwrap().as_bytes()).unwrap();
                let alignment = BString::from(line_split.next().unwrap());

                // placeholder values
                let mut beg1 = BString::from("0");
                let mut end1 = BString::from("0$");
                let mut beg2 = BString::from("0");
                let mut end2 = BString::from("0$");

                if alignment != "*" {
                    let len = alignment.len() - 1;
                    let dist = alignment[..len].to_str().unwrap().parse::<i64>().unwrap();

                    if from_node_orient == 43 && to_node_orient == 43 {
                        let x = (100 - dist).abs();
                        beg1 = BString::from(x.to_string());
                        end1 = BString::from("100$");
                        end2 = BString::from(dist.to_string());
                    } else if from_node_orient == 45 && to_node_orient == 45 {
                        let x = (100 - dist).abs();
                        end1 = BString::from(dist.to_string());
                        beg2 = BString::from(x.to_string());
                        end2 = BString::from("100$");
                    } else if from_node_orient == 45 && to_node_orient == 43 {
                        end1 = BString::from(dist.to_string());
                        end2 = BString::from(dist.to_string());
                    } else if from_node_orient == 43 && to_node_orient == 45 {
                        let x = (100 - dist).abs();
                        beg1 = BString::from(x.to_string());
                        end1 = BString::from("100$");
                        beg2 = BString::from(x.to_string());
                        end2 = BString::from("100$");
                    }
                }

                let mut tag = line_split.next();
                let mut opt_fields: Vec<&[u8]> = vec![];
                while tag.is_some() {
                    opt_fields.push(tag.unwrap().as_bytes());
                    tag = line_split.next();
                }
                let tag = opt_fields
                    .into_iter()
                    .map(BString::from)
                    .collect::<BString>();

                let edge = Edge {
                    id,
                    sid1: format!("{}{}", from_node, from_node_orient).parse::<usize>().unwrap(),
                    sid2: format!("{}{}", to_node, to_node_orient).parse::<usize>().unwrap(),
                    beg1,
                    end1,
                    beg2,
                    end2,
                    alignment,
                    tag,
                };
                gfa2.edges.push(edge);
            }
            "P" => {
                let id = BString::from(line_split.next().unwrap());
                let seg_ids = line_split.next().unwrap();
                let res = BString::from(str::replace(seg_ids, ",", " "));

                let mut tag = line_split.next();
                let mut opt_fields: Vec<&[u8]> = vec![];
                while tag.is_some() {
                    opt_fields.push(tag.unwrap().as_bytes());
                    tag = line_split.next();
                }
                let tag = opt_fields
                    .into_iter()
                    .map(BString::from)
                    .collect::<BString>();

                let ogroup = GroupO::new(id, res, &tag);
                gfa2.groups_o.push(ogroup);
            }
            // ignore all the other lines (typically C- and comment-lines)
            _ => (),
        }
    }
    gfa2
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util::save_file::save_on_file;
    use crate::util::ObjectType;
    use time::Instant;

    #[test]
    #[ignore]
    fn can_parse_and_write_big_file() {
        /*
        Convert file from GFA to GFA2: Duration { seconds: 288, nanoseconds: 430953600 }
        Save big gfa2 file: Duration { seconds: 69, nanoseconds: 178859800 }
        */
        let start = Instant::now();
        let path: String = "./tests/big_files/ape-4-0.10b.gfa".to_string();
        let gfa2: GFA2 = gfa_file_to_gfa2(path.clone());
        println!("Convert file from GFA to GFA2 {:?}", start.elapsed());
        let start = Instant::now();
        match save_on_file(
            ObjectType::GFA2(gfa2),
            Some(format!("{}{}", path, "2")),
        ) {
            Ok(_) => println!("Save big gfa2 file {:?}", start.elapsed()),
            Err(why) => println!("Error: {}", why),
        }
    }

    #[test]
    //#[ignore]
    fn can_parse_and_wirte_file_with_tags() {
        // Convert file from GFA to GFA2: Duration { seconds: 0, nanoseconds: 184460400 }
        let start = Instant::now();
        let path: String = "./tests/big_files/test.gfa".to_string();
        let gfa2: GFA2 = gfa_file_to_gfa2(path.clone());
        println!("Convert file from GFA to GFA2 {:?}", start.elapsed());
        match save_on_file(
            ObjectType::GFA2(gfa2),
            Some(format!("{}{}", path, "2")),
        ) {
            Ok(_) => println!("File converted and saved correctly!"),
            Err(why) => println!("Error: {}", why),
        }
    }
}
