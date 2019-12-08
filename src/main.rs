#[macro_use]
extern crate lazy_static;
use std::collections::*;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use rand::{thread_rng, Rng};
use regex::Regex;
use railroad::*;

fn syntax_diagram_expr(expansion: &str) -> String {
    println!("expansion: {}", expansion);
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(<[^<> ]*>)").unwrap();
    }
    let symbols: Vec<&str> = RE.split(expansion)
                                //.map(|e| e.as_str())
                                .filter(|&e| e != "")
                                .collect();
    println!("{:#?}", symbols);
    let mut seq = Sequence::default();
    seq.push(Box::new(Start));
    for sym in symbols {
        if is_nonterminal(sym) {
            seq.push(Box::new(NonTerminal::new(sym.to_owned())));
        } 
        seq.push(Box::new(Terminal::new(sym.to_owned())));
    }
    seq.push(Box::new(End));

    let mut dia = Diagram::new(seq);
    dia.add_element(svg::Element::new("style")
                    .set("type", "text/css")
                    .raw_text(DEFAULT_CSS));
    
    format!("<html>{}</html>", dia)
}

fn is_nonterminal(expansion: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(<[^<> ]*>)").unwrap();
    }
    RE.is_match(expansion)
}

fn nonterminals(expansion: &str) -> Vec<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(<[^<> ]*>)").unwrap();
    }
    RE.find_iter(expansion).map(|e| e.as_str()).collect()
}

fn print_grammar(grammar: &HashMap<&'static str, Vec<String>>) {
    for (key, value) in grammar.iter() {
        println!("KEY: {}", key);
        print!("VALUES: | ");
        for v in value.iter() {
            print!("{} | ", v);
        }
        println!()
    }
}

fn build_expr_grammer() -> HashMap<&'static str, Vec<String>> {
    let mut expr_grammar: HashMap<&str, Vec<String>> = HashMap::new();

    expr_grammar.insert("<start>", vec!["<expr>".to_string()]);
    expr_grammar.insert("<expr>",
        vec![
            "<term> + <expr>".to_string(),
            "<term> - <expr>".to_string(),
            "<term>".to_string(),
        ]
    );
    expr_grammar.insert("<term>",
        vec![
            "<factor> * <term>".to_string(),
            "<factor> / <term>".to_string(),
            "<factor>".to_string(),
        ]
    );
    expr_grammar.insert("<factor>",
        vec![
            "+<factor>".to_string(),
            "-<factor>".to_string(),
            "(<expr>)".to_string(),
            "<integer>.<integer>".to_string(),
            "<integer>".to_string(),
        ]
    );
    expr_grammar.insert("<integer>",
        vec![
            "<digit><integer>".to_string(),
            "<digit>".to_string(),
        ]
    );
    expr_grammar.insert("<digit>",
        vec![
            "0".to_string(),
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
            "5".to_string(),
            "6".to_string(),
            "7".to_string(),
            "8".to_string(),
            "9".to_string(),
        ]
    );

    expr_grammar
}

fn simple_grammar_fuzzer(
    grammar: &HashMap<&'static str, Vec<String>>, 
    start_symbol: String, 
    max_nonterminals: usize, 
    max_expansion_trails: u32, 
    log: bool, 
) -> String {
    let mut term = start_symbol.clone();
    let mut expansion_trails = 0;

    while nonterminals(&term).len() > 0 {
        let term_borrowed = term.clone();

        let symbol_to_expand: &str = nonterminals(&term_borrowed).get(
            thread_rng().gen_range(0, nonterminals(&term_borrowed).len())
        ).unwrap();

        let expansions = grammar.get(symbol_to_expand).unwrap();
        let expansion = expansions.get(
            thread_rng().gen_range(0, expansions.len())
        ).unwrap();

        let new_term = term.replace(symbol_to_expand, expansion);
        if nonterminals(&new_term).len() < max_nonterminals {
            term = new_term.clone();
            if log == true {
                println!("{} -> {}          {}",
                    symbol_to_expand, expansion, term
                );
            }
            expansion_trails = 0;
        } else {
            expansion_trails += 1;
            if expansion_trails >= max_expansion_trails {
                panic!("Cannot expand {}", term)
            }
        }
    }
    term
}

fn tests() {
    assert_eq!(
        nonterminals("<term> * <factor>"),
        vec!["<term>", "<factor>"]
    );
    assert_eq!(
        nonterminals("<digit><integer>"),
        vec!["<digit>", "<integer>"]
    );
    let empty: Vec<&str> = Vec::new();
    assert_eq!(
        nonterminals("1 < 3 > 2"),
        empty 
    );
    assert_eq!(
        nonterminals("1 <3> 2"),
        vec!["<3>"]
    );
    assert_eq!(
        nonterminals("1 + 2"),
        empty
    );
    assert_eq!(
        nonterminals("<1>"),
        vec!["<1>"]
    );


    assert_eq!(is_nonterminal("<abc>"), true);
    assert_eq!(is_nonterminal("<blah-1>"), true);
    assert_eq!(is_nonterminal("+"), false);
}

fn create_syntax_diagram(
    grammar: &HashMap<&'static str, Vec<String>>,
    term: &str
) {
    let mut diagram = match File::create("diagram.html") {
        Err(why) => panic!("couldn't create diagram.html: {}", why.description()),
        Ok(diagram) => diagram,
    };
    match diagram.write_all(
        syntax_diagram_expr(&grammar.get(&term)
                            .unwrap()[0])
                            .to_owned()
                            .as_bytes()
    ) {
        Err(why) => panic!("couldn't write to diagram.html: {}", why.description()),
        Ok(_) => println!("successfully wrote to diagram.html"),
    }
}

fn main() {
    tests();

    let expr_grammar = build_expr_grammer();
    //print_grammar(&expr_grammar);

    //println!("{:#?}", expr_grammar.get("<digit>"));
    create_syntax_diagram(&expr_grammar, "<expr>");

    //for _ in 1..10 {
    //    println!("{}", simple_grammar_fuzzer(
    //        &expr_grammar,
    //        "<start>".to_string(),
    //        10,
    //        100,
    //        false
    //    ));
    //}
}
