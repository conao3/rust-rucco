use std::rc::Rc;

use crate::core;
use crate::types;

pub fn compile(
    exp: &types::RuccoExpRef,
    arena: &mut types::RuccoArena,
) -> anyhow::Result<types::RuccoExpRef> {
    let stop = arena.alloc_symbol("stop");
    let code = types::alloc!(arena, [[stop]]);
    comp(exp, arena, &mut std::collections::HashMap::new(), &code)
}

fn comp(
    exp: &types::RuccoExpRef,
    arena: &mut types::RuccoArena,
    env: &mut core::RuccoEnv,
    code: &types::RuccoExpRef,
) -> anyhow::Result<types::RuccoExpRef> {
    let exp_ptr = exp
        .upgrade()
        .ok_or(types::RuccoRuntimeErr::InvalidReference)?;
    let ldc = arena.alloc_symbol("ldc");
    let sel = arena.alloc_symbol("sel");
    let join = arena.alloc_symbol("join");
    let x = match &*exp_ptr.borrow() {
        types::RuccoExp::Atom(ref atom) => match atom {
            types::RuccoAtom::Symbol(ref sym) if sym == "t" || sym == "nil" => {
                let exp_code = types::alloc!(arena, [ldc, exp]);
                Ok(types::alloc!(arena, [exp_code, code]))
            }
            types::RuccoAtom::Symbol(ref _sym) => {
                unimplemented!()
            }
            _atom => {
                let exp_code = types::alloc!(arena, [ldc, exp]);
                Ok(types::alloc!(arena, [exp_code; code]))
            }
        },
        types::RuccoExp::Cons { ref car, ref cdr } => {
            let car_ptr = car
                .upgrade()
                .ok_or(types::RuccoRuntimeErr::InvalidReference)?;
            let cdr_ptr = cdr
                .upgrade()
                .ok_or(types::RuccoRuntimeErr::InvalidReference)?;
            let x = match &*car_ptr.borrow() {
                types::RuccoExp::Atom(ref atom) => match atom {
                    types::RuccoAtom::Symbol(ref sym) if sym == "quote" => {
                        let cdr_code_ = cdr_ptr.borrow();
                        let cdr_code = cdr_code_.car_weak_ref()?;
                        Ok(types::alloc!(arena, [[ldc, cdr_code]; code]))
                    }
                    types::RuccoAtom::Symbol(ref sym) if sym == "if" => {
                        let nil = types::alloc!(arena, []);
                        let mut args = cdr_ptr.borrow().extract_args("compile", (2, 3), &nil)?;
                        let else_ptr = args.pop().ok_or(types::RuccoRuntimeErr::Unreachable)?;
                        let then_ptr = args.pop().ok_or(types::RuccoRuntimeErr::Unreachable)?;
                        let test_ptr = args.pop().ok_or(types::RuccoRuntimeErr::Unreachable)?;

                        let join_code = types::alloc!(arena, [[join]]);

                        let test_code = comp(&Rc::downgrade(&test_ptr), arena, env, &nil)?;
                        let then_code = comp(&Rc::downgrade(&then_ptr), arena, env, &join_code)?;
                        let else_code = comp(&Rc::downgrade(&else_ptr), arena, env, &join_code)?;

                        let test_ptr = test_code
                            .upgrade()
                            .ok_or(types::RuccoRuntimeErr::InvalidReference)?;
                        let test_car_code = &test_ptr.borrow().car_weak()?;
                        Ok(types::alloc!(arena, [test_car_code, [sel, then_code, else_code]; code]))
                    }
                    types::RuccoAtom::Symbol(ref _sym) => unimplemented!(),
                    _ => unimplemented!(),
                },
                types::RuccoExp::Cons { .. } => unimplemented!(),
            };
            x
        }
    };
    x
}
