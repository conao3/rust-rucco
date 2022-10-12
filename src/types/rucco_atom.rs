#[derive(Debug, PartialEq, Eq)]
pub enum RuccoAtom {
    Int(i64),
    Symbol(String),
}

impl std::fmt::Display for RuccoAtom {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RuccoAtom::Int(e) => write!(f, "{}", e),
            RuccoAtom::Symbol(e) => write!(f, "{}", e),
        }
    }
}

impl std::convert::From<i64> for RuccoAtom {
    fn from(e: i64) -> Self {
        RuccoAtom::Int(e)
    }
}

impl RuccoAtom {
    pub fn new_symbol<T>(e: T) -> Self
    where
        T: Into<String>,
    {
        RuccoAtom::Symbol(e.into())
    }
}
