use bstr::BString;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::gfa::gfa2::{Edge, GroupO, Header, Segment, GFA2};

/// Very BASIC converter from GFA1 format to GFA2 format.\
/// For now it consider only S-, L- and P- lines.
/// ignoring all the others.
/// WIP
pub fn gfa_file_to_gfa2(path: String) -> GFA2<BString> {
    let mut gfa2 = GFA2::new();
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut line_split = line.split_whitespace();
        let prefix = line_split.next().unwrap();

        match prefix {
            "H" => {
                // default header
                // ignore tag fields
                gfa2.headers = vec![Header::new(b"VN:Z:2.0", b"")];
            }
            "S" => {
                let id = BString::from(line_split.next().unwrap());
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
                let id = BString::from("*");

                let from_node = BString::from(line_split.next().unwrap());
                let from_node_orient = BString::from(line_split.next().unwrap());
                let to_node = BString::from(line_split.next().unwrap());
                let to_node_orient = BString::from(line_split.next().unwrap());

                // placeholder values
                /* real values or should they
                (++) = 100-align 100$ 0 align
                (--) = 0 align 100-align 100$
                (-+) = 0 align 0 align
                (+-) = 100-align 100$ 100-align 100$

                but what if align > 100?
                */
                let beg1 = BString::from("0");
                let end1 = BString::from("0$");
                let beg2 = BString::from("0");
                let end2 = BString::from("0$");

                let alignment = BString::from(line_split.next().unwrap());

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
                    sid1: BString::from(format!("{}{}", from_node, from_node_orient)),
                    sid2: BString::from(format!("{}{}", to_node, to_node_orient)),
                    beg1,
                    end1,
                    beg2,
                    end2,
                    alignment,
                    tag,
                };
                gfa2.edges.push(edge);
            }
            // TODO: the C-line should be inserted with the same L-line in the
            // corresponding E-line, but IDK how to do it
            //"C" => (),
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
            // ignore all the other lines (typically comment-lines)
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
        Convert file from GFA to GFA2: Duration { seconds: 285, nanoseconds: 130599900 }
        Save big gfa2 file: Duration { seconds: 46, nanoseconds: 674808000 }
        */
        let start = Instant::now();
        let path: String = "./tests/big_files/ape-4-0.10b.gfa".to_string();
        let gfa2: GFA2<BString> = gfa_file_to_gfa2(path.clone());
        println!("Convert file from GFA to GFA2 {:?}", start.elapsed());
        let start = Instant::now();
        match save_on_file(
            ObjectType::GFA2BSTRING(gfa2),
            Some(format!("{}{}", path, "2")),
        ) {
            Ok(_) => println!("Save big gfa2 file {:?}", start.elapsed()),
            Err(why) => println!("Error: {}", why),
        }
    }

    #[test]
    //#[ignore]
    fn can_parse_and_wirte_file_with_tags() {
        // Convert file from GFA to GFA2: Duration { seconds: 0, nanoseconds: 140577000 }
        let start = Instant::now();
        let path: String = "./tests/big_files/test.gfa".to_string();
        let gfa2: GFA2<BString> = gfa_file_to_gfa2(path.clone());
        println!("Convert file from GFA to GFA2 {:?}", start.elapsed());
        match save_on_file(
            ObjectType::GFA2BSTRING(gfa2),
            Some(format!("{}{}", path, "2")),
        ) {
            Ok(_) => println!("File converted and saved correctly!"),
            Err(why) => println!("Error: {}", why),
        }
    }
}
