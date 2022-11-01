#[derive(Debug, PartialEq)]
pub enum RuccoAtom {
    Int(i64),
    Float(f64),
    Symbol(String),
}

impl Eq for RuccoAtom {}

impl std::fmt::Display for RuccoAtom {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RuccoAtom::Int(e) => write!(f, "{}", e),
            RuccoAtom::Float(e) => write!(f, "{}", e),
            RuccoAtom::Symbol(e) => write!(f, "{}", e),
        }
    }
}

impl std::convert::From<i64> for RuccoAtom {
    fn from(e: i64) -> Self {
        RuccoAtom::Int(e)
    }
}

impl std::convert::From<f64> for RuccoAtom {
    fn from(e: f64) -> Self {
        RuccoAtom::Float(e)
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
