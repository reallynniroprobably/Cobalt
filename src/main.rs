use std::fs;
mod lex;

fn main() {
    let test: &str = "test";
    let input: String = fs::read_to_string(format!("tests/{test}.cb"))
        .expect("Failed to read input from file");
    let output: lex::LexedFile = lex::lex_program(&input);
    let s_output = output.0;
    let t_output = output.1;
    fs::write(format!("tests/{test}_output.txt"), s_output.join("\n"))
        .expect("Failed to write output to file");
    
}
