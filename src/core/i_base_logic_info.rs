/*************************************************
//  Copyright (C), 2020-2020, luwangda.
//  File name:       i_base_logic_info.h
//  Author:        	 luwangda
//  Version:         1.0
//  Date:            2020/05/16
//  Description:     base logic info interface define
//  Others:
//  History:
*************************************************/

// 逻辑方法信息
#[derive(Debug)]
trait i_call_back_info {
    pub fn get_name() -> str;
    pub fn get_nid_func() -> fn();
    pub fn get_return_table() -> bool;
}

trait i_base_logic_info {
    // 获得创建起
    pub fn get_creator() -> *i_base_logic_creator;
    
    // 返回名字空间
    pub fn get_space_name() -> str;

    // 返回类名
    pub fn get_logic_name() -> str;

    // 方法数量
    pub fn get_callback_count() -> u32;

    // 获取方法名字列表
    pub fn get_callback_list(result: &i_array_list) -> u32;

    // 在本类中获取方法信息
    pub fn get_callback_info(name: &str) -> *i_call_back_info;
}
