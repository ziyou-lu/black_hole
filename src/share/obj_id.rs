/*************************************************
//  Copyright (C), 2020-2020, luwangda.
//  File name:       ObjId.rs
//  Author:        	 sll
//  Version:         1.0
//  Date:            2017/08/07
//  Description:
//  Others:
//  History:
*************************************************/
use std::ops::{Deref, DerefMut};
use crate::core::object::IObject;

#[derive(Debug)]
pub(crate) struct ObjId {
    ident_: u32,
    serial_: u32,
}

impl ObjId {
    fn is_null(&self) -> bool{
        self.ident_ == 0 && self.serial_ == 0
    }

    fn equal_to(&self, other: &ObjId) -> bool{
        self.ident_ == other.ident_ && self.serial_ == other.serial_
    }
}

#[derive(Debug)]
struct ObjIdDbg {
    obj_: ObjId,
    obj_ptr_: * dyn IObject,
}

impl ObjIdDbg {
    fn new(id: ObjId, obj_ptr: * dyn IObject) -> ObjIdDbg {
        ObjIdDbg{obj_: id, obj_ptr_: obj_ptr}
    }
}

impl Deref for ObjIdDbg {
    type Target = ObjId;
    fn deref(&self) -> &Self::Target {
         &self.obj_
    }
}
