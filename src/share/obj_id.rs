/*************************************************
//  Copyright (C), 2020-2020, luwangda.
//  File name:       obj_id.rs
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
struct obj_id {
    ident_: u32;
    serial_: u32;
}

impl obj_id {
    fn is_null(&self) -> bool{
        self.ident_ == 0 && self.serial_ == 0
    }

    fn equal_to(&self, other: &obj_id) -> bool{
        self.ident_ == other.ident_ && self.serial_ == other.serial_
    }
}

#[derive(Debug)]
struct obj_id_dbg {
    obj_: obj_id;
    obj_ptr_: *i_object;

    fn new(id: &obj_id, obj_ptr: *i_object) -> obj_id_dbg {
        self.obj_ = id;
        self.obj_ptr_ = obj_ptr;
    }
}

impl Deref for obj_id_dbg {
    type Target = obj_id;
    fn deref<'a>('a &self) -> 'a obj_id {
         &self.obj_
    }
}
