use crate::parser::Stmt;

pub struct Interpret;

impl Interpret {
    pub fn interpret(stmts: &[Stmt]) {
        for stmt in stmts {
            Self::execute(stmt);
        }
    }

    fn execute(stmt: &Stmt) {
        match stmt {
            Stmt::Expression(expr) => {
                expr.evaluate();
            }
            Stmt::Print(expr) => {
                println!("{}", expr.evaluate().to_string());
            }
        }
    }
}
