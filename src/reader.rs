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

        if let Some(m) = FLOAT_PATTERN.captures(self.input) {
            let s = m.get(1).unwrap().as_str();
            let f = s.parse::<f64>().unwrap();
            self.input = &self.input[s.len()..];

            return Ok(self.arena.alloc(f.into()));
        }

        if let Some(m) = SYMBOL_PATTERN.captures(self.input) {
            let s = m.get(0).unwrap().as_str();
            self.input = &self.input[s.len()..];
            return Ok(self.arena.alloc_symbol(s));
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
                Ok(self.arena.alloc_symbol("nil"))
            }
            Some(_) => {
                let car = self.read()?;
                let mut cur = types::alloc!(self.arena, [car]);
                let mut cur_ptr = cur
                    .upgrade()
                    .ok_or(types::RuccoRuntimeErr::InvalidReference)?;

                let res = cur.clone();
                let mut prev = cur.clone();
                loop {
                    self.skip_whitespace();
                    match self.input.chars().next() {
                        None => anyhow::bail!(types::RuccoReaderErr::UnexpectedEof),
                        Some(')') => {
                            self.input = &self.input[1..]; // skip ')'
                            break;
                        }
                        Some('.') => {
                            self.input = &self.input[1..]; // skip '.'
                            let cdr = self.read()?;

                            self.skip_whitespace();
                            match self.input.chars().next() {
                                None => anyhow::bail!(types::RuccoReaderErr::UnexpectedEof),
                                Some(')') => {
                                    self.input = &self.input[1..]; // skip ')'
                                    cur_ptr.borrow_mut().setcdr(&cdr)?;
                                    break;
                                }
                                Some(char) => {
                                    anyhow::bail!(types::RuccoReaderErr::UnexpectedChar { char })
                                }
                            }
                        }
                        Some(_) => {
                            let car = self.read()?;
                            cur = types::alloc!(self.arena, [car]);
                            cur_ptr = cur
                                .upgrade()
                                .ok_or(types::RuccoRuntimeErr::InvalidReference)?;

                            let prev_ptr = prev
                                .upgrade()
                                .ok_or(types::RuccoRuntimeErr::InvalidReference)?;
                            prev_ptr.borrow_mut().setcdr(&cur)?;
                            prev = cur;
                        }
                    }
                }
                Ok(res)
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
            '\'' => {
                self.input = &self.input[1..]; // skip '\''
                let quote = self.arena.alloc_symbol("quote");
                let exp = self.read()?;

                Ok(types::alloc!(self.arena, [quote, exp]))
            }
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

    #[test]
    fn test_read_atom_3() {
        let input = "42.3";
        let arena = &mut types::RuccoArena::default();
        let mut reader = Reader::new(input, arena);
        let exp = reader.read().unwrap();
        let exp_ptr = exp.upgrade().unwrap();
        assert_eq!(*exp_ptr.borrow(), 42.3.into());
    }

    #[test]
    fn test_read_atom_4() {
        let input = "   42.3";
        let arena = &mut types::RuccoArena::default();
        let mut reader = Reader::new(input, arena);
        let exp = reader.read().unwrap();
        let exp_ptr = exp.upgrade().unwrap();
        assert_eq!(*exp_ptr.borrow(), 42.3.into());
    }

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
        assert_eq!(*exp_ptr.borrow().to_string(), "(1 2 3)".to_string());
    }

    #[test]
    fn test_read_cons_2() {
        let input = "(1 2 . 3)";
        let arena = &mut types::RuccoArena::default();
        let mut reader = Reader::new(input, arena);
        let exp_ = reader.read().unwrap();
        let exp_ptr = exp_.upgrade().unwrap();
        assert_eq!(*exp_ptr.borrow().to_string(), "(1 2 . 3)".to_string());
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
        assert_eq!(*exp_ptr.borrow().to_string(), "(1 2 3)".to_string());
    }

    #[test]
    fn test_read_cons_6() {
        let input = "(1 2 . 3))";
        let arena = &mut types::RuccoArena::default();
        let mut reader = Reader::new(input, arena);
        let exp_ = reader.read().unwrap();
        let exp_ptr = exp_.upgrade().unwrap();
        assert_eq!(*exp_ptr.borrow().to_string(), "(1 2 . 3)".to_string());
    }

    #[test]
    fn test_read_quote() {
        let input = "'a";
        let arena = &mut types::RuccoArena::default();
        let mut reader = Reader::new(input, arena);
        let exp_ = reader.read().unwrap();
        let exp_ptr = exp_.upgrade().unwrap();
        assert_eq!(*exp_ptr.borrow().to_string(), "(quote a)".to_string());
    }
}
