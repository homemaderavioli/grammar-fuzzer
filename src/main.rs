use std::collections::*;



fn main() {
    let expr_grammar = build_expr_grammer();

    //println!("{:#?}", expr_grammar.get("<digit>"));

    print_grammar(expr_grammar);
}

fn print_grammar(grammar: HashMap<&'static str, Vec<String>>) {
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
