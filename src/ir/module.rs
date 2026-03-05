use super::function::Function;
use super::types::FunctionType;
use super::value::GlobalVariable;
use crate::ast;
use indexmap::IndexMap;

use super::types::StructType;

pub struct Registry {
    pub struct_types: IndexMap<String, StructType>,
    pub function_types: IndexMap<String, FunctionType>,
}

pub struct Module {
    pub global_list: IndexMap<String, GlobalVariable>,
    pub function_list: IndexMap<String, Function>,
}

pub struct IrGenerator<'a> {
    pub input: &'a ast::Program,
    pub module: Module,
    pub registry: Registry,
}

impl<'a> IrGenerator<'a> {
    pub(crate) const TARGET_TRIPLE: &'static str = "aarch64-unknown-linux-gnu";
    pub(crate) const TARGET_DATALAYOUT: &'static str =
        "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128";

    pub fn new(input: &'a ast::Program) -> Self {
        let module = Module {
            global_list: IndexMap::new(),
            function_list: IndexMap::new(),
        };
        let registry = Registry {
            struct_types: IndexMap::new(),
            function_types: IndexMap::new(),
        };
        Self {
            input,
            module,
            registry,
        }
    }
}
