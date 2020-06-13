/*************************************************
//  Copyright (C), 2020-2020, luwangda.
//  File name:       i_any_list.h
//  Author:        	 luwangda
//  Version:         1.0
//  Date:            2020/05/16
//  Description:     array list
//  Others:          可变类型列表
//  History:
*************************************************/
use std::ops::Shl;
use super::obj_id::ObjId;
use super::any_type::VarType;

#[derive(Copy, Clone)]
union AnyType {
    v_i8: i8,
    v_i16: i16,
    v_i32: i32,
    v_i64: i64,
    v_f32: f32,
    v_f64: f64,
    v_bool: bool,
    v_obj: ObjId,
    v_str: &'static str,
}

#[derive(Copy, Clone)]
pub struct VarData {
    type_: VarType,
    data_: AnyType,
}

pub(crate) struct IArrayList {
    data_stack_: Vec<VarData>,
}

impl<> IArrayList {
    // 合并列表
    pub fn combine(&self, src: &Self) -> bool {}

    fn InnerAppend(src: &IArrayList, start: usize, end: usize) {
        for i in start .. end {
            match src.get_type(i) {
                VarType_I8 => self.add_value_i8(src.get_value_i8());
            }
        }
    }

    // 附加列表
    fn append(&self, src: &Self, start: u32, count: u32) {}

    // 清空
    pub fn clear(&self) {}

    // 是否为空
    fn is_empty(&self) -> bool {}

    // 数据长度
    fn get_count(&self) -> usize {}

    // 数据类型
    fn get_type(&self, index: usize) -> i32 {
        if index < 0 || index >= self.data_stack_.len() {
            return 0;
        }

        self.data_stack_[index].type_
    }

    // 获取内存用量
    fn get_memory_usage(&self) -> u32 {}

    pub fn add_bool(&self, value: bool) -> bool {
        let var = VarData{type_: VarType::VarType_Bool, data_: AnyType{v_bool: value}};
        self.data_stack_.push(var);
        true
    }

    pub fn add_i8(&self, value: i8) -> bool {
        let var = VarData{type_: VarType::VarType_I8, data_: AnyType{v_i8: value}};
        self.data_stack_.push(var);
        true
    }

    pub fn add_i16(&self, value: i16) -> bool {
        let var = VarData{type_: VarType::VarType_I16, data_: AnyType{v_i16: value}};
        self.data_stack_.push(var);
        true
    }

    pub fn add_i32(&self, value: i32) -> bool {
        let var = VarData{type_: VarType::VarType_I32, data_: AnyType{v_I32: value}};
        self.data_stack_.push(var);
        true
    }

    pub fn add_i64(&self, value: i64) -> bool {
        let var = VarData{type_: VarType::VarType_I64, data_: AnyType{v_i64: value}};
        self.data_stack_.push(var);
        true
    }

    pub fn add_f32(&self, value: f32) -> bool {
        let var = VarData{type_: VarType::VarType_F32, data_: AnyType{v_f32: value}};
        self.data_stack_.push(var);
        true
    }

    pub fn add_f64(&self, value: f64) -> bool {
        let var = VarData{type_: VarType::VarType_F64, data_: AnyType{v_f64: value}};
        self.data_stack_.push(var);
        true
    }

    pub fn add_str(&self, value: &'static str) -> bool {
        let var = VarData{type_: VarType::VarType_Str, data_: AnyType{v_str: value}};
        self.data_stack_.push(var);
        true
    }
 
    pub fn add_obj(&self, value: ObjId) -> bool {
        let var = VarData{type_: VarType::VarType_Obj, data_: AnyType{v_obj: value}};
        self.data_stack_.push(var);
        true
    }

    pub fn set_value<T>(&self, value: T) -> bool {}

    pub fn get_bool(&self, index: usize) -> bool {
        if index >= self.data_stack_.len() {
            return false;
        }
        self.data_stack_[index].data_.v_bool
    }

    pub fn get_i8(&self, index: usize) -> i8 {
        if index >= self.data_stack_.len() {
            return 0;
        }
        self.data_stack_[index].data_.v_i8
    }

    pub fn get_i16(&self, index: usize) -> i16 {
        if index >= self.data_stack_.len() {
            return 0;
        }
        self.data_stack_[index].data_.v_i16
    }

    pub fn get_i32(&self, index: usize) -> i32 {
        if index >= self.data_stack_.len() {
            return 0;
        }
        self.data_stack_[index].data_.v_i32
    }

    pub fn get_i64(&self, index: usize) -> i64 {
        if index >= self.data_stack_.len() {
            return 0;
        }
        self.data_stack_[index].data_.v_i64
    }

    pub fn get_f32(&self, index: usize) -> f32 {
        if index >= self.data_stack_.len() {
            return 0.0;
        }
        self.data_stack_[index].data_.v_f32
    }

    pub fn get_f64(&self, index: usize) -> f64 {
        if index >= self.data_stack_.len() {
            return 0.0;
        }
        self.data_stack_[index].data_.v_f64
    }

    pub fn get_str(&self, index: usize) -> &'static str {
        if index >= self.data_stack_.len() {
            return "";
        }
        self.data_stack_[index].data_.v_str
    }

    pub fn get_obj(&self, index: usize) -> ObjId {
        if index >= self.data_stack_.len() {
            return ObjId::new_null();
        }
        self.data_stack_[index].data_.v_obj
    }
}

impl Shl for IArrayList {
    fn Shl<T>(&self, value: T) {
        self.add_value(value);
    }
}
