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
}
