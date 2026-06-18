use std::fs;
mod lex;

fn main() {
    let test: &str = "test";
    let input: String = fs::read_to_string(format!("tests/{test}.cb"))
        .unwrap_or_default();
    let output: String = lex::lex_file(&input).iter().map(|a| a.v.as_str()).collect::<Vec<_>>().join("\n");
    fs::write("tests/test_output.txt", output)
        .expect("Failed to write output");
}
