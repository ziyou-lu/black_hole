/*************************************************
//  Copyright (C), 2020-2020, luwangda.
//  File name:       IEntityInfo
//  Author:        	 luwangda
//  Version:         1.0
//  Date:            2020/05/16
//  Description:     entity info interface define
//  Others:
//  History:
*************************************************/
use crate::share::any_list::*;
use crate::core::base_entity::IBaseEntity;
use super::entity_creator::IEntityCreator;

// 实体属性信息
trait IPropInfo {
    // 获取名字
    fn get_name(&self) -> String;
    // 获取类型
    fn get_type(&self) -> i32;
    // 获取get方法
    fn get_get_func(&self) -> fn();
    // 获取set方法
    fn get_set_func(&self) -> fn();
}

// 实体方法信息
trait IFuncInfo {
    // 获取名字
    fn get_name(&self) -> String;
    // 获取修改方法
    fn get_mid_func(&self) -> fn();
    // 获取是否返回
    fn get_returnable(&self) -> bool;
}

// 实体信息
pub(crate) trait IEntityInfo {
    // 获取创建器
    fn get_creator<T: IEntityCreator>(&self) -> Option<T>;

    // 获取父类名称
    fn get_parent_name(&self) -> String;

    // 返回名字空间
    fn get_space_name(&self) -> String;

    // 返回类名
    fn get_entity_name(&self) -> String;

    // 获取父类信息
    fn get_parent<T: IEntityInfo>(&self) -> Option<T>;

    // 是否属于某类或是自雷
    fn is_kind_of(&self, name: &str) -> bool;

    // 是否属于统一名字空间的某类或者是其子类
    fn is_kind_same_space<T: IBaseEntity>(&self, p_entity: &T, name: &str) -> bool;

    // 实体属性
    fn get_property_count(&self) -> u32;

    // 获得属性名字列表
    fn get_property_list<T: IArrayList>(&self, result: &mut T) -> u32;

    // 在本类中获得属性信息
    fn get_property_info<T: IPropInfo>(&self, name: &str) -> Option<T>;

    // 在本类和父类查找属性嘻嘻
    fn find_property_info<T: IPropInfo>(&self, name: &str) -> Option<T>;

    // 获得本类和父类的属性名字列表
    fn get_property_all<T: IArrayList>(&self, result: &mut T) -> u32;
}


