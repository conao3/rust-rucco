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
}

macro_rules! alloc {
    ($arena: expr, [$exp: tt]) => {{
        let e = crate::types::alloc!($arena, $exp);
        let nil = $arena.alloc_symbol("nil");
        $arena.alloc((e, nil).into())
    }};
    ($arena: expr, [$car: tt ; $cdr: tt]) => {{
        let car = crate::types::alloc!($arena, $car);
        let cdr = crate::types::alloc!($arena, $cdr);
        $arena.alloc((car, cdr).into())
    }};
    ($arena: expr, [$car: tt, $($rest: tt),* $( ; $last_cdr: tt )?]) => {{
        let car = crate::types::alloc!($arena, $car);
        let cdr = crate::types::alloc!($arena, [$($rest),* $( ; $last_cdr )?]);
        $arena.alloc((car, cdr).into())
    }};
    ($arena: expr, $exp: tt) => {
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
