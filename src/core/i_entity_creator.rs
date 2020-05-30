/*************************************************
//  Copyright (C), 2020-2020, luwangda.
//  File name:       i_entity_creator.rs
//  Author:        	 luwangda       
//  Version:         1.0     
//  Date:            2020/05/16
//  Description:     entity creator interface define
//  Others:
//  History:    
*************************************************/
use i_entity_info;

#[derive(Debug)]
struct i_entity_prop {
    name_: &str;
    type_: i32;
    get_func_: fn();
    set_func_: fn();
    next_: *i_entity_prop;
}

#[derive(Debug)]
struct i_entity_func {
    name_: &str;
    mid_func_: fn();
    return_table_: bool
    next_: i_entity_func;
}

trait i_entity_creator {

    // 是否为纯虚类（只能用来继承）
    pub fn is_abstract() -> bool {
        return false;
    }

    // 获取父类名称
    pub fn get_parent() -> &str;

    // 返回名字空间
    pub fn get_space() -> &str;

    // 返回名称
    pub fn get_name() -> &str;

    //创建
    pub fn create() i_base_

    next_: *i_entity_creator;
    property_: *i_entity_prop;
    method_: *i_entity_func;
}
