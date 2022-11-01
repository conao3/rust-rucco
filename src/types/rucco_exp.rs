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

/// Accessors
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
}
