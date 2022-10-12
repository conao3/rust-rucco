use super::RuccoAtom;

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq)]
pub enum RuccoExp {
    Atom(RuccoAtom),
    Cons{car: Rc<RefCell<RuccoExp>>, cdr: Rc<RefCell<RuccoExp>>},
}

impl std::fmt::Display for RuccoExp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RuccoExp::Atom(e) => write!(f, "{}", e),
            RuccoExp::Cons{car, cdr} => write!(f, "({} . {})", car.borrow(), cdr.borrow()),
        }
    }
}

impl std::convert::From<RuccoAtom> for RuccoExp {
    fn from(e: RuccoAtom) -> Self {
        RuccoExp::Atom(e)
    }
}

impl RuccoExp {
    pub fn new<T>(e: T) -> Self
    where
        T: Into<RuccoAtom>,
    {
        RuccoExp::Atom(e.into())
    }

    pub fn new_symbol<T>(e: T) -> Self
    where
        T: Into<String>,
    {
        RuccoExp::Atom(RuccoAtom::new_symbol(e))
    }
}

pub trait RuccoExpNewConsExt<S, T> {
    fn new_cons(car: S, cdr: T) -> RuccoExp;
}

impl<S, T> RuccoExpNewConsExt<S, T> for RuccoExp
where
    S: Into<Self>,
    T: Into<Self>,
{
    fn new_cons(car: S, cdr: T) -> Self {
        RuccoExp::Cons{car: Rc::new(RefCell::new(car.into())), cdr: Rc::new(RefCell::new(cdr.into()))}
    }
}

pub trait RuccoExpConsExt<T> {
    fn cons(self, cdr: T) -> RuccoExp;
}

impl<T> RuccoExpConsExt<T> for Rc<RefCell<RuccoExp>>
where
    T: Into<RuccoExp>,
{
    fn cons(self, cdr: T) -> RuccoExp {
        RuccoExp::Cons{car: Rc::new(RefCell::new(cdr.into())), cdr: self}
    }
}
