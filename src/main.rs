use tree_sitter::{Parser, Language};

extern crate tree_sitter_manchester; //this is specified in Cargo.toml

mod demo_highlight;
mod highlight;
mod transducer; 
mod syntax_checker;

fn main() {
    //let manchester_string = "g SubClassOf: (aa some (b or c)) and (p some d)";
    //let manchester_string = "(bo1 some (obo2 or obo3)) and (obo4 some obo5)";
    //let manchester_string = "oboOBI_0000293 some (oboIAO_0000010 or oboIAO_0000096)";
    //let manchester_string = "obo:OBI_0000293 some (obo:IAO_0000010 or obo:IAO_0000096)";
     //let manchester_string = "(obo:OBI_0000293 some (obo:IAO_0000010 or obo:IAO_0000096)) and (obo:OBI_0000299 some obo:IAO_0000010)";
    //let manchester_string = "obo:OBI_0001875 and (obo:OBI_0000643 some (obo:CL_0000000 and (not (obo:BFO_0000051 some obo:OBI_1110132)) and (not (obo:BFO_0000051 some obo:PR_000001004))))"; 
    //let manchester_string = "obo:OBI_0000070 DisjointWith: obo:OBI_0000339, obo:OBI_0200000, obo:OBI_0600013";
    //let manchester_string = "obo:IAO_0000078 EquivalentTo: obo:IAO_0000002 , obo:IAO_0000120 , obo:IAO_0000121 , obo:IAO_0000122 , obo:IAO_0000123 , obo:IAO_0000124 , obo:IAO_0000125 , obo:IAO_0000423 , obo:IAO_0000428";
    //let manchester_string = "obo:OBI_0000070 SubClassOf: obo:OBI_0000299 some (obo:IAO_0000027 and (obo:IAO_0000136 only (obo:BFO_0000040 or (not (obo:RO_0000087 some obo:OBI_0000067)))))";
    //let manchester_string = "obo:OBI_0000070 SubClassOf: <obo:OBI_0000299 some (obo:IAO_0000027 and (obo:IAO_0000136 only (<obo:BFO_0000040> or (not (obo:RO_0000087 some obo:OBI_0000067)))))";
    let manchester_string = "obo:OBI_0000070 SubClassOf: <obo:OBI_0000299";

    let mut parser = Parser::new();

    parser.set_language(tree_sitter_manchester::language()).expect("Error loading manchester grammar");

    let tree = parser.parse(manchester_string, None).unwrap();


    //println!("Has errors: {:#?}", tree.has_error());

    println!("Tree: {:#?}", tree);
    println!("");
    println!("S-Expression: {:#?}", tree.root_node().to_sexp());
    println!("");
    println!("Has Errors: {:#?}", syntax_checker::has_errors(&tree));
    println!("");
    println!("Error Vec: {:#?}", syntax_checker::get_errors(&tree));


    let t = transducer::translate(&tree.root_node(), manchester_string);
    println!("Translation to Serde Value: {:#?}", t);
    println!("");

    //println!("Serialisation: {:?}", serde_json::to_string(&t).unwrap());
    println!("Serialisation: {}", serde_json::to_string(&t).unwrap());
    println!("");

    let man_highlight = highlight::highlight(manchester_string);
    println!("Manchester String: {}", manchester_string);
    println!("Highlighted: {:#?}", man_highlight);
    //println!("Highlighted: {:?}", man_highlight); 

    //demo_highlight::highlight("obo:IAO_0000136 only (obo:BFO_0000040 or (not (obo:RO_0000087 some obo:OBI_0000067)))");


    //let mut rc = tree.root_node().child(0).unwrap().walk();
    //for x in tree.root_node().children(&mut rc){ 
    //    println!("Kind {:#?}", x.kind());
    //} 

    //Example for walking a tree
    //let tc = tree.walk();
    //println!("{:#?}", tc.node());
    //println!("{:#?}", tc.goto_first_child());
    //println!("{:#?}", tc.node());
    //println!("{:#?}", tc.goto_first_child());
    //println!("{:#?}", tc.node());

    //let start = tc.node().start_position().column;
    //let end = tc.node().end_position().column;
    //let extract = &manchester_string[start..end];
    //println!("{:#?}", extract);

    //let n = tc.node();
    //println!("---");
}

//pub fn translate_description(n : &Node, raw : &str) -> Value { 
//    let child_count = n.named_child_count();
//    let mut res = Vec::new();
//    res.push(json!("UnionOf"));
//
//    for i in 0..child_count {
//        res.push(translate(&n.named_child(i).unwrap(), raw));
//    }
//    json!(res)
//}

