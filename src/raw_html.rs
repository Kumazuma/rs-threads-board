extern crate ructe;
extern crate core;
use self::ructe::templates::ToHtml;
use self::core::fmt::Display;
use std::io::{Write,Error};
use std::mem::transmute;
pub struct Raw{
    obj:*const Display
}
impl Raw{
    pub fn to_html(&self, out: &mut Write) -> Result<(),Error>{
        let t:&Display = unsafe{transmute::<_, &Display>(self.obj)};
        write!(out, "{}", t)
    }
}
pub fn raw<T: Display + 'static>(obj:&T)->Raw{
    Raw{obj:obj}
}