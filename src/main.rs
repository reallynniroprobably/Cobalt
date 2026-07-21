use std::fs;
mod lexer;

fn main() {
    let test: &str = "test";
    let input: String = fs::read_to_string(format!("tests/{test}.cb"))
        .expect("Failed to read input from file");
    let output: Vec<lexer::Token> = lexer::tokenise(&input);
    fs::write(format!("tests/{test}_output.txt"), output.iter().map(|s| s.raw.clone() + " " + (if s.token_type == lexer::TokenType::Identifer { "identifer" } else { "other" })).collect::<Vec<_>>().join("\n"))
        .expect("Failed to write output to file");
}
