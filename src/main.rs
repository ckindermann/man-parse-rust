use tree_sitter::{Parser, Language, Node};
use serde_json::{Value};
use serde_json::json; 

extern crate tree_sitter_manchester;
use tree_sitter_manchester::language;

//tree transducer for manchester syntax parse tree
//I distinguish between three cases:
//(i) nodes with a single child
//(ii) nodes with more than one child
//(iii) nodes with no child
//
//The reasons for this is as follows: 
//(i):   most nodes with only one child are to be removed from the parse tree. 
//       So, the default behavior of the tree transducer for such nodes is
//       to contract edges stemming from such nodes.
//
//(ii):  here we need an exhaustive enumeration of all cases
//
//(iii): translation of terminal symbols (which are not necessarily stored by tree-sitter itself)
pub fn translate(n : &Node, raw : &str) -> Value {

    let child_count = n.named_child_count(); 

    if child_count == 1 { //transduce single child nodes
         return match n.kind() {
             "primaryNegation" => translate_operator(&n.named_child(0).unwrap(), raw, "ComplementOf"),
             "restrictionNegation" => translate_operator(&n.named_child(0).unwrap(), raw, "ComplementOf"),
             "objectPropertySelf" => translate_operator(&n, raw, "HasSelf"), 
             _ => translate(&n.named_child(0).unwrap(), raw),//default for (i)
         }
    } else { //transduce 
         return match n.kind() {
             "subClassOf" => translate_operator(&n, raw, "SubClassOf"), 
             "equivalentTo" => translate_operator(&n, raw, "EquivalentClasses"), 
             "disjointWith" => translate_operator(&n, raw, "DisjointClasses"), 
             "disjointUnionOf" => translate_operator(&n, raw, "DisjointUnionOf"), 
             "description" => translate_operator(&n, raw, "UnionOf"), 
             "conjunction" => translate_operator(&n, raw, "IntersectionOf"), 
             "objectPropertyExistential" => translate_operator(&n, raw, "SomeValuesFrom"), 
             "objectPropertyUniversal" => translate_operator(&n, raw, "AllValuesFrom"), 
             "objectPropertyValue" => translate_operator(&n, raw, "HasValue"), 
             "qualifiedObjectMinCardinality" => translate_operator(&n, raw, "QualifiedMinCardinality"), 
             "unqualifiedObjectMinCardinality" => translate_operator(&n, raw, "MinCardinality"), 
             "qualifiedObjectMaxCardinality" => translate_operator(&n, raw, "QualifiedMaxCardinality"), 
             "unqualifiedObjectMaxCardinality" => translate_operator(&n, raw, "MaxCardinality"), 
             "qualifiedObjectExactCardinality" => translate_operator(&n, raw, "QualifiedExactCardinality"), 
             "unqualifiedObjectExactCardinality" => translate_operator(&n, raw, "ExactCardinality"), 
             _ => translate_raw(&n, raw), //translation of terminal symbols, i.e. case (iii)
         }; 
    }
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

fn main() {
    //let code = "((b or c) and d) and e"; 
    //let code = "a or bbaba and ddd"; 
    //let code = "not((not a) and b)";
    //let code = "a some p or b only d";
    //let code = "a exactly 2 asd";
    //let code = "a some asd";
    //let code = "a exactly 2 asd or b some d:asdA";
    //let code = "(aa some (b or c)) and (p some d)";
    //let code = "(bo1 some (obo2 or obo3)) and (obo4 some obo5)";
    //let code = "oboOBI_0000293 some (oboIAO_0000010 or oboIAO_0000096)";
    //let code = "obo:OBI_0000293 some (obo:IAO_0000010 or obo:IAO_0000096)";
     //let code = "(obo:OBI_0000293 some (obo:IAO_0000010 or obo:IAO_0000096)) and (obo:OBI_0000299 some obo:IAO_0000010)";
    //let code = "obo:OBI_0001875 and (obo:OBI_0000643 some (obo:CL_0000000 and (not (obo:BFO_0000051 some obo:OBI_1110132)) and (not (obo:BFO_0000051 some obo:PR_000001004))))";

    //let code = "test DisjointWith Student or Person that hasAge some number";
    let code = "test DisjointUnionOf {a, b}";

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

    let t = translate(&tree.root_node(), code);
    println!("Translation: {:#?}", t);
    println!("");
    println!("Serialisation: {:?}", serde_json::to_string(&t).unwrap());


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

