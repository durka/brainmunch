use std::{fmt, str};

#[derive(Debug)]
pub struct Machine<'a> {
    pub memory: CambridgeArray<'a, u8>,
    pub output: UTF8Wrapper<'a>,
#ifdef PROFILE
    pub trace:  ProfileShim,
#endif
}

pub struct CambridgeArray<'a, T: 'a>(pub &'a [T]); // Cambridge is Oxford's rival
pub struct UTF8Wrapper<'a>(pub &'a [u8]);
#ifdef PROFILE
pub struct ProfileShim(pub fn() -> Profile);
#endif

impl<'a, T: fmt::Display> fmt::Debug for CambridgeArray<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "["));
        if self.0.len() > 0 {
            for e in &self.0[..] {
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

#ifdef PROFILE
#[derive(Debug, Default)]
pub struct Profile {
    pub instructions: u32,
    pub increments: u32, pub decrements: u32, pub overflows: u32, pub underflows: u32,
    pub lefts: u32, pub rights: u32, pub left_grows: u32, pub right_grows: u32,
    pub ins: u32, pub in_revconvs: u32, pub in_unaries: u32, pub eofs: u32,
    pub outs: u32, pub out_revs: u32,
    pub loops: u32, pub clears: u32,
    pub noops: u32,
}

impl fmt::Debug for ProfileShim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0(), f)
    }
}
#endif

