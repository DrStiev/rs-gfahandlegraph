# GFA in Rust
This crate provides Rust types, parsers and builders for the GFA (Graphical
Fragment Assembly) format, version 1 and 2.\
The aim of this project is to provide an HandleGraph as result of the parsing, to make possible to modify a GFA file as a graph.\
Ultimately the program provides a method to convert the graph back into a GFA Object to save it into a file or to print it to the console.

## Compatibility
This library is compatible with the [version 2 specification](https://github.com/GFA-spec/GFA-spec/blob/master/GFA2.md) and [version 1 specification](https://github.com/GFA-spec/GFA-spec/blob/master/GFA1.md) of GFA.

This library tries to implement Variation graphs in Rust, based on the C++ library
[libhandlegraph](https://github.com/vgteam/libhandlegraph).

While this draws heavily on the C++ implementation for now,
compatibility is not a goal, and the API will surely diverge as
development proceeds.

## Usage
To create a Graph from a GFA file:
```rust
let parser: Parser = Parser::new();
match parser.parse_file_to_graph("./tests/gfa2_files/irl.gfa2") {
	Ok(g) => g.print_graph(),
	Err(why) => println!("Error {}", why),
}
```
GFA File:
```
S	A	10	AAAAAAACGT
S	X	10	ACGTCCACGT
S	B	10	ACGTGGGGGG
F	15	ex-	10	10	20	20	*
E	1	A+	X+	6	10$	0	4	4M
E	2	A+	X-	6	10$	6	10$	4M
E	3	X+	B+	6	10$	0	4	4M
O	1	A+ X+ B+
O	2	A+ X- B+
```
Resulting Graph:
```
Graph: {
        Nodes: {
                65: AAAAAAACGT
                88: ACGTCCACGT
                66: ACGTGGGGGG
        }
        Edges: {
                65+ -- 88+
                65+ -- 88-
                66- -- 88-
        }
        Paths: {
                2: AAAAAAACGT -> ACGTGGACGT -(ACGTCCACGT) -> ACGTGGGGGG
                1: AAAAAAACGT -> ACGTCCACGT -> ACGTGGGGGG    
        }
}
```
To convert a Graph back into a GFA file:
```rust
let parser: Parser = Parser::new();
match parser.parse_file_to_graph("./tests/gfa2_files/irl.gfa2") {
	Ok(g) => {
		g.print_graph();
		let mut _file: GFA2<BString> = GFA2::new();
		_file = to_gfa2(&g);
		println!("{}", _file);
	}
	Err(why) => println!("Error {}", why),
}
```
Graph:
```
Graph: {
        Nodes: {
                65: AAAAAAACGT
                88: ACGTCCACGT
                66: ACGTGGGGGG
        }
        Edges: {
                65+ -- 88+
                65+ -- 88-
                66- -- 88-
        }
        Paths: {
                2: AAAAAAACGT -> ACGTGGACGT -(ACGTCCACGT) -> ACGTGGGGGG
                1: AAAAAAACGT -> ACGTCCACGT -> ACGTGGGGGG    
        }
}
```
Resulting GFA File:
```
H       VN:Z:2.0
S       65      10      AAAAAAACGT
S       88      10      ACGTCCACGT
S       66      10      ACGTGGGGGG
E       *       65+     88+     0       0$      0       0$      0M
E       *       65+     88-     0       0$      0       0$      0M
E       *       66-     88-     0       0$      0       0$      0M
O       2       65+ 88- 66+
O       1       65+ 88+ 66+
```