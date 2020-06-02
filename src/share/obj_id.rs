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

use engine::i_object;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
struct ObjId {
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
    obj_ptr_: *IObject,
}

impl New for ObjIdDbg {
    fn new(id: &ObjId, obj_ptr: *IObject) -> obj_id_dbg {
        obj_ = id;
        obj_ptr_ = obj_ptr;
        Self
    }
}

impl Deref for ObjIdDbg {
    type Target = ObjId;
    fn deref(&self) -> &Self::Target {
         &self.obj_
    }
}
