use super::lua_state::*;
#[macro_export]
macro_rules! CommonHeader{
    () => {
        next: Option<Box<GCObject>>,
        tt: char,
        marked: char,
    }
}

pub struct GCHeader {
    CommonHeader!(),
}