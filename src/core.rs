use crate::types;
use crate::reader;

type RuccoEnv = std::collections::HashMap<String, String>;

pub fn read(buf: &str, arena: &mut types::RuccoArena) -> anyhow::Result<types::RuccoExpRef> {
    let mut reader = reader::Reader::new(buf, arena);
    reader.read()
}

pub fn eval(exp: types::RuccoExpRef, env: &mut RuccoEnv) -> anyhow::Result<types::RuccoExpRef> {
    Ok(exp)
}

pub fn print(buf: &str) -> String {
    buf.to_string()
}

pub fn rep(buf: &str, env: &mut RuccoEnv, arena: &mut types::RuccoArena) -> anyhow::Result<String> {
    Ok(print(eval(read(buf, arena)?, env)?.upgrade().unwrap().borrow().to_string().as_str()))
}
