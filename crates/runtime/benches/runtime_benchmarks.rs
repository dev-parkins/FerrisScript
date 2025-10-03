use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use ferrisscript_compiler::compile;
use ferrisscript_runtime::{execute, Env, Value, call_function};

fn compilation_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("compilation");
    
    // Simple function
    let simple = r#"
        fn add(x: i32, y: i32) -> i32 {
            return x + y;
        }
    "#;
    group.bench_with_input(
        BenchmarkId::new("compile", "simple"),
        &simple,
        |b, input| {
            b.iter(|| {
                let _ = compile(black_box(input));
            });
        },
    );
    
    // Complex function
    let complex = r#"
        fn factorial(n: i32) -> i32 {
            if n <= 1 {
                return 1;
            }
            return n * factorial(n - 1);
        }
    "#;
    group.bench_with_input(
        BenchmarkId::new("compile", "complex"),
        &complex,
        |b, input| {
            b.iter(|| {
                let _ = compile(black_box(input));
            });
        },
    );
    
    group.finish();
}

fn execution_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("execution");
    
    // Simple arithmetic
    let simple_source = r#"
        fn add(x: i32, y: i32) -> i32 {
            return x + y;
        }
    "#;
    let simple_program = compile(simple_source).unwrap();
    
    group.bench_function("simple_arithmetic", |b| {
        b.iter(|| {
            let mut env = Env::new();
            execute(black_box(&simple_program), &mut env).unwrap();
            let _ = call_function("add", &[Value::Int(10), Value::Int(20)], &mut env);
        });
    });
    
    // Control flow
    let control_flow_source = r#"
        fn max(x: i32, y: i32) -> i32 {
            if x > y {
                return x;
            } else {
                return y;
            }
        }
    "#;
    let control_flow_program = compile(control_flow_source).unwrap();
    
    group.bench_function("control_flow", |b| {
        b.iter(|| {
            let mut env = Env::new();
            execute(black_box(&control_flow_program), &mut env).unwrap();
            let _ = call_function("max", &[Value::Int(10), Value::Int(20)], &mut env);
        });
    });
    
    // Loops
    let loop_source = r#"
        fn sum_to_n(n: i32) -> i32 {
            let mut sum: i32 = 0;
            let mut i: i32 = 1;
            while i <= n {
                sum = sum + i;
                i = i + 1;
            }
            return sum;
        }
    "#;
    let loop_program = compile(loop_source).unwrap();
    
    group.bench_function("loop_10_iterations", |b| {
        b.iter(|| {
            let mut env = Env::new();
            execute(black_box(&loop_program), &mut env).unwrap();
            let _ = call_function("sum_to_n", &[Value::Int(10)], &mut env);
        });
    });
    
    group.bench_function("loop_100_iterations", |b| {
        b.iter(|| {
            let mut env = Env::new();
            execute(&loop_program, &mut env).unwrap();
            let _ = call_function("sum_to_n", &[Value::Int(100)], &mut env);
        });
    });
    
    group.finish();
}

fn recursion_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("recursion");
    
    let factorial_source = r#"
        fn factorial(n: i32) -> i32 {
            if n <= 1 {
                return 1;
            }
            return n * factorial(n - 1);
        }
    "#;
    let factorial_program = compile(factorial_source).unwrap();
    
    for depth in [5, 10, 20].iter() {
        group.bench_with_input(
            BenchmarkId::new("factorial", depth),
            depth,
            |b, &depth| {
                b.iter(|| {
                    let mut env = Env::new();
                    execute(&factorial_program, &mut env).unwrap();
                    let _ = call_function("factorial", &[Value::Int(depth)], &mut env);
                });
            },
        );
    }
    
    group.finish();
}

fn variable_operations_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("variables");
    
    // Variable declaration and access
    let var_source = r#"
        fn test_vars() -> i32 {
            let x: i32 = 10;
            let y: i32 = 20;
            let z: i32 = 30;
            return x + y + z;
        }
    "#;
    let var_program = compile(var_source).unwrap();
    
    group.bench_function("local_vars", |b| {
        b.iter(|| {
            let mut env = Env::new();
            execute(black_box(&var_program), &mut env).unwrap();
            let _ = call_function("test_vars", &[], &mut env);
        });
    });
    
    // Mutable variable updates
    let mut_source = r#"
        fn test_mutable() -> i32 {
            let mut x: i32 = 0;
            x = 10;
            x = x + 5;
            x = x * 2;
            return x;
        }
    "#;
    let mut_program = compile(mut_source).unwrap();
    
    group.bench_function("mutable_updates", |b| {
        b.iter(|| {
            let mut env = Env::new();
            execute(black_box(&mut_program), &mut env).unwrap();
            let _ = call_function("test_mutable", &[], &mut env);
        });
    });
    
    group.finish();
}

fn type_operations_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("types");
    
    // Integer operations
    let int_source = r#"
        fn int_ops(x: i32, y: i32) -> i32 {
            let a: i32 = x + y;
            let b: i32 = x - y;
            let c: i32 = x * y;
            let d: i32 = x / y;
            return a + b + c + d;
        }
    "#;
    let int_program = compile(int_source).unwrap();
    
    group.bench_function("integer_arithmetic", |b| {
        b.iter(|| {
            let mut env = Env::new();
            execute(black_box(&int_program), &mut env).unwrap();
            let _ = call_function("int_ops", &[Value::Int(100), Value::Int(10)], &mut env);
        });
    });
    
    // Float operations
    let float_source = r#"
        fn float_ops(x: f32, y: f32) -> f32 {
            let a: f32 = x + y;
            let b: f32 = x - y;
            let c: f32 = x * y;
            let d: f32 = x / y;
            return a + b + c + d;
        }
    "#;
    let float_program = compile(float_source).unwrap();
    
    group.bench_function("float_arithmetic", |b| {
        b.iter(|| {
            let mut env = Env::new();
            execute(black_box(&float_program), &mut env).unwrap();
            let _ = call_function("float_ops", &[Value::Float(100.0), Value::Float(10.0)], &mut env);
        });
    });
    
    // Boolean operations
    let bool_source = r#"
        fn bool_ops(x: i32, y: i32) -> bool {
            return (x > y) && (x != 0) || (y < 0);
        }
    "#;
    let bool_program = compile(bool_source).unwrap();
    
    group.bench_function("boolean_logic", |b| {
        b.iter(|| {
            let mut env = Env::new();
            execute(black_box(&bool_program), &mut env).unwrap();
            let _ = call_function("bool_ops", &[Value::Int(10), Value::Int(5)], &mut env);
        });
    });
    
    group.finish();
}

criterion_group!(
    benches,
    compilation_benchmarks,
    execution_benchmarks,
    recursion_benchmarks,
    variable_operations_benchmarks,
    type_operations_benchmarks
);
criterion_main!(benches);
