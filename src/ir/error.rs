use crate::ast;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Initialization of structs not supported")]
    StructInitialization,

    #[error("Division by zero in constant expression")]
    DivisionByZero,

    #[error("Integer overflow in constant expression")]
    IntegerOverflow,

    #[error("Duplicated definition of variable {symbol}")]
    VariableRedefinition { symbol: String },

    #[error("Conflicted definition of function {symbol}")]
    ConflictedFunction { symbol: String },

    #[error("Symbol missing")]
    SymbolMissing,

    #[error("Mismatched declaration and definition of {symbol}")]
    DeclDefMismatch { symbol: String },

    #[error("Function {symbol} not defined")]
    FunctionNotDefined { symbol: String },

    #[error("Variable {symbol} not defined")]
    VariableNotDefined { symbol: String },

    #[error("Invalid array expression")]
    InvalidArrayExpression,

    #[error("Invalid struct member expression {expr}")]
    InvalidStructMemberExpression { expr: ast::MemberExpr },

    #[error("Invalid expression unit: {expr_unit}")]
    InvalidExprUnit { expr_unit: ast::ExprUnit },

    #[error("Unsupported local variable definition")]
    LocalVarDefinitionUnsupported,

    #[error("Unsupported function call")]
    FunctionCallUnsupported,

    #[error("Invalid continue instruction")]
    InvalidContinueInst,

    #[error("Invalid break instruction")]
    InvalidBreakInst,

    #[error("Unsupported return type")]
    ReturnTypeUnsupported,

    #[error("Struct type '{member_type}' used in struct '{struct_name}' is not defined")]
    UndefinedStructMemberType {
        struct_name: String,
        member_type: String,
    },

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}
