/*************************************************
//  Copyright (C), 2020-2020, luwangda.
//  File name:       i_entity_info.rs
//  Author:        	 luwangda
//  Version:         1.0
//  Date:            2020/05/16
//  Description:     entity info interface define
//  Others:
//  History:
*************************************************/

use i_entity_creator;

// 实体属性信息
trait i_prop_info {
    // 获取名字
    fn get_name() -> &str;
    // 获取类型
    fn get_type() -> i32;
    // 获取get方法
    fn get_getfunc() -> fn();
    // 获取set方法
    fn get_setfunc() -> fn();
}

// 实体方法信息
trait i_func_info {
    // 获取名字
    fn get_name() -> &str;
    // 获取修改方法
    fn get_midfunc() -> fn();
    // 获取是否返回
    fn get_returnable() -> bool;
}

// 实体信息
trait i_entity_info {
    // 获取创建器
    pub fn get_creator() -> *i_entity_creator;

    // 获取父类名称
    pub fn get_parent_name() -> str;

    // 返回名字空间
    pub fn get_space_name() -> str;

    // 返回类名
    pub fn get_entity_name() -> str;

    // 获取父类信息
    pub fn get_parent() -> *i_entity_info;

    // 是否属于某类或是自雷
    pub fn is_kind_of(name: &str) -> bool;

    // 是否属于统一名字空间的某类或者是其子类
    pub fn is_kind_same_space(p_entity: *i_base_entity, name: &str) -> bool;

    // 实体属性
    pub fn get_property_count() -> u32;

    // 获得属性名字列表
    pub fn get_property_list(result: &i_array_list) -> u32;

    // 在本类中获得属性信息
    pub fn get_property_info(name: &str) -> *i_prop_info;

    // 在本类和父类查找属性嘻嘻
    pub fn find_property_info(name: &str) -> *i_prop_info;

    // 获得本类和父类的属性名字列表
    pub fn get_property_all(result: &i_array_list) -> u32;

    // 
}
