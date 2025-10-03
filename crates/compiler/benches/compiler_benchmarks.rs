use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use ferrisscript_compiler::{lexer, parser, type_checker};

fn lexer_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("lexer");
    
    // Small input: single line
    let small_input = "let x: i32 = 42;";
    group.bench_with_input(
        BenchmarkId::new("tokenize", "small"),
        &small_input,
        |b, input| {
            b.iter(|| {
                let _ = lexer::tokenize(black_box(input));
            });
        },
    );
    
    // Medium input: function definition
    let medium_input = r#"
        fn calculate(x: i32, y: i32) -> i32 {
            let result: i32 = x + y;
            if result > 100 {
                return 100;
            }
            return result;
        }
    "#;
    group.bench_with_input(
        BenchmarkId::new("tokenize", "medium"),
        &medium_input,
        |b, input| {
            b.iter(|| {
                let _ = lexer::tokenize(black_box(input));
            });
        },
    );
    
    // Large input: bounce example
    let large_input = r#"
        fn update(delta: f32) {
            let mut velocity_y: f32 = get_velocity_y();
            let gravity: f32 = 980.0;
            velocity_y = velocity_y + gravity * delta;
            set_velocity_y(velocity_y);
            
            let mut pos_y: f32 = get_position_y();
            pos_y = pos_y + velocity_y * delta;
            
            if pos_y >= 500.0 {
                pos_y = 500.0;
                velocity_y = velocity_y * -0.8;
                set_velocity_y(velocity_y);
            }
            
            set_position_y(pos_y);
        }
    "#;
    group.bench_with_input(
        BenchmarkId::new("tokenize", "large"),
        &large_input,
        |b, input| {
            b.iter(|| {
                let _ = lexer::tokenize(black_box(input));
            });
        },
    );
    
    group.finish();
}

fn parser_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("parser");
    
    // Small input
    let small_input = "let x: i32 = 42;";
    group.bench_with_input(
        BenchmarkId::new("parse", "small"),
        &small_input,
        |b, input| {
            b.iter(|| {
                let tokens = lexer::tokenize(input).unwrap();
                let _ = parser::parse(black_box(&tokens));
            });
        },
    );
    
    // Medium input
    let medium_input = r#"
        fn calculate(x: i32, y: i32) -> i32 {
            let result: i32 = x + y;
            if result > 100 {
                return 100;
            }
            return result;
        }
    "#;
    group.bench_with_input(
        BenchmarkId::new("parse", "medium"),
        &medium_input,
        |b, input| {
            b.iter(|| {
                let tokens = lexer::tokenize(input).unwrap();
                let _ = parser::parse(black_box(&tokens));
            });
        },
    );
    
    // Large input
    let large_input = r#"
        fn update(delta: f32) {
            let mut velocity_y: f32 = get_velocity_y();
            let gravity: f32 = 980.0;
            velocity_y = velocity_y + gravity * delta;
            set_velocity_y(velocity_y);
            
            let mut pos_y: f32 = get_position_y();
            pos_y = pos_y + velocity_y * delta;
            
            if pos_y >= 500.0 {
                pos_y = 500.0;
                velocity_y = velocity_y * -0.8;
                set_velocity_y(velocity_y);
            }
            
            set_position_y(pos_y);
        }
    "#;
    group.bench_with_input(
        BenchmarkId::new("parse", "large"),
        &large_input,
        |b, input| {
            b.iter(|| {
                let tokens = lexer::tokenize(input).unwrap();
                let _ = parser::parse(black_box(&tokens));
            });
        },
    );
    
    group.finish();
}

fn type_checker_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("type_checker");
    
    // Small input
    let small_input = "let x: i32 = 42;";
    group.bench_with_input(
        BenchmarkId::new("check", "small"),
        &small_input,
        |b, input| {
            b.iter(|| {
                let tokens = lexer::tokenize(input).unwrap();
                let ast = parser::parse(&tokens).unwrap();
                let _ = type_checker::check(black_box(&ast));
            });
        },
    );
    
    // Medium input
    let medium_input = r#"
        fn calculate(x: i32, y: i32) -> i32 {
            let result: i32 = x + y;
            if result > 100 {
                return 100;
            }
            return result;
        }
    "#;
    group.bench_with_input(
        BenchmarkId::new("check", "medium"),
        &medium_input,
        |b, input| {
            b.iter(|| {
                let tokens = lexer::tokenize(input).unwrap();
                let ast = parser::parse(&tokens).unwrap();
                let _ = type_checker::check(black_box(&ast));
            });
        },
    );
    
    group.finish();
}

fn full_pipeline_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("full_pipeline");
    
    // Complete compilation pipeline
    let input = r#"
        fn calculate(x: i32, y: i32) -> i32 {
            let result: i32 = x + y;
            if result > 100 {
                return 100;
            }
            return result;
        }
    "#;
    
    group.bench_function("lex_parse_check", |b| {
        b.iter(|| {
            let tokens = lexer::tokenize(black_box(input)).unwrap();
            let ast = parser::parse(&tokens).unwrap();
            let _ = type_checker::check(&ast);
        });
    });
    
    group.finish();
}

criterion_group!(
    benches,
    lexer_benchmarks,
    parser_benchmarks,
    type_checker_benchmarks,
    full_pipeline_benchmarks
);
criterion_main!(benches);
