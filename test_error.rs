use ferrisscript_compiler::compile;
fn main() {
    let source = r#"fn test() {
    let s = \"hello
}"#;
    match compile(source) {
        Ok(_) => println!(\"Success\"),
        Err(e) => println!(\"Error:\\n{}\", e),
    }
}
