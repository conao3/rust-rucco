use crate::core;
use crate::types;

pub fn compile(
    exp: &types::RuccoExpRef,
    arena: &mut types::RuccoArena,
) -> anyhow::Result<types::RuccoExpRef> {
    let stop = arena.alloc(types::RuccoExp::new_symbol("stop"));
    let stop_code = arena.alloc_list(vec![&stop]);
    let code = arena.alloc_list(vec![&stop_code]);
    comp(exp, arena, &mut std::collections::HashMap::new(), &code)
}

fn comp(
    exp: &types::RuccoExpRef,
    arena: &mut types::RuccoArena,
    _env: &mut core::RuccoEnv,
    code: &types::RuccoExpRef,
) -> anyhow::Result<types::RuccoExpRef> {
    let exp_ptr = exp
        .upgrade()
        .ok_or(types::RuccoRuntimeErr::InvalidReference)?;
    let ldc = arena.alloc(types::RuccoExp::new_symbol("ldc"));
    let x = match &*exp_ptr.borrow() {
        types::RuccoExp::Atom(ref atom) => match atom {
            types::RuccoAtom::Symbol(ref _sym) => {
                unimplemented!()
            }
            _atom => {
                let exp_code = arena.alloc_list(vec![&ldc, exp]);
                Ok(arena.alloc_dotlist(vec![&exp_code, code]))
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
                        let exp_code =
                            arena.alloc_list(vec![&ldc, cdr_ptr.borrow().car_weak_ref()?]);
                        Ok(arena.alloc_dotlist(vec![&exp_code, code]))
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
