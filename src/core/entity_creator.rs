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

#[derive(Debug)]
struct IEntityProp<'a> {
    name_: &'a str,
    type_: i32,
    get_func_: fn(),
    set_func_: fn(),
    next_: *i_entity_prop,
}

#[derive(Debug)]
struct IEntityFunc<'a> {
    name_: &'a str,
    mid_func_: fn(),
    return_table_: bool,
    next_: i_entity_func,
}

pub(crate) trait IEntityCreator {

    // 是否为纯虚类（只能用来继承）
    fn is_abstract() -> bool {
        return false;
    }

    // 获取父类名称
    fn get_parent() -> &str;

    // 返回名字空间
    fn get_space() -> &str;

    // 返回名称
    fn get_name() -> &str;

    //创建
    fn create() -> * dyn IBaseEntity;

    // 删除
    fn destroy(p: * dyn IBaseEntity);

    // 获取下一个
    fn get_next() -> * dyn IEntityCreator;

    // 获得属性链表
    fn get_property_link() -> * IEntityProp;

    // 设置属性链表
    fn set_property_link(value: *IEntityProp);

    // 获得方法链表
    fn get_method_link() -> * IEntityFunc;

    // 设置方法链表
    fn set_method_link(value: *IEntityFunc);
}

struct EntityCreator<'a> {
    next_: * EntityCreator<'a>,
    property_: *IEntityProp<'a>,
    method_: *IEntityFunc<'a>,
}

impl IEntityCreator for EntityCreator {
    fn is_abstract() -> bool {
        unimplemented!()
    }

    fn get_parent() -> &str {
        unimplemented!()
    }

    fn get_space() -> &str {
        unimplemented!()
    }

    fn get_name() -> &str {
        unimplemented!()
    }

    fn create() {
        unimplemented!()
    }

    fn destroy(p: *const dyn IBaseEntity) {
        unimplemented!()
    }

    fn get_next() -> *const dyn IEntityCreator {
        unimplemented!()
    }

    fn get_property_link<'a>() -> *const IEntityProp<'a> {
        unimplemented!()
    }

    fn set_property_link(value: *const IEntityProp) {
        unimplemented!()
    }

    fn get_method_link<'a>() -> *const IEntityFunc<'a> {
        unimplemented!()
    }

    fn set_method_link(value: *const IEntityFunc) {
        unimplemented!()
    }
}

