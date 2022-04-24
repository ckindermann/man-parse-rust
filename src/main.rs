use tree_sitter::{Parser, Language, Node};
use serde_json::{Value};
use serde_json::json; 
use tree_sitter_highlight::Highlighter; 
use tree_sitter_highlight::Highlight; 
use tree_sitter_highlight::HighlightConfiguration;

extern crate tree_sitter_manchester; //this is specified in Cargo.toml
use tree_sitter_manchester::language;
use tree_sitter_highlight::HighlightEvent;

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
             "inverseObjectProperty" => translate_operator(&n.named_child(0).unwrap(), raw, "InverseOf"),
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

//this gets atomic entities (labels,entity names,etc.)
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

#[derive(Debug)]
pub struct HighlightSlice {
    syntax: String,
    highlight: String, 
}

pub fn get_highlight_code(h:  Highlight) -> usize { 
        match h {
            Highlight(x) => x,
        }
}

pub fn highlight(input: &str) -> String  {
    let mut highlighter = Highlighter::new(); 

    let mut man_config = HighlightConfiguration::new(
        tree_sitter_manchester::language(),
        tree_sitter_manchester::HIGHLIGHTS_QUERY,
        "",
        "",
        ).unwrap();

    let highlight_names = &[
    "blue",
    "magenta",
    "turquise",
    "white",
    "gray",
    ];

    man_config.configure(&highlight_names[..]);
    let highlights = highlighter.highlight(
        &man_config,
        input.as_bytes(),
        None,
        |_| None
        ).unwrap();

    let mut res = Vec::new(); 
    let h = HighlightSlice{syntax:String::from("a"),
    highlight: String::from("b"), };
    res.push(h);
    println!("{:?}", res);

    //TODO: implement a stack to iterate thourhg this
    let mut stack = Vec::new();

    println!("Highliting start");
    for event in highlights {
        match event.unwrap() {
            HighlightEvent::Source {start, end} => {

                //let hh: usize = end;
                let hh: &str = &input[start..end];
                eprintln!("source: {}-{}", start, end);
                eprintln!("source: {:?}", &input[start..end]); 
                if ! stack.is_empty() {
                    println!("ah {:?}", stack[0]);
                } else {
                    println!("bh"); 
                }

    //let extract = &raw[start..end];
    //json!(extract) 
            },
            HighlightEvent::HighlightStart(s) => {
                eprintln!("highlight style started: {:?}", s);
                eprintln!("highlight style started: {:?}", highlight_names[get_highlight_code(s)]);
                stack.push(get_highlight_code(s));
            },
            HighlightEvent::HighlightEnd => {
                eprintln!("highlight style ended");
                stack.pop();
            },
        }
    }
    println!("Highliting end");

    String::from("asd")
}

fn main() {
    let mut highlighter = Highlighter::new(); 
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

    let mut man_config = HighlightConfiguration::new(
        tree_sitter_manchester::language(),
        tree_sitter_manchester::HIGHLIGHTS_QUERY,
        "",
        "",
        ).unwrap();

    let highlight_names = &[
    "blue",
    "magenta",
    "turquise",
    "white",
    "gray",
    ];

    man_config.configure(&highlight_names[..]);

        //let example_string = b"obo:OBI_0000070 SubClassOf: obo:OBI_0000299 some (obo:IAO_0000027 and (obo:IAO_0000136 only (obo:BFO_0000040 or (not (obo:RO_0000087 some obo:OBI_0000067)))))";
        let example_string_2 = "obo:OBI_0000070 SubClassOf: obo:OBI_0000299 some (obo:IAO_0000027 and (obo:IAO_0000136 only (obo:BFO_0000040 or (not (obo:RO_0000087 some obo:OBI_0000067)))))";

    let highlights = highlighter.highlight(
        &man_config,
        //b"obo:OBI_0000070 SubClassOf: obo:OBI_0000299 some (obo:IAO_0000027 and (obo:IAO_0000136 only (obo:BFO_0000040 or (not (obo:RO_0000087 some obo:OBI_0000067)))))",
        example_string_2.as_bytes(),
        None,
        |_| None
        ).unwrap();

    println!("Highliting start");
    for event in highlights {
        match event.unwrap() {
            HighlightEvent::Source {start, end} => {
                eprintln!("source: {}-{}", start, end);
                eprintln!("source: {:?}", &example_string_2[start..end]);

    //let extract = &raw[start..end];
    //json!(extract) 
            },
            HighlightEvent::HighlightStart(s) => {
                eprintln!("highlight style started: {:?}", s);
                eprintln!("highlight style started: {:?}", highlight_names[get_highlight_code(s)]);
            },
            HighlightEvent::HighlightEnd => {
                eprintln!("highlight style ended");
            },
        }
    }
    println!("Highliting end");



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
    println!("Serialisation: {}", serde_json::to_string(&t).unwrap());

    highlight("obo:OBI_0000070 SubClassOf: obo:OBI_0000299 some (obo:IAO_0000027 and (obo:IAO_0000136 only (obo:BFO_0000040 or (not (obo:RO_0000087 some obo:OBI_0000067)))))");


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

