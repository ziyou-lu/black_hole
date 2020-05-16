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

trait i_array_list {
    // 合并列表
    pub combine(src: &i_array_list) -> bool;

    // 附加列表
    pub append(src: &i_array_list, start u32, count u32);

    // 清空
    pub clear();

    // 是否为空
    pub is_empty() -> bool;

    // 数据长度
    pub get_count() -> u32;

    // 数据类型
    pub get_type(index: &u32) -> i32;

    // 增加bool数据
    pub add_bool(value: &bool) -> bool;

    // 增加i8数据
    pub add_i8(value: &i8) -> bool;

    // 增加i16数据
    pub add_i16(value: &i16) -> bool;

    // 增加i32数据
    pub add_i32(value: &i32) -> bool;

    // 增加i64数据
    pub add_i64(value: &i64) -> bool;

    // 增加i128数据
    pub add_i128(value: &i128) -> bool;

    // 增加f32数据
    pub add_f32(value: &f32) -> bool;

    // 增加f64数据
    pub add_f64(value: &f64) -> bool;

    // 增加string数据
    pub add_str(value: &str) -> bool;
    pub add_str(start_pos: &str, value: &str) -> bool;

    // 增加对象号数据
    pub add_obj(value: &obj_id) -> bool;

    // 修改bool值
    pub set_bool(index: u32, value: &bool) -> bool;

    // 修改i8值
    pub set_i8(index: u32, value: &i8) -> bool;

    // 修改i16值
    pub set_i16(index: u32, value: &i16) -> bool;

    // 修改i32值
    pub set_i32(index: u32, value: &i32) -> bool;

    // 修改i64值
    pub set_i64(index: u32, value: &i64) -> bool;

    // 修改i128值
    pub set_i64(index: u32, value: &i128) -> bool;

    // 修改f32值
    pub set_f32(index: u32, value: &f32) -> bool;

    // 修改f64值
    pub set_f64(index: u32, value: &f64) -> bool;

    // 修改str值
    pub set_str(index: u32, value: &str) -> bool;

    // 修改obj值
    pub set_obj(index: u32, value: &obj_id) -> bool;

    // 获取bool值
    pub get_bool(index: u32) -> bool;

    // 获取i8值
    pub get_i8(index: u32) -> i8;

    // 获取i16值
    pub get_i16(index: u32) -> i16;

    // 获取i32值
    pub get_i32(index: u32) -> i32;

    // 获取i64值
    pub get_i64(index: u32) -> i64;

    // 获取i128值
    pub get_i128(index: u32) -> i128;

    // 获取f32值
    pub get_f32(index: u32) -> f32;

    // 获取f64值
    pub get_f64(index: u32) -> f64;

    // 获取obj值
    pub get_str(index: u32) -> str;

    // 获取obj值
    pub get_obj(index: u32) -> obj_id;
   
    // 获取内存用量
    pub get_memory_usage() -> u32;

    pub fn shl(self, value: bool) -> Self {
        self.add_bool(value);
        return self;
    }

    pub fn shl(self, value: i8) -> Self {
        self.add_i8(value);
        return self;
    }

    pub fn shl(self, value: i16) -> Self {
        self.add_i16(value);
        return self;
    }

    pub fn shl(self, value: i32) -> Self {
        self.add_i32(value);
        return self;
    }

    pub fn shl(self, value: i64) -> Self {
        self.add_i64(value);
        return self;
    }

    pub fn shl(self, value: i128) -> Self {
        self.add_i128(value);
        return self;
    }

    pub fn shl(self, value: f32) -> Self {
        self.add_f32(value);
        return self;
    }

    pub fn shl(self, value: f64) -> Self {
        self.add_f64(value);
        return self;
    }

    pub fn shl(self, value: &str) -> Self {
        self.add_str(value);
        return self;
    }

    pub fn shl(self, value: obj_id) -> Self {
        self.add_obj(value);
        return self;
    }

    pub fn shl(self, value: &i_array_list) -> Self {
        self.combine(value);
        return self;
    }
}
