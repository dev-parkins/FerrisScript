/// Simple CLI tool to test FerrisScript compilation and see error messages
/// 
/// Usage: cargo run --example test_ferris -- <path-to-file>
/// Example: cargo run --example test_ferris -- examples/error_showcase.ferris

use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: cargo run --example test_ferris -- <path-to-ferris-file>");
        eprintln!("Example: cargo run --example test_ferris -- examples/hello.ferris");
        process::exit(1);
    }
    
    let file_path = &args[1];
    
    println!("Testing FerrisScript file: {}", file_path);
    println!("{}", "=".repeat(60));
    println!();
    
    let source = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            process::exit(1);
        }
    };
    
    match ferrisscript_compiler::compile(&source) {
        Ok(_ast) => {
            println!("✓ Compilation successful!");
            println!();
            println!("The script compiled without errors.");
            println!("To execute it, load it in Godot with the FerrisScript GDExtension.");
            println!();
        }
        Err(error) => {
            println!("✗ Compilation failed:");
            println!();
            println!("{}", error);
            println!();
            println!("{}", "=".repeat(60));
            println!("Error messages include:");
            println!("  • Source context (±2 lines around the error)");
            println!("  • Visual pointer (^) at the error location");
            println!("  • Helpful hint explaining what's expected");
            println!();
            process::exit(1);
        }
    }
}
