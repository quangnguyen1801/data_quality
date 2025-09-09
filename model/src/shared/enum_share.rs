use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EnumOperator {
    Eq(String),
    Ne(String),
    Bt(String, String),
    Ge(String),
    Gt(String),
    Le(String),
    Lt(String),
    Cp(String),
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EnumGroup {
    Tt(String),
    Bt(String),
}
