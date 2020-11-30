use bstr::{BString, ByteSlice};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::str;

use rayon::iter::{ParallelBridge, ParallelIterator};
use std::sync::Mutex;

/// Very BASIC converter from
/// [`GFA`](file:///D:/GitHub/rs-gfahandlegraph/target/doc/gfahandlegraph/gfa/gfa1/struct.GFA.html) format to
/// [`GFA2`](file:///D:/GitHub/rs-gfahandlegraph/target/doc/gfahandlegraph/gfa/gfa2/struct.GFA2.html) format.\
/// For now it consider only S-, L- and P- lines,
/// ignoring all the others.
pub fn gfa_file_to_gfa2(path: String) -> std::io::Result<()> {
    let res = Mutex::new(File::create(format!("{}{}", path, 2))?);
    let file = File::open(path)?;
    let reader = BufReader::new(file).lines();

    reader.par_bridge().for_each(|line| {
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
                let mut tag = opt_fields
                    .into_iter()
                    .map(|x| {
                        BString::from(
                            str::from_utf8(x).unwrap().to_owned() + "\t",
                        )
                    })
                    .collect::<BString>();
                tag.pop();

                res.lock()
                    .unwrap()
                    .write(format!("H\t{}\t{}\n", version, tag).as_bytes())
                    .expect("unable to write file");
            }
            "S" => {
                let id = line_split.next().unwrap().to_string();
                let sequence = BString::from(line_split.next().unwrap());
                let len = BString::from(sequence.len().to_string());

                let mut tag = line_split.next();
                let mut opt_fields: Vec<&[u8]> = vec![];
                while tag.is_some() {
                    opt_fields.push(tag.unwrap().as_bytes());
                    tag = line_split.next();
                }
                let mut tag = opt_fields
                    .into_iter()
                    .map(|x| {
                        BString::from(
                            str::from_utf8(x).unwrap().to_owned() + "\t",
                        )
                    })
                    .collect::<BString>();
                tag.pop();

                res.lock()
                    .unwrap()
                    .write(
                        format!("S\t{}\t{}\t{}\t{}\n", id, len, sequence, tag)
                            .as_bytes(),
                    )
                    .expect("unable to write file");
            }
            "L" => {
                // placeholder value
                let id = "*".to_string();

                let from_node = line_split.next().unwrap().to_string();
                let from_node_orient = line_split.next().unwrap().to_string();
                let to_node = line_split.next().unwrap().to_string();
                let to_node_orient = line_split.next().unwrap().to_string();
                let alignment = BString::from(line_split.next().unwrap());

                // placeholder values
                let mut beg1 = BString::from("0");
                let mut end1 = BString::from("0$");
                let mut beg2 = BString::from("0");
                let mut end2 = BString::from("0$");

                if alignment != "*" {
                    let len = alignment.len() - 1;
                    let dist = alignment[..len]
                        .to_str()
                        .unwrap()
                        .parse::<i64>()
                        .unwrap();

                    if from_node_orient == "+" && to_node_orient == "+" {
                        let x = (100 - dist).abs();
                        beg1 = BString::from(x.to_string());
                        end1 = BString::from("100$");
                        end2 = BString::from(dist.to_string());
                    } else if from_node_orient == "-" && to_node_orient == "-" {
                        let x = (100 - dist).abs();
                        end1 = BString::from(dist.to_string());
                        beg2 = BString::from(x.to_string());
                        end2 = BString::from("100$");
                    } else if from_node_orient == "-" && to_node_orient == "+" {
                        end1 = BString::from(dist.to_string());
                        end2 = BString::from(dist.to_string());
                    } else if from_node_orient == "+" && to_node_orient == "-" {
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
                let mut tag = opt_fields
                    .into_iter()
                    .map(|x| {
                        BString::from(
                            str::from_utf8(x).unwrap().to_owned() + "\t",
                        )
                    })
                    .collect::<BString>();
                tag.pop();

                res.lock()
                    .unwrap()
                    .write(
                        format!(
                            "E\t{}\t{}{}\t{}{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
                            id,
                            from_node,
                            from_node_orient,
                            to_node,
                            to_node_orient,
                            beg1,
                            end1,
                            beg2,
                            end2,
                            alignment,
                            tag
                        )
                        .as_bytes(),
                    )
                    .expect("unable to write file");
            }
            "P" => {
                let id = BString::from(line_split.next().unwrap());
                let seg_ids = line_split.next().unwrap();
                let var_field = BString::from(str::replace(seg_ids, ",", " "));

                let mut tag = line_split.next();
                let mut opt_fields: Vec<&[u8]> = vec![];
                while tag.is_some() {
                    opt_fields.push(tag.unwrap().as_bytes());
                    tag = line_split.next();
                }
                let mut tag = opt_fields
                    .into_iter()
                    .map(|x| {
                        BString::from(
                            str::from_utf8(x).unwrap().to_owned() + "\t",
                        )
                    })
                    .collect::<BString>();
                tag.pop();

                res.lock()
                    .unwrap()
                    .write(
                        format!("P\t{}\t{}\t{}\n", id, var_field, tag)
                            .as_bytes(),
                    )
                    .expect("unable to write file");
            }
            // ignore all the other lines (typically C- and comment-lines)
            _ => (),
        }
    });
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use time::Instant;

    #[test]
    #[ignore]
    fn can_parse_and_write_big_file() {
        /*
        Convert file from GFA to GFA2 Duration { seconds: 55, nanoseconds: 894165300 }
        Convert file from GFA to GFA2 Duration { seconds: 49, nanoseconds: 224287200 }
        Convert file from GFA to GFA2 Duration { seconds: 46, nanoseconds: 612642400 }
        */
        const FILES: [&str; 3] = [
            "./tests/big_files/ape-4-0.10b.gfa",
            "./tests/big_files/CHM13v1Y-GRCh38-HPP58-0.12.gfa",
            "./tests/big_files/GRCh38-20-0.10b.gfa",
        ];
        for i in 0..3 {
            let start = Instant::now();
            let path: String = FILES[i].to_string();
            match gfa_file_to_gfa2(path.clone()) {
                Err(why) => println!("Error: {}", why),
                _ => println!(
                    "Convert file from GFA to GFA2 {:?}",
                    start.elapsed()
                ),
            }
        }
    }

    #[test]
    fn can_parse_and_write_medium_file() {
        // Convert file from GFA to GFA2 Duration { seconds: 0, nanoseconds: 149387100 }
        let start = Instant::now();
        let path: String = "./tests/big_files/test.gfa".to_string();
        match gfa_file_to_gfa2(path.clone()) {
            Err(why) => println!("Error: {}", why),
            _ => {
                println!("Convert file from GFA to GFA2 {:?}", start.elapsed())
            }
        }
    }

    #[test]
    fn can_parse_and_write_medium_file_with_tag() {
        // Convert file from GFA to GFA2 Duration { seconds: 0, nanoseconds: 214957300 }
        let start = Instant::now();
        let path: String = "./tests/big_files/A-3105.sort.gfa".to_string();
        match gfa_file_to_gfa2(path.clone()) {
            Err(why) => println!("Error: {}", why),
            _ => {
                println!("Convert file from GFA to GFA2 {:?}", start.elapsed())
            }
        }
    }

    #[test]
    #[ignore]
    fn convert_files() {
        const FILES: [&str; 4] = [
            "./tests/big_files/A-3105.sort.gfa",
            "./tests/big_files/DRB1-3123.sort.gfa",
            "./tests/big_files/A-3105.gfa",
            "./tests/big_files/DRB1-3123.gfa",
        ];
        for i in 0..4 {
            let path = FILES[i].to_string();
            match gfa_file_to_gfa2(path.clone()) {
                Err(why) => println!("Error: {}", why),
                _ => (),
            }
        }
    }
}
