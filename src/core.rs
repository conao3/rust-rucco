use std::rc::Rc;
use std::cell::RefCell;

use crate::types;

pub fn read(buf: &str) -> anyhow::Result<Rc<RefCell<types::RuccoExp>>> {
    Ok(Rc::new(RefCell::new(types::RuccoExp::Atom(types::RuccoAtom::Int(1)))))
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
