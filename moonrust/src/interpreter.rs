use crate::ast::*;
use crate::interpreter::environment::Env;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::{cell::RefCell, rc::Rc};

pub mod environment;

#[derive(Debug, PartialEq)]
enum LuaVal {
    LuaTable(Table),
    LuaNil,
    LuaBool(bool),
    LuaNum([u8; 8]), // Represent numerals as an array of 8 bytes
    LuaString(String),
    Function(LuaFunction),
}

#[derive(Debug, PartialEq)]
pub struct LuaFunction {
    name: String,
    arity: usize, // The number of arguments
    statement: Vec<AST>,
}

// Wrapper around LuaVal to allow multiple owners

#[derive(Debug, PartialEq, Clone)]
pub struct LuaValue(Rc<RefCell<LuaVal>>);
impl LuaValue {
    pub fn new(val: LuaVal) -> Self {
        LuaValue(Rc::new(RefCell::new(val)))
    }

    pub fn get(&self) -> LuaVal {
        *self.0.borrow()
    }

    pub fn clone(&self) -> LuaValue {
        LuaValue(Rc::clone(&self.0))
    }
}

// TODO: Or use hashmap representation?
#[derive(Debug, PartialEq)]
struct Table(Vec<(LuaVal, LuaValue)>);

impl AST {
    pub fn exec(self, env: &mut Env) -> Result<(), ASTExecError> {
        self.0.exec(env)?;
        Ok(())
    }
}

impl Block {
    fn exec(self, env: &mut Env) -> Result<Vec<LuaValue>, ASTExecError> {
        // Extend environment when entering a new scope
        env.extend_env();

        // Execute each statement
        for statement in &self.statements {
            statement.exec(env)?;
        }

        // Optional return statement
        let explist = match self.return_stat {
            Some(explist) => explist,
            None => vec![],
        };

        let return_val = vec![LuaValue::new(LuaVal::LuaNil); explist.len()];
        for i in 0..explist.len() {
            return_val[i] = explist[i].eval(env)?;
        }

        // Remove environment when exiting a scope
        env.pop_env();

        Ok(return_val)
    }
}

impl Statement {
    fn exec(self, env: &mut Env) -> Result<(), ASTExecError> {
        match self {
            Statement::Assignment((varlist, explist)) => {
                assert!(varlist.len() == explist.len());
                for i in 0..varlist.len() {
                    let var = explist[i].eval(env)?;
                    match varlist[i] {
                        Var::NameVar(name) => {
                            env.insert(name, var);
                        }
                        // TODO: assignments for tables
                        Var::BracketVar((name, exp)) => {
                            unimplemented!()
                        }
                        Var::DotVar((name, field)) => {
                            unimplemented!()
                        }
                    }
                }
            }
            Statement::FunctionCall(funcall) => {
                unimplemented!()
            }
            Statement::Break => {
                unimplemented!()
            }
            Statement::DoBlock(block) => {
                unimplemented!()
            }
            Statement::While((exp, block)) => {
                unimplemented!()
            }
            Statement::Repeat((block, exp)) => {
                unimplemented!()
            }
            Statement::If((exp, block, elseifs, elseblock)) => {
                unimplemented!()
            }
            Statement::ForNum((name, exp1, exp2, exp3, block)) => {
                unimplemented!()
            }
            Statement::ForGeneric((names, explist, block)) => {
                unimplemented!()
            }
            Statement::FunctionDecl((name, parlist, block)) => {
                unimplemented!()
            }
            Statement::LocalFuncDecl((name, parlist, block)) => {
                unimplemented!()
            }
        };

        Ok(())
    }
}

impl Expression {
    fn eval(self, env: &mut Env) -> Result<LuaValue, ASTExecError> {
        let val = match self {
            Expression::Nil => LuaValue::new(LuaVal::LuaNil),
            Expression::False => LuaValue::new(LuaVal::LuaBool(false)),
            Expression::True => LuaValue::new(LuaVal::LuaBool(true)),
            Expression::Numeral(n) => match n {
                Numeral::Integer(i) => LuaValue::new(LuaVal::LuaNum(i.to_be_bytes())),
                Numeral::Float(f) => LuaValue::new(LuaVal::LuaNum(f.to_be_bytes())),
            },
            Expression::LiteralString(s) => LuaValue::new(LuaVal::LuaString(s)),
            Expression::DotDotDot => unimplemented!(),
            Expression::FunctionDef((parlist, block)) => unimplemented!(),
            Expression::PrefixExp(prefixexp) => unimplemented!(),
            Expression::TableConstructor(fields) => unimplemented!(),
            Expression::BinaryOp((left, op, right)) => unimplemented!(),
            Expression::UnaryOp((op, exp)) => unimplemented!(),
        };
        Ok(val)
    }
}

#[derive(Debug, PartialEq)]
pub struct ASTExecError(String);
impl Display for ASTExecError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_exp() {
        // Test Expression eval method
        let mut env = Env::new();

        // Nil
        let exp_nil = Expression::Nil;
        assert_eq!(exp_nil.eval(&mut env), Ok(LuaValue::new(LuaVal::LuaNil)));

        // Boolean
        let exp_false = Expression::False;
        let exp_true = Expression::True;
        assert_eq!(
            exp_false.eval(&mut env),
            Ok(LuaValue::new(LuaVal::LuaBool(false)))
        );
        assert_eq!(
            exp_true.eval(&mut env),
            Ok(LuaValue::new(LuaVal::LuaBool(true)))
        );

        // Integer
        let num: i64 = 10;
        let exp_int = Expression::Numeral(Numeral::Integer(num));
        assert_eq!(
            exp_int.eval(&mut env),
            Ok(LuaValue::new(LuaVal::LuaNum(num.to_be_bytes())))
        );

        // Float
        let num: f64 = 10.04;
        let exp_float = Expression::Numeral(Numeral::Float(num));
        assert_eq!(
            exp_float.eval(&mut env),
            Ok(LuaValue::new(LuaVal::LuaNum(num.to_be_bytes())))
        );

        // String
        let exp_str = Expression::LiteralString("Hello World!".to_string());
        assert_eq!(
            exp_str.eval(&mut env),
            Ok(LuaValue::new(LuaVal::LuaString("Hello World!".to_string())))
        );
    }

    #[test]
    fn test_exec_stat() {
        // Test Statement exec method
        let mut env = Env::new();

        // Assignment
        let a: i64 = 10;
        let b: i64 = 20;
        let varlist = vec![Var::NameVar("a".to_string()), Var::NameVar("b".to_string())];
        let explist = vec![
            Expression::Numeral(Numeral::Integer(10)),
            Expression::Numeral(Numeral::Integer(20)),
        ];
        let stat = Statement::Assignment((varlist, explist));
        assert_eq!(stat.exec(&mut env), Ok(()));
        assert_eq!(env.get("a").unwrap().get(), LuaVal::LuaNum(a.to_be_bytes()));
        assert_eq!(env.get("b").unwrap().get(), LuaVal::LuaNum(b.to_be_bytes()));
    }
}
