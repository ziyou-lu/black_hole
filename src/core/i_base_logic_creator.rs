/*************************************************
//  Copyright (C), 2020-2020, luwangda.
//  File name:       i_base_creator.rs
//  Author:        	 luwangda
//  Version:         1.0
//  Date:            2020/05/16
//  Description:     base interface creator define
//  Others:
//  History:
*************************************************/
use i_base;

#[derive(Debug)]
struct i_base_logic_call_back {
    name_: str;
    mid_func_: fn();
    return_table_: bool;
    next_: *i_base_logic_call_back;
}

trait i_base_logic_creator {
    // 返回空间名字
    pub fn get_space() -> &str;
    // 返回名称
    pub fn get_name() -> &str;
    // 创建
    pub fn create() -> *bi;
    // 删除
    pub fn destroy(p: *bi);
    // 获得下一个
    pub fn get_next() -> *i_base_logic_creator {
        return next_;
    }
    // 获取方法链表
    pub fn get_callback_link() -> *i_base_logic_call_back {
        return call_back_;
    }

    // 设置方法链表
    pub fn set_callback_link(value: *i_base_logic_call_back) {
        call_back_ = value;
    }

    next_ *i_base_creator;
    call_back_ *i_base_logic_call_back;
}
