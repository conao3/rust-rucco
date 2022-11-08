use super::rucco_exp::*;

use std::cell::RefCell;
use std::rc::Rc;

pub struct RuccoArena {
    arena: Vec<RuccoExpRefStrong>,
    symbols: std::collections::HashMap<String, RuccoExpRef>,
}

impl RuccoArena {
    pub fn alloc(&mut self, exp: RuccoExp) -> RuccoExpRef {
        let exp_ref = Rc::new(RefCell::new(exp));
        self.arena.push(exp_ref.clone());
        Rc::downgrade(&exp_ref)
    }

    pub fn alloc_symbol(&mut self, sym: &str) -> RuccoExpRef {
        if let Some(exp) = self.symbols.get(sym) {
            exp.clone()
        } else {
            let exp = self.alloc(RuccoExp::new_symbol(sym));
            self.symbols.insert(sym.to_string(), exp.clone());
            exp
        }
    }

    pub fn alloc_cons(&mut self, car: &RuccoExpRef, cdr: &RuccoExpRef) -> RuccoExpRef {
        self.alloc((car, cdr).into())
    }

    pub fn alloc_list(&mut self, exps: Vec<&RuccoExpRef>) -> RuccoExpRef {
        let mut lst = self.alloc_symbol("nil");
        for exp in exps.into_iter().rev() {
            lst = self.alloc_cons(exp, &lst);
        }
        lst
    }

    pub fn alloc_dotlist(&mut self, exps: Vec<&RuccoExpRef>) -> RuccoExpRef {
        let mut iter = exps.into_iter().rev();
        let last_cdr = iter.next().unwrap();
        let last_car = iter.next().unwrap();
        let mut lst = self.alloc_cons(last_car, last_cdr);
        for exp in iter {
            lst = self.alloc_cons(exp, &lst);
        }
        lst
    }

    pub fn cell(&mut self) -> RuccoExpRef {
        let nil = self.alloc_symbol("nil");
        self.alloc((&nil, &nil).into())
    }
}

macro_rules! alloc {
    ($arena: ident, [$exp: tt]) => {{
        let e = alloc!($arena, $exp);
        let nil = $arena.alloc_symbol("nil");
        $arena.alloc((e, nil).into())
    }};
    ($arena: ident, [$car: tt ; $cdr: tt]) => {{
        let car = alloc!($arena, $car);
        let cdr = alloc!($arena, $cdr);
        $arena.alloc((car, cdr).into())
    }};
    ($arena: ident, [$car: tt, $($rest: tt),* $( ; $last_cdr: tt )?]) => {{
        let car = alloc!($arena, $car);
        let cdr = alloc!($arena, [$($rest),* $( ; $last_cdr )?]);
        $arena.alloc((car, cdr).into())
    }};
    ($arena: ident, $exp: tt) => {
        $exp.clone()
    };
}

pub(crate) use alloc;

impl Default for RuccoArena {
    fn default() -> Self {
        Self {
            arena: Vec::with_capacity(10000),
            symbols: std::collections::HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alloc() {
        let mut arena = RuccoArena::default();
        let nil = arena.alloc_symbol("nil");
        let c1 = arena.alloc(1.into());
        let c2 = arena.alloc(2.into());
        let c3 = arena.alloc(3.into());

        let e1 = arena.alloc((&c1, &nil).into());
        let e2 = arena.alloc((&c2, &e1).into());
        let e3 = arena.alloc((&c3, &e2).into());
        assert_eq!(e3.upgrade().unwrap().borrow().to_string(), "(3 2 1)");
    }

    #[test]
    fn test_alloc_macro() {
        let mut arena = RuccoArena::default();
        let c1 = arena.alloc(1.into());
        let c2 = arena.alloc(2.into());
        let c3 = arena.alloc(3.into());

        let e1 = alloc!(arena, [c1]);
        assert_eq!(e1.upgrade().unwrap().borrow().to_string(), "(1)");

        let e2 = alloc!(arena, [c1, c2, c3]);
        assert_eq!(e2.upgrade().unwrap().borrow().to_string(), "(1 2 3)");

        let e3 = alloc!(arena, [[c1, c2], c3]);
        assert_eq!(e3.upgrade().unwrap().borrow().to_string(), "((1 2) 3)");

        let e4 = alloc!(arena, [c1, [c2, c3]]);
        assert_eq!(e4.upgrade().unwrap().borrow().to_string(), "(1 (2 3))");
    }

    #[test]
    fn test_alloc_macro_dotlist() {
        let mut arena = RuccoArena::default();
        let c1 = arena.alloc(1.into());
        let c2 = arena.alloc(2.into());
        let c3 = arena.alloc(3.into());

        let e1 = alloc!(arena, [c1]);
        assert_eq!(e1.upgrade().unwrap().borrow().to_string(), "(1)");

        let e2 = alloc!(arena, [c1; c2]);
        assert_eq!(e2.upgrade().unwrap().borrow().to_string(), "(1 . 2)");

        let e3 = alloc!(arena, [c1, c2, c3; e1]);
        assert_eq!(e3.upgrade().unwrap().borrow().to_string(), "(1 2 3 1)");

        let e4 = alloc!(arena, [c1, c2, c3; [c1, c2]]);
        assert_eq!(e4.upgrade().unwrap().borrow().to_string(), "(1 2 3 1 2)");

        let e5 = alloc!(arena, [c1, c2, c3; [c1, c2; c3]]);
        assert_eq!(
            e5.upgrade().unwrap().borrow().to_string(),
            "(1 2 3 1 2 . 3)"
        );
    }
}
