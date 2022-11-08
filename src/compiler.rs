use std::rc::Rc;

use crate::core;
use crate::types;

pub fn compile(
    exp: &types::RuccoExpRef,
    arena: &mut types::RuccoArena,
) -> anyhow::Result<types::RuccoExpRef> {
    let stop = arena.alloc_symbol("stop");
    let stop_code = types::alloc!(arena, [stop]);
    let code = types::alloc!(arena, [stop_code]);
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
                        let exp_code = types::alloc!(arena, [ldc, cdr_code]);
                        Ok(types::alloc!(arena, [exp_code; code]))
                    }
                    types::RuccoAtom::Symbol(ref sym) if sym == "if" => {
                        let nil = arena.alloc_symbol("nil");
                        let args = cdr_ptr.borrow().extract_args("compile", (2, 3), &nil)?;
                        let test_ptr = &args[0];
                        let then_ptr = &args[1];
                        let else_ptr = &args[2];

                        let join_exp = types::alloc!(arena, [join]);
                        let join_code = types::alloc!(arena, [join_exp]);

                        let test_code = comp(&Rc::downgrade(test_ptr), arena, env, &nil)?;
                        let then_code = comp(&Rc::downgrade(then_ptr), arena, env, &join_code)?;
                        let else_code = comp(&Rc::downgrade(else_ptr), arena, env, &join_code)?;

                        let test_ptr = test_code
                            .upgrade()
                            .ok_or(types::RuccoRuntimeErr::InvalidReference)?;
                        let test_car_code = &test_ptr.borrow().car_weak()?;
                        let sel_body = types::alloc!(arena, [sel, then_code, else_code]);
                        Ok(types::alloc!(arena, [test_car_code, sel_body; code]))
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
