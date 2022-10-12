use std::cell::RefCell;
use std::rc::Rc;

use crate::types;

static INT_PATTERN: once_cell::sync::Lazy<regex::Regex> =
    once_cell::sync::Lazy::new(|| regex::Regex::new(r"^([+-]?[0-9]+)(?:[ ();]|$)").unwrap());
static FLOAT_PATTERN: once_cell::sync::Lazy<regex::Regex> = once_cell::sync::Lazy::new(|| {
    regex::Regex::new(r"^([+-]?[0-9]*\.[0-9]+)(?:[ ();]|$)").unwrap()
});
static SYMBOL_PATTERN: once_cell::sync::Lazy<regex::Regex> =
    once_cell::sync::Lazy::new(|| regex::Regex::new(r"^[^ ();]+").unwrap());

pub struct Reader<'a> {
    input: &'a str,
}

impl Reader<'_> {
    pub fn new(input: &str) -> Reader {
        Reader { input }
    }

    fn skip_whitespace(&mut self) {
        self.input = self.input.trim_start();
    }

    fn read_atom(&mut self) -> anyhow::Result<Rc<RefCell<types::RuccoExp>>> {
        self.skip_whitespace();

        if let Some(m) = INT_PATTERN.captures(self.input) {
            let s = m.get(1).unwrap().as_str();
            let i = s.parse::<i64>().unwrap();
            self.input = &self.input[s.len()..];

            return Ok(Rc::new(RefCell::new(types::RuccoExp::new(i))));
        }

        if let Some(m) = SYMBOL_PATTERN.captures(self.input) {
            let s = m.get(0).unwrap().as_str();
            self.input = &self.input[s.len()..];
            return Ok(Rc::new(RefCell::new(types::RuccoExp::new_symbol(s))));
        }

        unreachable!()
    }

    fn read_cons(&mut self) -> anyhow::Result<Rc<RefCell<types::RuccoExp>>> {
        self.skip_whitespace();

        anyhow::ensure!(!self.input.is_empty(), types::RuccoReaderErr::UnexpectedEof);

        if self.input.starts_with(')') {
            self.input = &self.input[1..]; // skip ')'
            return Ok(Rc::new(RefCell::new(types::RuccoExp::new_symbol("nil"))));
        }

        let car = self.read()?;

        self.skip_whitespace();
        if self.input.starts_with('.') {
            self.input = &self.input[1..]; // skip '.'

            self.skip_whitespace();

            if self.input.starts_with(')') {
                self.input = &self.input[1..]; // skip ')'
                anyhow::bail!(types::RuccoReaderErr::UnexpectedEof);
            }

            let cdr = self.read()?;

            self.skip_whitespace();
            anyhow::ensure!(self.input.starts_with(')'), types::RuccoReaderErr::UnexpectedEof);

            self.input = &self.input[1..]; // skip ')'

            return Ok(Rc::new(RefCell::new(types::RuccoExp::Cons{car, cdr})));
        }

        let cdr = self.read_cons()?;
        Ok(Rc::new(RefCell::new(types::RuccoExp::Cons{car, cdr})))
    }

    pub fn read(&mut self) -> anyhow::Result<Rc<RefCell<types::RuccoExp>>> {
        self.skip_whitespace();
        let c = self
            .input
            .chars()
            .next()
            .ok_or(types::RuccoReaderErr::UnexpectedEof)?;

        match c {
            '\'' => {
                self.input = &self.input[1..]; // skip '\''
                Ok(Rc::new(RefCell::new(types::RuccoExp::Cons {
                    car: Rc::new(RefCell::new(types::RuccoExp::new_symbol
                        ("quote")
                    )),
                    cdr: Rc::new(RefCell::new(types::RuccoExp::Cons {
                        car: self.read()?,
                        cdr: Rc::new(RefCell::new(types::RuccoExp::new_symbol("nil"))),
                    })),
                })))
            }
            '(' => {
                self.input = &self.input[1..]; // skip '('
                self.read_cons()
            }
            ')' => {
                self.input = &self.input[1..]; // skip ')'
                Err(anyhow::anyhow!(types::RuccoReaderErr::UnexpectedEof))
            }
            _ => self.read_atom(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use types::RuccoReaderErr;
    use types::RuccoExp as Exp;

    #[test]
    fn test_read_atom_0() {
        let input = "";
        let mut reader = Reader::new(input);
        let exp = reader.read().unwrap_err();
        assert_eq!(exp.to_string(), RuccoReaderErr::UnexpectedEof.to_string());
    }

    #[test]
    fn test_read_atom_1() {
        let input = "   ";
        let mut reader = Reader::new(input);
        let exp = reader.read().unwrap_err();
        assert_eq!(exp.to_string(), RuccoReaderErr::UnexpectedEof.to_string());
    }

    #[test]
    fn test_read_atom_2() {
        let input = "42";
        let mut reader = Reader::new(input);
        let exp = reader.read().unwrap();
        assert_eq!(exp, Rc::new(RefCell::new(Exp::new(42))));
    }

    // #[test]
    // fn test_read_atom_3() {
    //     let input = "42.3";
    //     let mut reader = Reader::new(input);
    //     let exp = reader.read().unwrap();
    //     assert_eq!(exp, Atom(Float(42.3)));
    // }

    // #[test]
    // fn test_read_atom_4() {
    //     let input = "   42.3";
    //     let mut reader = Reader::new(input);
    //     let exp = reader.read().unwrap();
    //     assert_eq!(exp, Atom(Float(42.3)));
    // }

    #[test]
    fn test_read_atom_5() {
        let input = "a";
        let mut reader = Reader::new(input);
        let exp = reader.read().unwrap();
        assert_eq!(exp, Rc::new(RefCell::new(Exp::new_symbol('a'))))
    }

    #[test]
    fn test_read_atom_6() {
        let input = "   a";
        let mut reader = Reader::new(input);
        let exp = reader.read().unwrap();
        assert_eq!(exp, Rc::new(RefCell::new(Exp::new_symbol('a'))))
    }

    #[test]
    fn test_read_atom_7() {
        let input = "1+";
        let mut reader = Reader::new(input);
        let exp = reader.read().unwrap();
        assert_eq!(exp, Rc::new(RefCell::new(Exp::new_symbol("1+"))))
    }

    #[test]
    fn test_read_cons_1() {
        let input = "()";
        let mut reader = Reader::new(input);
        let exp = reader.read().unwrap();
        assert_eq!(exp, Rc::new(RefCell::new(Exp::new_symbol("nil"))));

        let input = "(1 2 3)";
        let mut reader = Reader::new(input);
        let exp = reader.read().unwrap();
        assert_eq!(
            exp,
            Rc::new(RefCell::new(
            Exp::Cons {
                car: Rc::new(RefCell::new(Exp::new(1))),
                cdr: Rc::new(RefCell::new(Exp::Cons {
                    car: Rc::new(RefCell::new(Exp::new(2))),
                    cdr: Rc::new(RefCell::new(Exp::Cons {
                        car: Rc::new(RefCell::new(Exp::new(3))),
                        cdr: Rc::new(RefCell::new(types::RuccoExp::new_symbol("nil"))),
                    })),
                })),
            }
        )));
    }

    #[test]
    fn test_read_cons_2() {
        let input = "(1 2 . 3)";
        let mut reader = Reader::new(input);
        let exp = reader.read().unwrap();
        assert_eq!(
            exp,
            Rc::new(RefCell::new(
            Exp::Cons {
                car: Rc::new(RefCell::new(Exp::new(1))),
                cdr: Rc::new(RefCell::new(Exp::Cons {
                    car: Rc::new(RefCell::new(Exp::new(2))),
                    cdr: Rc::new(RefCell::new(Exp::new(3))),
                })),
            }))
        );

        let input = "(1 2 . 3";
        let mut reader = Reader::new(input);
        let exp = reader.read().unwrap_err();
        assert_eq!(exp.to_string(), RuccoReaderErr::UnexpectedEof.to_string());
    }
}
