use tree_sitter_highlight::Highlighter; 
use tree_sitter_highlight::Highlight; 
use tree_sitter_highlight::HighlightConfiguration;

extern crate tree_sitter_manchester; //this is specified in Cargo.toml
use tree_sitter_highlight::HighlightEvent;


#[derive(Debug)]
pub struct HighlightSlice {
    syntax_element: String,
    highlight: Option<String>, 
}

pub fn get_highlight_code(h:  Highlight) -> usize { 
        match h {
            Highlight(x) => x,
        }
}

pub fn highlight(input: &str) -> Vec<HighlightSlice>  {
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
    let mut stack = Vec::new();

    for event in highlights {
        match event.unwrap() {
            HighlightEvent::Source {start, end} => {

                let syntax_element: &str = &input[start..end]; 
                //retrieve highlighting from stack
                if ! stack.is_empty() {
                    let slice = HighlightSlice{syntax_element:String::from(syntax_element),
                                               highlight: Some(String::from(stack[0])),
                                               };
                    res.push(slice);
                } else {
                    let slice = HighlightSlice{syntax_element:String::from(syntax_element),
                                               highlight: None };
                    res.push(slice); 
                } 
            },
            HighlightEvent::HighlightStart(s) => {
                stack.push(highlight_names[get_highlight_code(s)]);
            },
            HighlightEvent::HighlightEnd => {
                stack.pop();
            },
        }
    }
    res 
}
