use super::lua_object::*;

pub(crate) struct LuaState {
    CommonHeader!(),
    status: 
}

pub union GCObject {
    gch: GCHeader;
}