use std::fs;
mod lexer;
mod parser;
mod types;

fn main() {
    let test: &str = "test";
    let input: String = fs::read_to_string(format!("tests/{test}.cb"))
        .expect("Failed to read input from file");
    let output: types::LexedFile = lexer::lex_program(&input);
    let s_output: types::TokenList = output.0;
    let t_output: types::TypeList = output.1;
    fs::write(format!("tests/{test}_output.txt"), s_output.join("\n"))
        .expect("Failed to write output to file");
    
}
