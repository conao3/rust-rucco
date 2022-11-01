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
    arena: &'a mut types::RuccoArena,
}

impl Reader<'_> {
    pub fn new<'a>(input: &'a str, arena: &'a mut types::RuccoArena) -> Reader {
        Reader { input, arena }
    }

    fn skip_whitespace(&mut self) {
        self.input = self.input.trim_start();
    }

    fn read_atom(&mut self) -> anyhow::Result<types::RuccoExpRef> {
        self.skip_whitespace();

        if let Some(m) = INT_PATTERN.captures(self.input) {
            let s = m.get(1).unwrap().as_str();
            let i = s.parse::<i64>().unwrap();
            self.input = &self.input[s.len()..];

            return Ok(self.arena.alloc(i.into()));
        }

        if let Some(m) = SYMBOL_PATTERN.captures(self.input) {
            let s = m.get(0).unwrap().as_str();
            self.input = &self.input[s.len()..];
            return Ok(self.arena.alloc(types::RuccoExp::new_symbol(s)));
        }

        unreachable!()
    }

    fn read_cons(&mut self) -> anyhow::Result<types::RuccoExpRef> {
        self.input = &self.input[1..]; // skip '('

        self.skip_whitespace();
        match self.input.chars().next() {
            None => anyhow::bail!(types::RuccoReaderErr::UnexpectedEof),
            Some(')') => {
                self.input = &self.input[1..]; // skip ')'
                Ok(self.arena.alloc(types::RuccoExp::nil()))
            }
            Some(_) => {
                let nil = self.arena.alloc(types::RuccoExp::nil());
                let res = self.arena.alloc((&nil, &nil).into());
                let mut ptr = res
                    .upgrade()
                    .ok_or(types::RuccoRuntimeErr::InvalidReference)?;
                loop {
                    let car = self.read()?;

                    self.skip_whitespace();
                    match self.input.chars().next() {
                        None => anyhow::bail!(types::RuccoReaderErr::UnexpectedEof),
                        Some(')') => {
                            self.input = &self.input[1..]; // skip ')'
                            ptr.borrow_mut().setcar(&car)?;
                            return Ok(res);
                        }
                        Some('.') => {
                            self.input = &self.input[1..]; // skip '.'
                            let cdr = self.read()?;

                            self.skip_whitespace();
                            match self.input.chars().next() {
                                None => anyhow::bail!(types::RuccoReaderErr::UnexpectedEof),
                                Some(')') => {
                                    self.input = &self.input[1..]; // skip ')'
                                    ptr.borrow_mut().setcar(&car)?;
                                    ptr.borrow_mut().setcdr(&cdr)?;
                                    return Ok(res);
                                }
                                Some(char) => {
                                    anyhow::bail!(types::RuccoReaderErr::UnexpectedChar { char })
                                }
                            }
                        }
                        Some(_) => {
                            let cell = self.arena.alloc((&nil, &nil).into());
                            ptr.borrow_mut().setcar(&car)?;
                            ptr.borrow_mut().setcdr(&cell)?;
                            ptr = ptr.clone().borrow().cdr()?;
                        },
                    }
                }
            }
        }
    }

    pub fn read(&mut self) -> anyhow::Result<types::RuccoExpRef> {
        self.skip_whitespace();
        let c = self
            .input
            .chars()
            .next()
            .ok_or(types::RuccoReaderErr::UnexpectedEof)?;

        match c {
            // '\'' => {
            //     self.input = &self.input[1..]; // skip '\''
            //     Ok(Rc::new(RefCell::new(types::RuccoExp::Cons {
            //         car: Rc::new(RefCell::new(types::RuccoExp::new_symbol
            //             ("quote")
            //         )),
            //         cdr: Rc::new(RefCell::new(types::RuccoExp::Cons {
            //             car: self.read()?,
            //             cdr: Rc::new(RefCell::new(types::RuccoExp::new_symbol("nil"))),
            //         })),
            //     })))
            // }
            '(' => self.read_cons(),
            ')' => Err(anyhow::anyhow!(types::RuccoReaderErr::UnexpectedEof)),
            _ => self.read_atom(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use types::RuccoExp;
    use types::RuccoReaderErr;

    #[test]
    fn test_read_atom_0() {
        let input = "";
        let arena = &mut types::RuccoArena::default();
        let mut reader = Reader::new(input, arena);
        let exp = reader.read().unwrap_err();
        assert_eq!(exp.to_string(), RuccoReaderErr::UnexpectedEof.to_string());
    }

    #[test]
    fn test_read_atom_1() {
        let input = "    ";
        let arena = &mut types::RuccoArena::default();
        let mut reader = Reader::new(input, arena);
        let exp = reader.read().unwrap_err();
        assert_eq!(exp.to_string(), RuccoReaderErr::UnexpectedEof.to_string());
    }

    #[test]
    fn test_read_atom_2() {
        let input = "42";
        let arena = &mut types::RuccoArena::default();
        let mut reader = Reader::new(input, arena);
        let exp_ = reader.read().unwrap();
        let exp_ptr = exp_.upgrade().unwrap();
        assert_eq!(*exp_ptr.borrow(), 42.into());
    }

    // // #[test]
    // // fn test_read_atom_3() {
    // //     let input = "42.3";
    // //     let mut reader = Reader::new(input);
    // //     let exp = reader.read().unwrap();
    // //     assert_eq!(exp, Atom(Float(42.3)));
    // // }

    // // #[test]
    // // fn test_read_atom_4() {
    // //     let input = "   42.3";
    // //     let mut reader = Reader::new(input);
    // //     let exp = reader.read().unwrap();
    // //     assert_eq!(exp, Atom(Float(42.3)));
    // // }

    #[test]
    fn test_read_atom_5() {
        let input = "a";
        let arena = &mut types::RuccoArena::default();
        let mut reader = Reader::new(input, arena);
        let exp_ = reader.read().unwrap();
        let exp_ptr = exp_.upgrade().unwrap();
        assert_eq!(*exp_ptr.borrow(), RuccoExp::new_symbol("a"));
    }

    #[test]
    fn test_read_atom_6() {
        let input = "   a";
        let arena = &mut types::RuccoArena::default();
        let mut reader = Reader::new(input, arena);
        let exp_ = reader.read().unwrap();
        let exp_ptr = exp_.upgrade().unwrap();
        assert_eq!(*exp_ptr.borrow(), RuccoExp::new_symbol("a"));
    }

    #[test]
    fn test_read_atom_7() {
        let input = "1+";
        let arena = &mut types::RuccoArena::default();
        let mut reader = Reader::new(input, arena);
        let exp_ = reader.read().unwrap();
        let exp_ptr = exp_.upgrade().unwrap();
        assert_eq!(*exp_ptr.borrow(), RuccoExp::new_symbol("1+"));
    }

    #[test]
    fn test_read_cons_1() {
        let input = "(1 2 3)";
        let arena = &mut types::RuccoArena::default();
        let mut reader = Reader::new(input, arena);
        let exp_ = reader.read().unwrap();
        let exp_ptr = exp_.upgrade().unwrap();
        assert_eq!(
            *exp_ptr.borrow().to_string(),
            "(1 . (2 . (3 . nil)))".to_string()
        );
    }

    #[test]
    fn test_read_cons_2() {
        let input = "(1 2 . 3)";
        let arena = &mut types::RuccoArena::default();
        let mut reader = Reader::new(input, arena);
        let exp_ = reader.read().unwrap();
        let exp_ptr = exp_.upgrade().unwrap();
        assert_eq!(*exp_ptr.borrow().to_string(), "(1 . (2 . 3))".to_string());
    }

    #[test]
    fn test_read_cons_3() {
        let input = "(1 2 3";
        let arena = &mut types::RuccoArena::default();
        let mut reader = Reader::new(input, arena);
        let exp = reader.read().unwrap_err();
        assert_eq!(exp.to_string(), RuccoReaderErr::UnexpectedEof.to_string());
    }

    #[test]
    fn test_read_cons_4() {
        let input = "(1 2 . 3";
        let arena = &mut types::RuccoArena::default();
        let mut reader = Reader::new(input, arena);
        let exp = reader.read().unwrap_err();
        assert_eq!(exp.to_string(), RuccoReaderErr::UnexpectedEof.to_string());
    }

    #[test]
    fn test_read_cons_5() {
        let input = "(1 2 3))";
        let arena = &mut types::RuccoArena::default();
        let mut reader = Reader::new(input, arena);
        let exp_ = reader.read().unwrap();
        let exp_ptr = exp_.upgrade().unwrap();
        assert_eq!(
            *exp_ptr.borrow().to_string(),
            "(1 . (2 . (3 . nil)))".to_string()
        );
    }

    #[test]
    fn test_read_cons_6() {
        let input = "(1 2 . 3))";
        let arena = &mut types::RuccoArena::default();
        let mut reader = Reader::new(input, arena);
        let exp_ = reader.read().unwrap();
        let exp_ptr = exp_.upgrade().unwrap();
        assert_eq!(*exp_ptr.borrow().to_string(), "(1 . (2 . 3))".to_string());
    }
}
