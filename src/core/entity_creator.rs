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
    fn create<T: IBaseEntity>(&self) -> Option<T>;

    // 删除
    fn destroy<T: IBaseEntity>(&self, p: T);

    // 获取下一个
    fn get_next<T: IEntityCreator>(&self) -> Option<T>;

    // 获得属性链表
    fn get_property_link(&self) -> Option<EntityProp>;

    // 设置属性链表
    fn set_property_link(&mut self, value: EntityProp);

    // 获得方法链表
    fn get_method_link(&self) -> Option<EntityFunc>;

    // 设置方法链表
    fn set_method_link(&mut self, value: EntityFunc);
}

struct EntityCreator {
    next_: Some(EntityCreator),
    property_: Some(EntityProp),
    method_: Some(EntityFunc),
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

    fn destroy<T: IBaseEntity>(&self, p: T) {
        unimplemented!()
    }

    fn get_next<T: IEntityCreator>(&self) -> Option<T> {
        unimplemented!()
    }

    fn get_property_link(&self) -> Option<EntityProp> {
        self.property_
    }

    fn set_property_link(&mut self, value: EntityProp) {
        self.property_ = value;
    }

    fn get_method_link(&self) -> Option<EntityFunc> {
        self.method_
    }

    fn set_method_link(&mut self, value: EntityFunc) {
        self.method_ = value;
    }
}

