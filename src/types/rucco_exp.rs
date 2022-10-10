use super::RuccoAtom;

use std::rc::Rc;
use std::cell::RefCell;

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
