use tree_sitter_highlight::Highlighter; 
use tree_sitter_highlight::Highlight; 
use tree_sitter_highlight::HighlightConfiguration;

extern crate tree_sitter_manchester; //this is specified in Cargo.toml
use tree_sitter_highlight::HighlightEvent;


pub fn get_highlight_code(h:  Highlight) -> usize { 
        match h {
            Highlight(x) => x,
        }
}

pub fn highlight(input: &str) {
    let mut highlighter = Highlighter::new(); 

    let mut man_config = HighlightConfiguration::new(
        tree_sitter_manchester::language(),
        tree_sitter_manchester::HIGHLIGHTS_QUERY,
        "",
        "",
        ).unwrap();

    let highlight_names = &[
    "magenta",
    "blue",
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

    println!("Highliting start");
    for event in highlights {
        match event.unwrap() {
            HighlightEvent::Source {start, end} => {
                eprintln!("source: {}-{}", start, end);
                eprintln!("source: {:?}", &input[start..end]); 
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
}
