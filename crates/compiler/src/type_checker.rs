use crate::ast::Program;

pub fn check(_program: &Program) -> Result<(), String> {
    // Placeholder: type-check AST
    // TODO: Implement proper type checker in Phase 4
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Program;

    #[test]
    fn test_check_empty() {
        let program = Program::new();
        assert!(check(&program).is_ok());
    }
}
