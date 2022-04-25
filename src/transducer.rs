use serde_json::{Value};
use serde_json::json; 
use tree_sitter::{Node};

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
