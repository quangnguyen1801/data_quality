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

pub enum EnumGroup {
    Tt(String),
    Bt(String),
}
