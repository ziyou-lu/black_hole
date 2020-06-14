use super::lua_state::*;
#[macro_export]
macro_rules! CommonHeader{
    (*) => {
        let next: Option<Box<GCObject>>;
        let tt: char;
        let marked: char;
    };
}

pub struct GCHeader {
    CommonHeader!;
}