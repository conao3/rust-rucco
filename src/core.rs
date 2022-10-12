use std::rc::Rc;
use std::cell::RefCell;

use crate::types;
use crate::reader;

pub fn read(buf: &str) -> anyhow::Result<Rc<RefCell<types::RuccoExp>>> {
    let mut reader = reader::Reader::new(buf);
    reader.read()
}

pub fn eval(exp: Rc<RefCell<types::RuccoExp>>, env: &mut std::collections::HashMap<String, String>) -> anyhow::Result<Rc<RefCell<types::RuccoExp>>> {
    Ok(exp)
}

pub fn print(buf: &str) -> String {
    buf.to_string()
}

pub fn rep(buf: &str, env: &mut std::collections::HashMap<String, String>) -> anyhow::Result<String> {
    Ok(print(eval(read(buf)?, env)?.borrow().to_string().as_str()))
}
