use super::rucco_exp::*;

use std::cell::RefCell;
use std::rc::Rc;

pub struct RuccoArena {
    arena: Vec<RuccoExpRefStrong>,
    nil: RuccoExpRef,
    t: RuccoExpRef,
}

impl RuccoArena {
    pub fn alloc(&mut self, exp: RuccoExp) -> RuccoExpRef {
        let exp_ref = Rc::new(RefCell::new(exp));
        self.arena.push(exp_ref.clone());
        Rc::downgrade(&exp_ref)
    }

    pub fn nil(&self) -> RuccoExpRef {
        self.nil.clone()
    }

    pub fn t(&self) -> RuccoExpRef {
        self.t.clone()
    }
}

impl Default for RuccoArena {
    fn default() -> Self {
        let mut arena: Vec<RuccoExpRefStrong> = Vec::with_capacity(10000);
        arena.push(Rc::new(RefCell::new(RuccoExp::new_symbol("nil"))));
        arena.push(Rc::new(RefCell::new(RuccoExp::new_symbol("t"))));

        let nil = Rc::downgrade(&arena[0]);
        let t = Rc::downgrade(&arena[1]);
        Self { arena, nil, t }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alloc() {
        let mut arena = RuccoArena::default();
        let nil = arena.nil();
        let c1 = arena.alloc(1.into());
        let c2 = arena.alloc(2.into());
        let c3 = arena.alloc(3.into());

        let e1 = arena.alloc((&c1, &nil).into());
        let e2 = arena.alloc((&c2, &e1).into());
        let e3 = arena.alloc((&c3, &e2).into());
        assert_eq!(
            e3.upgrade().unwrap().borrow().to_string(),
            "(3 . (2 . (1 . nil)))"
        );
    }
}
