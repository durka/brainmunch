use std::{fmt, str};

#[derive(Debug)]
pub struct Machine<'a> {
    pub memory: CambridgeArray<'a, u8>,
    pub output: UTF8Wrapper<'a>,
}

pub struct CambridgeArray<'a, T: 'a>(pub &'a [T]); // Cambridge is Oxford's rival
pub struct UTF8Wrapper<'a>(pub &'a [u8]);

impl<'a, T: fmt::Display> fmt::Debug for CambridgeArray<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "["));
        if self.0.len() > 0 {
            for e in &self.0[1..] {
                try!(write!(f, " {}", e));
            }
        }
        write!(f, " ]")
    }
}
impl<'a> fmt::Debug for UTF8Wrapper<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n{}", try!(str::from_utf8(self.0).map_err(|_| fmt::Error)))
    }
}

