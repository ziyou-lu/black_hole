/*************************************************
//  Copyright (C), 2020-2020, luwangda.
//  File name:       IEntityCreator
//  Author:        	 luwangda       
//  Version:         1.0     
//  Date:            2020/05/16
//  Description:     entity creator interface define
//  Others:
//  History:    
*************************************************/
use super::entity_info::IEntityInfo;
use super::base_entity::IBaseEntity;
use std::os::raw::c_char;

#[derive(Debug)]
struct EntityProp {
    name_: String,
    type_: i32,
    get_func_: fn(),
    set_func_: fn(),
    next_: *EntityProp,
}

#[derive(Debug)]
struct EntityFunc {
    name_: String,
    mid_func_: fn(),
    return_table_: bool,
    next_: * EntityFunc,
}

pub(crate) trait IEntityCreator {

    // 是否为纯虚类（只能用来继承）
    fn is_abstract(&self) -> bool {
        return false;
    }

    // 获取父类名称
    fn get_parent(&self) -> String;

    // 返回名字空间
    fn get_space(&self) -> String;

    // 返回名称
    fn get_name(&self) -> String;

    //创建
    fn create(&self) -> *const dyn IBaseEntity;

    // 删除
    fn destroy(&self, p: *const dyn IBaseEntity);

    // 获取下一个
    fn get_next(&self) -> *const dyn IEntityCreator;

    // 获得属性链表
    fn get_property_link(&self) -> *const EntityProp;

    // 设置属性链表
    fn set_property_link(&self, value: *const EntityProp);

    // 获得方法链表
    fn get_method_link(&self) -> *const EntityFunc;

    // 设置方法链表
    fn set_method_link(&self, value: *const EntityFunc);
}

struct EntityCreator {
    next_: * EntityCreator,
    property_: * EntityProp,
    method_: * EntityFunc,
}

impl IEntityCreator for EntityCreator {
    fn is_abstract(&self) -> bool {
        unimplemented!()
    }

    fn get_parent(&self) -> String {
        unimplemented!()
    }

    fn get_space(&self) -> String {
        unimplemented!()
    }

    fn get_name(&self) -> String {
        unimplemented!()
    }

    fn create(&self) {
        unimplemented!()
    }

    fn destroy(&self, p: *const dyn IBaseEntity) {
        unimplemented!()
    }

    fn get_next(&self) -> *const dyn IEntityCreator {
        self.next_
    }

    fn get_property_link(&self) -> *const EntityProp {
        self.property_
    }

    fn set_property_link(&mut self, value: *const EntityProp) {
        self.property_ = value;
    }

    fn get_method_link(&self) -> *const EntityFunc {
        self.method_
    }

    fn set_method_link(&mut self, value: *const EntityFunc) {
        self.method_ = value;
    }
}

