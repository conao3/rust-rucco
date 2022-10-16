use super::rucco_err::*;
use super::RuccoAtom;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum RuccoExp {
    Atom(RuccoAtom),
    Cons {
        car: Rc<RefCell<RuccoExp>>,
        cdr: Rc<RefCell<RuccoExp>>,
    },
}

impl std::fmt::Display for RuccoExp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RuccoExp::Atom(e) => write!(f, "{}", e),
            RuccoExp::Cons { car, cdr } => write!(f, "({} . {})", car.borrow(), cdr.borrow()),
        }
    }
}

impl std::convert::From<RuccoAtom> for RuccoExp {
    fn from(e: RuccoAtom) -> Self {
        RuccoExp::Atom(e)
    }
}

/// Constructors
impl RuccoExp {
    /// Create a RuccoExp::Atom::Any
    ///
    /// # Examples
    ///
    /// ```
    /// use rucco::types::*;
    ///
    /// let e = RuccoExp::atom(1);
    ///
    /// assert_eq!(e, RuccoExp::Atom(RuccoAtom::Int(1)));
    /// ```
    pub fn atom<T>(e: T) -> Self
    where
        T: Into<RuccoAtom>,
    {
        RuccoExp::Atom(e.into())
    }

    /// Create a RuccoExp::Atom::Symbol
    ///
    /// # Examples
    ///
    /// ```
    /// use rucco::types::*;
    ///
    /// let e = RuccoExp::new_symbol("a");
    ///
    /// assert_eq!(e, RuccoExp::Atom(RuccoAtom::Symbol("a".to_string())));
    /// ```
    pub fn new_symbol<T>(e: T) -> Self
    where
        T: Into<String>,
    {
        RuccoExp::Atom(RuccoAtom::new_symbol(e))
    }

    /// Create a RuccoExp::Atom::Symbol("nil")
    ///
    /// # Examples
    ///
    /// ```
    /// use rucco::types::*;
    ///
    /// let e = RuccoExp::nil();
    ///
    /// assert_eq!(e, RuccoExp::Atom(RuccoAtom::Symbol("nil".to_string())));
    /// ```
    pub fn nil() -> Self {
        RuccoExp::new_symbol("nil")
    }

    /// Create a RuccoExp::Atom::Symbol("t")
    ///
    /// # Examples
    ///
    /// ```
    /// use rucco::types::*;
    ///
    /// let e = RuccoExp::t();
    ///
    /// assert_eq!(e, RuccoExp::Atom(RuccoAtom::Symbol("t".to_string())));
    /// ```
    pub fn t() -> Self {
        RuccoExp::new_symbol("t")
    }

    /// Create a RuccoExp::Cons
    ///
    /// # Arguments
    ///
    /// * `car` - Value of car
    /// * `cdr` - Value of cdr
    ///
    /// # Examples
    ///
    /// ```
    /// use rucco::types::*;
    /// use std::rc::Rc;
    /// use std::cell::RefCell;
    ///
    /// let e = Rc::new(RefCell::new(RuccoExp::cons(
    ///     &Rc::new(RefCell::new(RuccoExp::atom(1))),
    ///     &Rc::new(RefCell::new(RuccoExp::atom(2))),
    /// )));
    ///
    /// assert_eq!(*e.borrow(), RuccoExp::Cons {
    ///     car: Rc::new(RefCell::new(RuccoExp::atom(1))),
    ///     cdr: Rc::new(RefCell::new(RuccoExp::atom(2))),
    /// });
    /// ```
    ///
    /// ```
    /// use rucco::types::*;
    /// use std::rc::Rc;
    /// use std::cell::RefCell;
    ///
    /// let e1 = Rc::new(RefCell::new(RuccoExp::cons(
    ///     &Rc::new(RefCell::new(RuccoExp::atom(1))),
    ///     &Rc::new(RefCell::new(RuccoExp::nil()))
    /// )));
    /// let e2 = Rc::new(RefCell::new(RuccoExp::cons(&Rc::new(RefCell::new(RuccoExp::atom(2))), &e1)));
    /// let e3 = Rc::new(RefCell::new(RuccoExp::cons(&Rc::new(RefCell::new(RuccoExp::atom(3))), &e2)));
    ///
    /// assert_eq!(*e3.borrow(), RuccoExp::Cons {
    ///     car: Rc::new(RefCell::new(RuccoExp::atom(3))),
    ///     cdr: Rc::new(RefCell::new(RuccoExp::Cons {
    ///         car: Rc::new(RefCell::new(RuccoExp::atom(2))),
    ///         cdr: Rc::new(RefCell::new(RuccoExp::Cons {
    ///             car: Rc::new(RefCell::new(RuccoExp::atom(1))),
    ///             cdr: Rc::new(RefCell::new(RuccoExp::nil()))
    ///         }))
    ///     }))
    /// });
    /// ```
    pub fn cons(car: &Rc<RefCell<RuccoExp>>, cdr: &Rc<RefCell<RuccoExp>>) -> Self {
        RuccoExp::Cons {
            car: car.clone(),
            cdr: cdr.clone(),
        }
    }
}

/// Predicates
impl RuccoExp {
    /// Predicate for RuccoExp::Atom
    ///
    /// # Examples
    ///
    /// ```
    /// use rucco::types::*;
    /// use std::rc::Rc;
    /// use std::cell::RefCell;
    ///
    /// let e = RuccoExp::atom(1);
    /// assert_eq!(e.atomp(), true);
    /// ```
    pub fn atomp(&self) -> bool {
        matches!(self, RuccoExp::Atom(_))
    }

    /// Predicate for RuccoExp::Cons
    ///
    /// # Examples
    ///
    /// ```
    /// use rucco::types::*;
    /// use std::rc::Rc;
    /// use std::cell::RefCell;
    ///
    /// let e = RuccoExp::cons(
    ///     &Rc::new(RefCell::new(RuccoExp::atom(1))),
    ///     &Rc::new(RefCell::new(RuccoExp::atom(2)))
    /// );
    /// assert_eq!(e.consp(), true);
    /// ```
    pub fn consp(&self) -> bool {
        matches!(self, RuccoExp::Cons { .. })
    }

    /// Predicate for RuccoExp::Atom::Int
    ///
    /// # Examples
    ///
    /// ```
    /// use rucco::types::*;
    /// use std::rc::Rc;
    /// use std::cell::RefCell;
    ///
    /// let e = RuccoExp::atom(1);
    /// assert_eq!(e.intp(), true);
    /// ```
    pub fn intp(&self) -> bool {
        matches!(self, RuccoExp::Atom(RuccoAtom::Int(_)))
    }

    /// Predicate for RuccoExp::Atom::Symbol
    ///
    /// # Examples
    ///
    /// ```
    /// use rucco::types::*;
    /// use std::rc::Rc;
    /// use std::cell::RefCell;
    ///
    /// let e = RuccoExp::new_symbol("a");
    /// assert_eq!(e.symbolp(), true);
    /// ```
    pub fn symbolp(&self) -> bool {
        matches!(self, RuccoExp::Atom(RuccoAtom::Symbol(_)))
    }
}

/// Accessors
impl RuccoExp {
    /// Get int of the value
    ///
    /// # Examples
    ///
    /// ```
    /// use rucco::types::*;
    /// use std::rc::Rc;
    /// use std::cell::RefCell;
    ///
    /// let e = RuccoExp::atom(1);
    /// assert_eq!(*e.get_int().unwrap(), 1);
    /// ```
    pub fn get_int(&self) -> anyhow::Result<&i64> {
        match self {
            RuccoExp::Atom(RuccoAtom::Int(i)) => Ok(i),
            _ => Err(anyhow::anyhow!(RuccoRuntimeErr::WrongTypeArgument {
                name: "get_int".to_string(),
                expected: RuccoDataType::Cons,
                actual: RuccoActualDataType::from(self)
            })),
        }
    }

    /// Get symbol of the value
    ///
    /// # Examples
    ///
    /// ```
    /// use rucco::types::*;
    /// use std::rc::Rc;
    /// use std::cell::RefCell;
    ///
    /// let e = RuccoExp::new_symbol("a");
    /// assert_eq!(*e.get_symbol().unwrap(), "a".to_string());
    /// ```
    pub fn get_symbol(&self) -> Option<&String> {
        match self {
            RuccoExp::Atom(RuccoAtom::Symbol(s)) => Some(s),
            _ => None,
        }
    }

    /// Get value of car
    ///
    /// # Examples
    ///
    /// ```
    /// use rucco::types::*;
    /// use std::rc::Rc;
    /// use std::cell::RefCell;
    ///
    /// let e = Rc::new(RefCell::new(RuccoExp::cons(
    ///     &Rc::new(RefCell::new(RuccoExp::atom(1))),
    ///     &Rc::new(RefCell::new(RuccoExp::atom(2))),
    /// )));
    /// assert_eq!(*e.borrow().car().unwrap(), Rc::new(RefCell::new(RuccoExp::atom(1))));
    /// ```
    ///
    /// ```
    /// use rucco::types::*;
    /// use std::rc::Rc;
    /// use std::cell::RefCell;
    ///
    /// let e1 = Rc::new(RefCell::new(RuccoExp::cons(
    ///     &Rc::new(RefCell::new(RuccoExp::atom(1))),
    ///     &Rc::new(RefCell::new(RuccoExp::nil()))
    /// )));
    /// let e2 = Rc::new(RefCell::new(RuccoExp::cons(&Rc::new(RefCell::new(RuccoExp::atom(2))), &e1)));
    /// let e3 = Rc::new(RefCell::new(RuccoExp::cons(&Rc::new(RefCell::new(RuccoExp::atom(3))), &e2)));
    ///
    /// let c1_binding = e3.borrow();
    /// let c1 = c1_binding.car().unwrap();
    ///
    /// let c2_binding_1 = e3.borrow();
    /// let c2_binding_2 = c2_binding_1.cdr().unwrap().borrow();
    /// let c2 = c2_binding_2.car().unwrap();
    ///
    /// let c3_binding_1 = e3.borrow();
    /// let c3_binding_2 = c3_binding_1.cdr().unwrap().borrow();
    /// let c3_binding_3 = c3_binding_2.cdr().unwrap().borrow();
    /// let c3 = c3_binding_3.car().unwrap();
    ///
    /// assert_eq!(*c1, Rc::new(RefCell::new(RuccoExp::atom(3))));
    /// assert_eq!(*c2, Rc::new(RefCell::new(RuccoExp::atom(2))));
    /// assert_eq!(*c3, Rc::new(RefCell::new(RuccoExp::atom(1))));
    /// ```
    pub fn car(&self) -> anyhow::Result<&Rc<RefCell<RuccoExp>>> {
        match self {
            RuccoExp::Cons { car, .. } => Ok(car),
            _ => Err(anyhow::anyhow!(RuccoRuntimeErr::WrongTypeArgument {
                name: "car".to_string(),
                expected: RuccoDataType::Cons,
                actual: RuccoActualDataType::from(self),
            })),
        }
    }

    /// Get value of cdr
    ///
    /// # Examples
    ///
    /// ```
    /// use rucco::types::*;
    /// use std::rc::Rc;
    /// use std::cell::RefCell;
    ///
    /// let e = Rc::new(RefCell::new(RuccoExp::cons(
    ///     &Rc::new(RefCell::new(RuccoExp::atom(1))),
    ///     &Rc::new(RefCell::new(RuccoExp::atom(2))),
    /// )));
    /// assert_eq!(*e.borrow().cdr().unwrap(), Rc::new(RefCell::new(RuccoExp::atom(2))));
    /// ```
    ///
    /// ```
    /// use rucco::types::*;
    /// use std::rc::Rc;
    /// use std::cell::RefCell;
    ///
    /// let e1 = Rc::new(RefCell::new(RuccoExp::cons(
    ///     &Rc::new(RefCell::new(RuccoExp::atom(1))),
    ///     &Rc::new(RefCell::new(RuccoExp::nil()))
    /// )));
    /// let e2 = Rc::new(RefCell::new(RuccoExp::cons(&Rc::new(RefCell::new(RuccoExp::atom(2))), &e1)));
    /// let e3 = Rc::new(RefCell::new(RuccoExp::cons(&Rc::new(RefCell::new(RuccoExp::atom(3))), &e2)));
    ///
    /// let c1_binding = e3.borrow();
    /// let c1 = c1_binding.car().unwrap();
    ///
    /// let c2_binding_1 = e3.borrow();
    /// let c2_binding_2 = c2_binding_1.cdr().unwrap().borrow();
    /// let c2 = c2_binding_2.car().unwrap();
    ///
    /// let c3_binding_1 = e3.borrow();
    /// let c3_binding_2 = c3_binding_1.cdr().unwrap().borrow();
    /// let c3_binding_3 = c3_binding_2.cdr().unwrap().borrow();
    /// let c3 = c3_binding_3.car().unwrap();
    ///
    /// assert_eq!(*c1, Rc::new(RefCell::new(RuccoExp::atom(3))));
    /// assert_eq!(*c2, Rc::new(RefCell::new(RuccoExp::atom(2))));
    /// assert_eq!(*c3, Rc::new(RefCell::new(RuccoExp::atom(1))));
    /// ```
    pub fn cdr(&self) -> anyhow::Result<&Rc<RefCell<RuccoExp>>> {
        match self {
            RuccoExp::Cons { cdr, .. } => Ok(cdr),
            _ => Err(anyhow::anyhow!(RuccoRuntimeErr::WrongTypeArgument {
                name: "cdr".to_string(),
                expected: RuccoDataType::Cons,
                actual: RuccoActualDataType::from(self),
            })),
        }
    }
}

pub trait RuccoExpNewConsExt<S, T> {
    fn cons(car: S, cdr: T) -> RuccoExp;
}

impl<S, T> RuccoExpNewConsExt<S, T> for RuccoExp
where
    S: Into<Self>,
    T: Into<Self>,
{
    fn cons(car: S, cdr: T) -> Self {
        RuccoExp::Cons {
            car: Rc::new(RefCell::new(car.into())),
            cdr: Rc::new(RefCell::new(cdr.into())),
        }
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
        RuccoExp::Cons {
            car: Rc::new(RefCell::new(cdr.into())),
            cdr: self,
        }
    }
}
