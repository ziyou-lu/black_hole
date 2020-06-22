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
use super::obj_id::ObjId;
use super::any_type::*;

pub(crate) struct IArrayList {
    data_stack_: Vec<VarData>,
}

impl<> IArrayList {
    // 合并列表
    pub fn combine(&self, src: &Self) {
        self.InnerAppend(src, 0, src.get_count());
    }

    fn InnerAppend(&self, src: &IArrayList, start: usize, end: usize) {
        for i in start .. end {
            match src.get_type(i) {
                VarType::VarTypeBool => {self.add_bool(src.get_bool(i));},
                VarType::VarTypeI8 => {self.add_i8(src.get_i8(i));},
                VarType::VarTypeI16 => {self.add_i16(src.get_i16(i));},
                VarType::VarTypeI32 => {self.add_i32(src.get_i32(i));},
                VarType::VarTypeI64 => {self.add_i64(src.get_i64(i));},
                VarType::VarTypeF32 => {self.add_f32(src.get_f32(i));},
                VarType::VarTypeF64 => {self.add_f64(src.get_f64(i));},
                VarType::VarTypeStr => {self.add_str(src.get_str(i));},
                VarType::VarTypeObj => {self.add_obj(src.get_obj(i));},
                _ => {println!("type is not valid");},
            }
        }
    }

    // 附加列表
    fn append(&self, src: &Self, start: u32, count: u32) {}

    // 清空
    pub fn clear(&self) {
        self.data_stack_.clear();
    }

    // 是否为空
    pub fn is_empty(&self) -> bool {
        self.get_count() == 0
    }

    // 数据长度
    pub fn get_count(&self) -> usize {
        self.data_stack_.len()
    }

    // 数据类型
    pub fn get_type(&self, index: usize) -> VarType {
        if index < 0 || index >= self.data_stack_.len() {
            return VarType::VarTypeUnKnow;
        }

        self.data_stack_[index].type_
    }

    // 获取内存用量
    fn get_memory_usage(&self) -> usize {
        let mem: usize = 0;
        for i in 0 .. self.get_count() {
            match self.get_type(i) {
                VarType::VarTypeBool => mem += 2,
                VarType::VarTypeI8 => mem += 4,
                VarType::VarTypeI16 => mem += 8,
                VarType::VarTypeI32 => mem += 16,
                VarType::VarTypeI64 => mem += 32,
                VarType::VarTypeI128 => mem += 64,
                VarType::VarTypeF32 => mem += 16,
                VarType::VarTypeF64 => mem += 32,
                VarType::VarTypeStr => mem += self.get_str(i).len() * 2,
                VarType::VarTypeObj => mem += 32,
                _ => println!("type not invalid"),
             }
        }
        mem
    }

    pub fn add_bool(&self, value: bool) -> bool {
        let var = VarData{type_: VarType::VarTypeBool, data_: AnyType{v_bool: value}};
        self.data_stack_.push(var);
        true
    }

    pub fn add_i8(&self, value: i8) -> bool {
        let var = VarData{type_: VarType::VarTypeI8, data_: AnyType{v_i8: value}};
        self.data_stack_.push(var);
        true
    }

    pub fn add_i16(&self, value: i16) -> bool {
        let var = VarData{type_: VarType::VarTypeI16, data_: AnyType{v_i16: value}};
        self.data_stack_.push(var);
        true
    }

    pub fn add_i32(&self, value: i32) -> bool {
        let var = VarData{type_: VarType::VarTypeI32, data_: AnyType{v_i32: value}};
        self.data_stack_.push(var);
        true
    }

    pub fn add_i64(&self, value: i64) -> bool {
        let var = VarData{type_: VarType::VarTypeI64, data_: AnyType{v_i64: value}};
        self.data_stack_.push(var);
        true
    }

    pub fn add_f32(&self, value: f32) -> bool {
        let var = VarData{type_: VarType::VarTypeF32, data_: AnyType{v_f32: value}};
        self.data_stack_.push(var);
        true
    }

    pub fn add_f64(&self, value: f64) -> bool {
        let var = VarData{type_: VarType::VarTypeF64, data_: AnyType{v_f64: value}};
        self.data_stack_.push(var);
        true
    }

    pub fn add_str(&self, value: &'static str) -> bool {
        let var = VarData{type_: VarType::VarTypeStr, data_: AnyType{v_str: value}};
        self.data_stack_.push(var);
        true
    }
 
    pub fn add_obj(&self, value: ObjId) -> bool {
        let var = VarData{type_: VarType::VarTypeObj, data_: AnyType{v_obj: value}};
        self.data_stack_.push(var);
        true
    }

    pub fn set_bool(&self, index: usize, value: bool) {
        self.data_stack_[index].type_ = VarType::VarTypeBool;
        self.data_stack_[index].data_ = AnyType{v_bool: value};
    }
    
    pub fn set_i8(&self, index: usize, value: i8) {
        self.data_stack_[index].type_ = VarType::VarTypeI8;
        self.data_stack_[index].data_ = AnyType{v_i8: value};
    }

    pub fn set_i16(&self, index: usize, value: i16) {
        self.data_stack_[index].type_ = VarType::VarTypeI16;
        self.data_stack_[index].data_ = AnyType{v_i16: value};
    }

    pub fn set_i32(&self, index: usize, value: i32) {
        self.data_stack_[index].type_ = VarType::VarTypeI32;
        self.data_stack_[index].data_ = AnyType{v_i32: value};
    }

    pub fn set_i64(&self, index: usize, value: i64) {
        self.data_stack_[index].type_ = VarType::VarTypeI64;
        self.data_stack_[index].data_ = AnyType{v_i64: value};
    }

    pub fn set_f32(&self, index: usize, value: f32) {
        self.data_stack_[index].type_ = VarType::VarTypeF32;
        self.data_stack_[index].data_ = AnyType{v_f32: value};
    }

    pub fn set_f64(&self, index: usize, value: f64) {
        self.data_stack_[index].type_ = VarType::VarTypeF64;
        self.data_stack_[index].data_ = AnyType{v_f64: value};
    }

    pub fn set_str(&self, index: usize, value: &'static str) {
        self.data_stack_[index].type_ = VarType::VarTypeStr;
        self.data_stack_[index].data_ = AnyType{v_str: value};
    }

    pub fn set_obj(&self, index: usize, value: ObjId) {
        self.data_stack_[index].type_ = VarType::VarTypeObj;
        self.data_stack_[index].data_ = AnyType{v_obj: value};
    }

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
