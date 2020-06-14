use super::lua_state::*;

pub(crate) struct LuaState {
    
}

pub union GCObject {
    gch: GCHeader;
}