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

pub(crate) trait IArrayList {
    // 合并列表
    fn combine(src: &IArrayList) -> bool;

    // 附加列表
    fn append(src: &IArrayList, start: u32, count: u32);

    // 清空
    fn clear();

    // 是否为空
    fn is_empty() -> bool;

    // 数据长度
    fn get_count() -> u32;

    // 数据类型
    fn get_type(index: &u32) -> i32;

    fn add_value<T>(value: T) ->bool;

    fn set_value<T>(value: T) -> bool;

    fn get_value<T>() -> T;
    /*// 增加bool数据
    fn add_bool(value: &bool) -> bool;

    // 增加i8数据
    fn add_i8(value: &i8) -> bool;

    // 增加i16数据
    fn add_i16(value: &i16) -> bool;

    // 增加i32数据
    fn add_i32(value: &i32) -> bool;

    // 增加i64数据
    fn add_i64(value: &i64) -> bool;

    // 增加i128数据
    fn add_i128(value: &i128) -> bool;

    // 增加f32数据
    fn add_f32(value: &f32) -> bool;

    // 增加f64数据
    fn add_f64(value: &f64) -> bool;

    // 增加string数据
    fn add_str(value: &str) -> bool;
    fn add_str_pos(start_pos: &str, value: &str) -> bool;

    // 增加对象号数据
    fn add_obj(value: &obj_id) -> bool;*/

    // 修改bool值
    /*fn set_bool(index: u32, value: &bool) -> bool;

    // 修改i8值
    fn set_i8(index: u32, value: &i8) -> bool;

    // 修改i16值
    fn set_i16(index: u32, value: &i16) -> bool;

    // 修改i32值
    fn set_i32(index: u32, value: &i32) -> bool;

    // 修改i64值
    fn set_i64(index: u32, value: &i64) -> bool;

    // 修改i128值
    fn set_i128(index: u32, value: &i128) -> bool;

    // 修改f32值
    fn set_f32(index: u32, value: &f32) -> bool;

    // 修改f64值
    fn set_f64(index: u32, value: &f64) -> bool;

    // 修改str值
    fn set_str(index: u32, value: &str) -> bool;

    // 修改obj值
    fn set_obj(index: u32, value: &obj_id) -> bool;*/

    // 获取bool值
   /* fn get_bool(index: u32) -> bool;

    // 获取i8值
    fn get_i8(index: u32) -> i8;

    // 获取i16值
    fn get_i16(index: u32) -> i16;

    // 获取i32值
    fn get_i32(index: u32) -> i32;

    // 获取i64值
    fn get_i64(index: u32) -> i64;

    // 获取i128值
    fn get_i128(index: u32) -> i128;

    // 获取f32值
    fn get_f32(index: u32) -> f32;

    // 获取f64值
    fn get_f64(index: u32) -> f64;

    // 获取obj值
    fn get_str(index: u32) -> str;

    // 获取obj值
    fn get_obj(index: u32) -> obj_id;*/
   
    // 获取内存用量
    fn get_memory_usage() -> u32;

    fn shl<T>(&self, value: T) -> Box<Self> {
        self.add_value(value);
        return self;
    }
}




