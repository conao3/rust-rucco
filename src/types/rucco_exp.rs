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
            RuccoExp::Cons { .. } => {
                let mut lst: Vec<String> = Vec::new();

                for (car, cdr) in self.into_cons_iter().unwrap() {
                    lst.push(format!("{}", car.borrow()));
                    match &*cdr.borrow() {
                        RuccoExp::Atom(RuccoAtom::Symbol(s)) if s == "nil" => {}
                        RuccoExp::Atom(_) => {
                            lst.push(".".to_string());
                            lst.push(format!("{}", cdr.borrow()));
                        }
                        RuccoExp::Cons { .. } => (),
                    }
                }

                write!(f, "({})", lst.join(" "))
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

    pub fn car_weak(&self) -> anyhow::Result<RuccoExpRef> {
        match self {
            RuccoExp::Atom(_) => Err(anyhow::anyhow!(RuccoRuntimeErr::WrongTypeArgument {
                name: "car".to_string(),
                expected: RuccoDataType::Cons,
                actual: RuccoActualDataType::from(self)
            })),
            RuccoExp::Cons { car, .. } => Ok(car.clone()),
        }
    }

    pub fn car_weak_ref(&self) -> anyhow::Result<&RuccoExpRef> {
        match self {
            RuccoExp::Atom(_) => Err(anyhow::anyhow!(RuccoRuntimeErr::WrongTypeArgument {
                name: "car".to_string(),
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
                name: "cdr".to_string(),
                expected: RuccoDataType::Cons,
                actual: RuccoActualDataType::from(self)
            })),
            RuccoExp::Cons { cdr, .. } => Ok(cdr.clone()),
        }
    }

    pub fn cdr_weak_ref(&self) -> anyhow::Result<&RuccoExpRef> {
        match self {
            RuccoExp::Atom(_) => Err(anyhow::anyhow!(RuccoRuntimeErr::WrongTypeArgument {
                name: "cdr".to_string(),
                expected: RuccoDataType::Cons,
                actual: RuccoActualDataType::from(self)
            })),
            RuccoExp::Cons { cdr, .. } => Ok(cdr),
        }
    }

    pub fn into_iter(&self) -> anyhow::Result<RuccoExpIter> {
        match self {
            RuccoExp::Atom(_) => anyhow::bail!(RuccoRuntimeErr::WrongTypeArgument {
                name: "into_iter".to_string(),
                expected: RuccoDataType::Cons,
                actual: RuccoActualDataType::from(self)
            }),
            RuccoExp::Cons { car, cdr } => Ok(RuccoExpIter {
                car: Some(car.clone()),
                cdr: Some(cdr.clone()),
            }),
        }
    }

    pub fn into_cons_iter(&self) -> anyhow::Result<RuccoExpConsIter> {
        match self {
            RuccoExp::Atom(_) => anyhow::bail!(RuccoRuntimeErr::WrongTypeArgument {
                name: "into_cons_iter".to_string(),
                expected: RuccoDataType::Cons,
                actual: RuccoActualDataType::from(self)
            }),
            RuccoExp::Cons { car, cdr } => Ok(RuccoExpConsIter {
                car: Some(car.clone()),
                cdr: Some(cdr.clone()),
            }),
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
        let car_val = self.car.take()?;
        let cdr_val = self.cdr.take()?;

        Some(|| -> Self::Item {
            let car_ptr = car_val.upgrade().ok_or(RuccoRuntimeErr::InvalidReference)?;
            let cdr_ptr = cdr_val.upgrade().ok_or(RuccoRuntimeErr::InvalidReference)?;

            let cdr = cdr_ptr.borrow();
            match &*cdr {
                RuccoExp::Atom(RuccoAtom::Symbol(sym)) if sym == "nil" => {
                    self.car = None;
                    self.cdr = None;
                }
                RuccoExp::Atom(_) => {
                    anyhow::bail!(RuccoRuntimeErr::WrongTypeArgument {
                        name: "next".to_string(),
                        expected: RuccoDataType::List,
                        actual: RuccoActualDataType::from(&*cdr)
                    });
                }
                RuccoExp::Cons { car, cdr } => {
                    self.car = Some(car.clone());
                    self.cdr = Some(cdr.clone());
                }
            }

            Ok(car_ptr)
        }())
    }
}

pub struct RuccoExpConsIter {
    car: Option<RuccoExpRef>,
    cdr: Option<RuccoExpRef>,
}

impl Iterator for RuccoExpConsIter {
    type Item = (RuccoExpRefStrong, RuccoExpRefStrong);

    fn next(&mut self) -> Option<Self::Item> {
        let car_val = self.car.take()?;
        let cdr_val = self.cdr.take()?;

        Some(|| -> Self::Item {
            let car_ptr = car_val.upgrade().expect("Valid reference");
            let cdr_ptr = cdr_val.upgrade().expect("Valid reference");

            let cdr = cdr_ptr.clone();
            match &*cdr.borrow() {
                RuccoExp::Atom(_) => {
                    self.car = None;
                    self.cdr = None;
                }
                RuccoExp::Cons { car, cdr } => {
                    self.car = Some(car.clone());
                    self.cdr = Some(cdr.clone());
                }
            }

            (car_ptr, cdr_ptr)
        }())
    }
}

/// Setters
impl RuccoExp {
    /// Set car of the value
    ///
    /// # Examples
    /// ```
    /// use rucco::types::*;
    /// use std::rc::Rc;
    /// use std::cell::RefCell;
    ///
    /// let mut arena = RuccoArena::default();
    /// let nil = arena.nil();
    /// let c1 = arena.alloc(1.into());
    /// let c2 = arena.alloc(2.into());
    /// let c3 = arena.alloc(3.into());
    ///
    /// let e1 = arena.alloc((&c1, &nil).into());
    /// let e2 = arena.alloc((&c2, &e1).into());
    /// let e3 = arena.alloc((&c3, &e2).into());
    /// assert_eq!(e3.upgrade().unwrap().borrow().to_string(), "(3 2 1)");
    ///
    /// let v1 = arena.alloc(42.into());
    /// let e2_ptr = e2.upgrade().unwrap();
    /// e2_ptr.borrow_mut().setcar(&v1);
    /// assert_eq!(e3.upgrade().unwrap().borrow().to_string(), "(3 42 1)");
    /// ```
    pub fn setcar<'a>(&mut self, car: &'a RuccoExpRef) -> anyhow::Result<&'a RuccoExpRef> {
        match self {
            RuccoExp::Cons {
                car: ref mut cons_car,
                ..
            } => *cons_car = car.clone(),
            RuccoExp::Atom(_) => anyhow::bail!(RuccoRuntimeErr::WrongTypeArgument {
                name: "setcar".to_string(),
                expected: RuccoDataType::Cons,
                actual: RuccoActualDataType::from(&*self)
            }),
        }

        Ok(car)
    }

    /// Set cdr of the value
    ///
    /// # Examples
    /// ```
    /// use rucco::types::*;
    /// use std::rc::Rc;
    /// use std::cell::RefCell;
    ///
    /// let mut arena = RuccoArena::default();
    /// let nil = arena.nil();
    /// let c1 = arena.alloc(1.into());
    /// let c2 = arena.alloc(2.into());
    /// let c3 = arena.alloc(3.into());
    ///
    /// let e1 = arena.alloc((&c1, &nil).into());
    /// let e2 = arena.alloc((&c2, &e1).into());
    /// let e3 = arena.alloc((&c3, &e2).into());
    /// assert_eq!(e3.upgrade().unwrap().borrow().to_string(), "(3 2 1)");
    ///
    /// let v1 = arena.alloc(42.into());
    /// let e2_ptr = e2.upgrade().unwrap();
    /// e2_ptr.borrow_mut().setcdr(&v1);
    /// assert_eq!(e3.upgrade().unwrap().borrow().to_string(), "(3 2 . 42)");
    /// ```
    pub fn setcdr<'a>(&mut self, cdr: &'a RuccoExpRef) -> anyhow::Result<&'a RuccoExpRef> {
        match self {
            RuccoExp::Cons {
                cdr: ref mut cons_cdr,
                ..
            } => *cons_cdr = cdr.clone(),
            RuccoExp::Atom(_) => anyhow::bail!(RuccoRuntimeErr::WrongTypeArgument {
                name: "setcdr".to_string(),
                expected: RuccoDataType::Cons,
                actual: RuccoActualDataType::from(&*self)
            }),
        }

        Ok(cdr)
    }
}

#[cfg(test)]
mod tests {
    use super::super::RuccoArena;
    use super::*;

    #[test]
    fn test_cons() {
        let mut arena = RuccoArena::default();
        let nil = arena.nil();
        let c1 = arena.alloc(1.into());
        let c2 = arena.alloc(2.into());
        let c3 = arena.alloc(3.into());

        let e1 = arena.alloc((&c1, &nil).into());
        let e2 = arena.alloc((&c2, &e1).into());
        let e3 = arena.alloc((&c3, &e2).into());
        assert_eq!(e3.upgrade().unwrap().borrow().to_string(), "(3 2 1)");
    }

    #[test]
    fn test_cons_iter() {
        let mut arena = RuccoArena::default();
        let nil = arena.nil();
        let c1 = arena.alloc(1.into());
        let c2 = arena.alloc(2.into());
        let c3 = arena.alloc(3.into());

        let e1 = arena.alloc((&c1, &nil).into());
        let e2 = arena.alloc((&c2, &e1).into());
        let e3 = arena.alloc((&c3, &e2).into());

        let mut iter = e3.upgrade().unwrap().borrow().into_iter().unwrap();
        assert_eq!(*iter.next().unwrap().unwrap().borrow(), 3.into());
        assert_eq!(*iter.next().unwrap().unwrap().borrow(), 2.into());
        assert_eq!(*iter.next().unwrap().unwrap().borrow(), 1.into());
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_cons_refcell() {
        let mut arena = RuccoArena::default();

        // cons
        let nil = arena.nil();
        let v1 = arena.alloc(5.into());
        let v2 = arena.alloc(6.into());
        let v3 = arena.alloc(10.into());

        let a = arena.alloc((&v1, &nil).into());
        let b = arena.alloc((&v2, &a).into());
        let c = arena.alloc((&v3, &a).into());

        assert_eq!(a.upgrade().unwrap().borrow().to_string(), "(5)");
        assert_eq!(b.upgrade().unwrap().borrow().to_string(), "(6 5)");
        assert_eq!(c.upgrade().unwrap().borrow().to_string(), "(10 5)");

        // modify atom
        let v1_ptr = v1.upgrade().unwrap();
        *v1_ptr.borrow_mut() = 15.into();

        assert_eq!(a.upgrade().unwrap().borrow().to_string(), "(15)");
        assert_eq!(b.upgrade().unwrap().borrow().to_string(), "(6 15)");
        assert_eq!(c.upgrade().unwrap().borrow().to_string(), "(10 15)");

        // modify cons
        let w1 = arena.alloc(42.into());
        let w2 = arena.alloc(43.into());

        let a_ptr = a.upgrade().unwrap();
        *a_ptr.borrow_mut() = (&w1, &w2).into();

        assert_eq!(a.upgrade().unwrap().borrow().to_string(), "(42 . 43)");
        assert_eq!(b.upgrade().unwrap().borrow().to_string(), "(6 42 . 43)");
        assert_eq!(c.upgrade().unwrap().borrow().to_string(), "(10 42 . 43)");

        // modify car
        let x1 = arena.alloc(100.into());

        let a_ptr = a.upgrade().unwrap();
        match *a_ptr.borrow_mut() {
            RuccoExp::Cons { ref mut car, .. } => *car = x1,
            _ => panic!("not cons"),
        }

        assert_eq!(a.upgrade().unwrap().borrow().to_string(), "(100 . 43)");
        assert_eq!(b.upgrade().unwrap().borrow().to_string(), "(6 100 . 43)");
        assert_eq!(c.upgrade().unwrap().borrow().to_string(), "(10 100 . 43)");
    }
}
