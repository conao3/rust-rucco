use crate::compiler;
use crate::reader;
use crate::types;

pub type RuccoEnv = std::collections::HashMap<String, String>;

pub fn read(buf: &str, arena: &mut types::RuccoArena) -> anyhow::Result<types::RuccoExpRef> {
    let mut reader = reader::Reader::new(buf, arena);
    reader.read()
}

pub fn eval(
    exp: &types::RuccoExpRef,
    env: &mut RuccoEnv,
    arena: &mut types::RuccoArena,
) -> anyhow::Result<types::RuccoExpRef> {
    compiler::compile(exp, arena)
}

pub fn print(buf: &str) -> String {
    buf.to_string()
}

pub fn rep(buf: &str, env: &mut RuccoEnv, arena: &mut types::RuccoArena) -> anyhow::Result<String> {
    let exp = read(buf, arena)?;
    let exp = eval(&exp, env, arena)?;
    Ok(print(
        &exp.upgrade()
            .ok_or(types::RuccoRuntimeErr::InvalidReference)?
            .borrow()
            .to_string(),
    ))
}
