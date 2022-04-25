use tree_sitter::{Parser, Language};

extern crate tree_sitter_manchester; //this is specified in Cargo.toml
use tree_sitter_manchester::language;

mod demo_highlight;
mod highlight;
mod transducer; 

fn main() {
    //let code = "((b or c) and d) and e"; 
    //let code = "a or bbaba and ddd"; 
    //let code = "not((not a) and b)";
    //let code = "g SubClassOf: inverse l some not p or b only d";
    //let code = "a exactly 2 asd";
    //let code = "a some asd";
    //let code = "a exactly 2 asd or b some d:asdA";
    //let code = "g SubClassOf: (aa some (b or c)) and (p some d)";
    //let code = "(bo1 some (obo2 or obo3)) and (obo4 some obo5)";
    //let code = "oboOBI_0000293 some (oboIAO_0000010 or oboIAO_0000096)";
    //let code = "obo:OBI_0000293 some (obo:IAO_0000010 or obo:IAO_0000096)";
     //let code = "(obo:OBI_0000293 some (obo:IAO_0000010 or obo:IAO_0000096)) and (obo:OBI_0000299 some obo:IAO_0000010)";
    //let code = "obo:OBI_0001875 and (obo:OBI_0000643 some (obo:CL_0000000 and (not (obo:BFO_0000051 some obo:OBI_1110132)) and (not (obo:BFO_0000051 some obo:PR_000001004))))";
    //
    let code = "obo:OBI_0000070 SubClassOf: obo:OBI_0000299 some (obo:IAO_0000027 and (obo:IAO_0000136 only (obo:BFO_0000040 or (not (obo:RO_0000087 some obo:OBI_0000067)))))";

    //let code = "test DisjointWith Student or Person that hasAge some number";
    //let code = "test DisjointUnionOf a, b";
    //let code = "obo:OBI_0000070 DisjointWith: obo:OBI_0000339, obo:OBI_0200000, obo:OBI_0600013";
    //let code = "obo:IAO_0000078 EquivalentTo: obo:IAO_0000002 , obo:IAO_0000120 , obo:IAO_0000121 , obo:IAO_0000122 , obo:IAO_0000123 , obo:IAO_0000124 , obo:IAO_0000125 , obo:IAO_0000423 , obo:IAO_0000428";

    let mut parser = Parser::new();

    let language : Language = language();

    parser.set_language(language).expect("Error loading manchester grammar");
    let tree = parser.parse(code, None).unwrap();

    println!("Tree: {:#?}", tree);
    println!("");
    println!("");
    println!("S-Expression: {:#?}", tree.root_node().to_sexp());
    println!("");
    println!("");

    let t = transducer::translate(&tree.root_node(), code);
    println!("Translation: {:#?}", t);
    println!("");
    println!("Serialisation: {:?}", serde_json::to_string(&t).unwrap());
    println!("Serialisation: {}", serde_json::to_string(&t).unwrap());

    let man_highlight = highlight::highlight("obo:OBI_0000070 SubClassOf: obo:OBI_0000299 some (obo:IAO_0000027 and (obo:IAO_0000136 only (obo:BFO_0000040 or (not (obo:RO_0000087 some obo:OBI_0000067)))))");
    println!("Highlighted: {:#?}", man_highlight);
    //println!("Highlighted: {:?}", man_highlight); 

    //demo_highlight::highlight("obo:IAO_0000136 only (obo:BFO_0000040 or (not (obo:RO_0000087 some obo:OBI_0000067)))");


    //let mut rc = tree.root_node().child(0).unwrap().walk();
    //for x in tree.root_node().children(&mut rc){ 
    //    println!("Kind {:#?}", x.kind());
    //} 

    //Example for walking a tree
    //let mut tc = tree.walk();
    //println!("{:#?}", tc.node());
    //println!("{:#?}", tc.goto_first_child());
    //println!("{:#?}", tc.node());
    //println!("{:#?}", tc.goto_first_child());
    //println!("{:#?}", tc.node());

    //let start = tc.node().start_position().column;
    //let end = tc.node().end_position().column;
    //let extract = &code[start..end];
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

