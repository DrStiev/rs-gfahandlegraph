use gfahandlegraph::parser::*;

// ONLY TO ENABLE DEBUGGER
// see: https://www.forrestthewoods.com/blog/how-to-debug-rust-with-visual-studio-code/
// make tests into the main function to use breakpoint and debugger info
fn main() {
    println!("Hello world");
    can_create_graph_from_gfa2_file();
}

fn can_create_graph_from_gfa2_file() {
    let parser: Parser<usize> = Parser::new();
    match parser
        .parse_file_to_graph("D:\\GitHub\\rs-gfahandlegraph\\tests\\gfa2_files\\spec_q7.gfa2")
    {
        Ok(g) => g.print_graph(),
        Err(why) => println!("Error {}", why),
    }
}
