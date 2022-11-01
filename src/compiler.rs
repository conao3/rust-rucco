use crate::core;
use crate::types;

pub fn compile(
    exp: &types::RuccoExpRef,
    arena: &mut types::RuccoArena,
) -> anyhow::Result<types::RuccoExpRef> {
    let stop = arena.alloc(types::RuccoExp::new_symbol("stop"));
    let code = arena.alloc_list(vec![&stop]);
    comp(exp, arena, &mut std::collections::HashMap::new(), &code)
}

fn comp(
    exp: &types::RuccoExpRef,
    arena: &mut types::RuccoArena,
    env: &mut core::RuccoEnv,
    code: &types::RuccoExpRef,
) -> anyhow::Result<types::RuccoExpRef> {
    let exp_ptr = exp.upgrade().unwrap();
    let x = match *exp_ptr.borrow() {
        types::RuccoExp::Atom(ref atom) => match atom {
            types::RuccoAtom::Symbol(ref symbol) => {
                unreachable!()
            }
            ref _atom => {
                let ldc = arena.alloc(types::RuccoExp::new_symbol("ldc"));
                Ok(arena.alloc_dotlist(vec![&ldc, &exp, &code]))
            }
        },
        types::RuccoExp::Cons {
            car: ref _car,
            cdr: ref _cdr,
        } => {
            unreachable!("cons")
        }
    };
    x
}
