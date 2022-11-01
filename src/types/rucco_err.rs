use super::RuccoExp;

#[derive(thiserror::Error, Debug)]
pub enum RuccoReplErr {
    #[error("EmptyInput")]
    EmptyInput,
}

#[derive(thiserror::Error, Debug)]
pub enum RuccoReaderErr {
    #[error("UnexpectedEof")]
    UnexpectedEof,

    #[error("UnexpectedChar")]
    UnexpectedChar { char: char },
}

#[derive(Debug)]
pub enum RuccoDataType {
    // top level
    Atom,
    Cons,

    // atom
    Int,
    Float,
    Symbol,

    // cons
    List,
    DotList,

    // bool
    Nil,
    T,
}

#[derive(Debug)]
pub struct RuccoActualDataType {
    pub data_type: Vec<RuccoDataType>,
    pub value: String,
}

impl From<&RuccoExp> for RuccoActualDataType {
    fn from(exp: &RuccoExp) -> Self {
        match exp {
            RuccoExp::Atom(atom) => match atom {
                super::RuccoAtom::Int(e) => RuccoActualDataType {
                    data_type: vec![RuccoDataType::Atom, RuccoDataType::Int],
                    value: e.to_string(),
                },
                super::RuccoAtom::Float(e) => RuccoActualDataType {
                    data_type: vec![RuccoDataType::Atom, RuccoDataType::Float],
                    value: e.to_string(),
                },
                super::RuccoAtom::Symbol(e) => RuccoActualDataType {
                    data_type: vec![RuccoDataType::Atom, RuccoDataType::Symbol],
                    value: e.to_string(),
                },
            },
            RuccoExp::Cons { car: _, cdr: _ } => RuccoActualDataType {
                data_type: vec![RuccoDataType::Cons, RuccoDataType::List],
                value: exp.to_string(),
            },
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RuccoRuntimeErr {
    #[error("VoidVariable")]
    VoidVariable { name: String },

    #[error("VoidFunction")]
    VoidFunction { name: String },

    #[error("InvalidReference")]
    InvalidReference,

    #[error("WrongTypeArgument")]
    WrongTypeArgument {
        name: String,
        expected: RuccoDataType,
        actual: RuccoActualDataType,
    },

    #[error("WrongNumberOfArguments")]
    WrongNumberOfArguments {
        name: String,
        expected: usize,
        actual: usize,
    },
}
