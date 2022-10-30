use super::rucco_err::*;
use super::RuccoAtom;

use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

pub type RuccoExpRef = Weak<RefCell<RuccoExp>>;
pub type RuccoExpRefStrong = Rc<RefCell<RuccoExp>>;

#[derive(Debug)]
pub enum RuccoExp {
    Atom(RuccoAtom),
    Cons { car: RuccoExpRef, cdr: RuccoExpRef },
}

impl std::fmt::Display for RuccoExp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RuccoExp::Atom(e) => write!(f, "{}", e),
            RuccoExp::Cons { car, cdr } => {
                match || -> anyhow::Result<String> {
                    let car_rc = car.upgrade().ok_or(RuccoRuntimeErr::InvalidReference)?;
                    let cdr_rc = cdr.upgrade().ok_or(RuccoRuntimeErr::InvalidReference)?;

                    Ok(format!("({} . {})", car_rc.borrow(), cdr_rc.borrow()))
                }() {
                    Ok(e) => write!(f, "{}", e),
                    Err(e) => write!(f, "{}", e),
                }
            }
        }
    }
}

impl PartialEq for RuccoExp {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RuccoExp::Atom(e1), RuccoExp::Atom(e2)) => e1 == e2,
            (
                RuccoExp::Cons {
                    car: car1,
                    cdr: cdr1,
                },
                RuccoExp::Cons {
                    car: car2,
                    cdr: cdr2,
                },
            ) => match || -> anyhow::Result<bool> {
                let car1_rc = car1.upgrade().ok_or(RuccoRuntimeErr::InvalidReference)?;
                let car2_rc = car2.upgrade().ok_or(RuccoRuntimeErr::InvalidReference)?;
                let cdr1_rc = cdr1.upgrade().ok_or(RuccoRuntimeErr::InvalidReference)?;
                let cdr2_rc = cdr2.upgrade().ok_or(RuccoRuntimeErr::InvalidReference)?;

                let car1_ = car1_rc.borrow();
                let car2_ = car2_rc.borrow();
                let cdr1_ = cdr1_rc.borrow();
                let cdr2_ = cdr2_rc.borrow();

                Ok(*car1_ == *car2_ && *cdr1_ == *cdr2_)
            }() {
                Ok(e) => e,
                Err(e) => {
                    eprintln!("{}", e);
                    false
                }
            },
            _ => false,
        }
    }
}

impl<T> From<T> for RuccoExp
where
    T: Into<RuccoAtom>,
{
    fn from(t: T) -> Self {
        RuccoExp::Atom(t.into())
    }
}

impl std::convert::From<(&RuccoExpRef, &RuccoExpRef)> for RuccoExp {
    fn from((car, cdr): (&RuccoExpRef, &RuccoExpRef)) -> Self {
        RuccoExp::Cons {
            car: car.clone(),
            cdr: cdr.clone(),
        }
    }
}

/// Constructors
impl RuccoExp {
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
    pub fn get_symbol(&self) -> anyhow::Result<&String> {
        match self {
            RuccoExp::Atom(RuccoAtom::Symbol(s)) => Ok(s),
            _ => Err(anyhow::anyhow!(RuccoRuntimeErr::WrongTypeArgument {
                name: "get_symbol".to_string(),
                expected: RuccoDataType::Cons,
                actual: RuccoActualDataType::from(self)
            })),
        }
    }
}

impl RuccoExp {
    pub fn car(&self) -> anyhow::Result<RuccoExpRefStrong> {
        match self {
            RuccoExp::Atom(_) => Err(anyhow::anyhow!(RuccoRuntimeErr::WrongTypeArgument {
                name: "car".to_string(),
                expected: RuccoDataType::Cons,
                actual: RuccoActualDataType::from(self)
            })),
            RuccoExp::Cons { car, .. } => {
                Ok(car.upgrade().ok_or(RuccoRuntimeErr::InvalidReference)?)
            }
        }
    }

    pub fn car_weak(&self) -> anyhow::Result<RuccoExpRef> {
        match self {
            RuccoExp::Atom(_) => Err(anyhow::anyhow!(RuccoRuntimeErr::WrongTypeArgument {
                name: "car_weak".to_string(),
                expected: RuccoDataType::Cons,
                actual: RuccoActualDataType::from(self)
            })),
            RuccoExp::Cons { car, .. } => Ok(car.clone()),
        }
    }

    pub fn car_weak_ref(&self) -> anyhow::Result<&RuccoExpRef> {
        match self {
            RuccoExp::Atom(_) => Err(anyhow::anyhow!(RuccoRuntimeErr::WrongTypeArgument {
                name: "car_weak_ref".to_string(),
                expected: RuccoDataType::Cons,
                actual: RuccoActualDataType::from(self)
            })),
            RuccoExp::Cons { car, .. } => Ok(car),
        }
    }

    pub fn cdr(&self) -> anyhow::Result<RuccoExpRefStrong> {
        match self {
            RuccoExp::Atom(_) => Err(anyhow::anyhow!(RuccoRuntimeErr::WrongTypeArgument {
                name: "cdr".to_string(),
                expected: RuccoDataType::Cons,
                actual: RuccoActualDataType::from(self)
            })),
            RuccoExp::Cons { cdr, .. } => {
                Ok(cdr.upgrade().ok_or(RuccoRuntimeErr::InvalidReference)?)
            }
        }
    }

    pub fn cdr_weak(&self) -> anyhow::Result<RuccoExpRef> {
        match self {
            RuccoExp::Atom(_) => Err(anyhow::anyhow!(RuccoRuntimeErr::WrongTypeArgument {
                name: "cdr_weak".to_string(),
                expected: RuccoDataType::Cons,
                actual: RuccoActualDataType::from(self)
            })),
            RuccoExp::Cons { cdr, .. } => Ok(cdr.clone()),
        }
    }

    pub fn cdr_weak_ref(&self) -> anyhow::Result<&RuccoExpRef> {
        match self {
            RuccoExp::Atom(_) => Err(anyhow::anyhow!(RuccoRuntimeErr::WrongTypeArgument {
                name: "cdr_weak_ref".to_string(),
                expected: RuccoDataType::Cons,
                actual: RuccoActualDataType::from(self)
            })),
            RuccoExp::Cons { cdr, .. } => Ok(cdr),
        }
    }

    pub fn into_iter(&self) -> RuccoExpIter {
        RuccoExpIter {
            car: self.car_weak().ok(),
            cdr: self.cdr_weak().ok(),
        }
    }
}

pub struct RuccoExpIter {
    car: Option<RuccoExpRef>,
    cdr: Option<RuccoExpRef>,
}

impl Iterator for RuccoExpIter {
    type Item = anyhow::Result<RuccoExpRefStrong>;

    fn next(&mut self) -> Option<Self::Item> {
        self.car.as_ref()?;

        match || -> Self::Item {
            if let Some(car_val) = self.car.take() {
                if let Some(cdr_val) = self.cdr.take() {
                    let cdr_ptr = cdr_val.upgrade().ok_or(RuccoRuntimeErr::InvalidReference)?;
                    let cdr = cdr_ptr.borrow();

                    self.car = Some(cdr.car_weak()?);
                    self.cdr = Some(cdr.cdr_weak()?);
                }
                Ok(car_val.upgrade().ok_or(RuccoRuntimeErr::InvalidReference)?)
            } else {
                unreachable!("car should not None")
            }
        }() {
            Ok(v) => Some(Ok(v)),
            Err(e) => Some(Err(e)),
        }
    }
}
