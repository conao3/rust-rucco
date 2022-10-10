pub enum RuccoAtom {
    Int(i64),
}

impl std::fmt::Display for RuccoAtom {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RuccoAtom::Int(e) => write!(f, "{}", e),
        }
    }
}
