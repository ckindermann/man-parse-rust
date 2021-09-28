use tree_sitter::{Parser, Language, Node};
use serde_json::{Value};
use serde_json::json; 

extern crate tree_sitter_manchester;
use tree_sitter_manchester::language;

//translation of a syntax node
pub fn translate(n : &Node, raw : &str) -> Value {
    let child_count = n.named_child_count(); 

    if child_count == 1 { //transduce single child nodes
         return match n.kind() {
             "primaryNegation" => translate_operator(&n.named_child(0).unwrap(), raw, "ComplementOf"),
             _ => translate(&n.named_child(0).unwrap(), raw),//just unpack
         }
    } else { //transduce -
         return match n.kind() {
             "description" => translate_operator(&n, raw, "UnionOf"), 
             "conjunction" => translate_operator(&n, raw, "IntersectionOf"), 
             "objectPropertyExistential" => translate_operator(&n, raw, "SomeValuesFrom"), 
             "objectPropertyUniversal" => translate_operator(&n, raw, "AllValuesFrom"), 
             "objectPropertySelf" => translate_operator(&n, raw, "HasSelf"), 
             "objectPropertyValue" => translate_operator(&n, raw, "HasValue"), 
             _ => translate_raw(&n, raw),
         }; 
    } 
    //let res : [Value; child_count] = [json!("an"), json!("array")];
}

pub fn translate_raw(n : &Node, raw : &str) -> Value { 
    let start = n.start_position().column;
    let end = n.end_position().column;
    let extract = &raw[start..end];
    json!(extract) 
}

pub fn translate_operator(n : &Node, raw : &str, operator : &str) -> Value { 
    let child_count = n.named_child_count();
    let mut res = Vec::new();
    res.push(json!(operator));

    for i in 0..child_count {
        res.push(translate(&n.named_child(i).unwrap(), raw));
    }
    json!(res)
}

pub fn translate_description(n : &Node, raw : &str) -> Value { 
    let child_count = n.named_child_count();
    let mut res = Vec::new();
    res.push(json!("UnionOf"));

    for i in 0..child_count {
        res.push(translate(&n.named_child(i).unwrap(), raw));
    }
    json!(res)
}

fn main() {
    //let code = "((b or c) and d) and e"; 
    //let code = "a or bbaba and ddd"; 
    //let code = "not((not a) and b)";
    let code = "a some p or b only d";

    let mut parser = Parser::new();

    let language : Language = language();
    parser.set_language(language).expect("Error loading manchester grammar");
    let tree = parser.parse(code, None).unwrap();


    let mut tc = tree.walk();
    println!("{:#?}", tc.node());
    println!("{:#?}", tc.goto_first_child());
    println!("{:#?}", tc.node());
    println!("{:#?}", tc.goto_first_child());
    println!("{:#?}", tc.node());

    let start = tc.node().start_position().column;
    let end = tc.node().end_position().column;
    let extract = &code[start..end];
    println!("{:#?}", extract);

    let n = tc.node();
    println!("---");


    println!("Tree: {:#?}", tree);
    println!("");
    println!("S-Expression: {:#?}", tree.root_node().to_sexp());
    println!("");

    let t = translate(&tree.root_node(), code);
    println!("Translation: {:#?}", t);
    println!("");
    println!("Serialisation: {:?}", serde_json::to_string(&t).unwrap());


    //let mut rc = tree.root_node().child(0).unwrap().walk();
    //for x in tree.root_node().children(&mut rc){ 
    //    println!("AA{:#?}", x.kind());
    //} 
}
