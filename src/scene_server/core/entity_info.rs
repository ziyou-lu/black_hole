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
use super::base_entity::IBaseEntity;
use super::entity_creator::IEntityCreator;
use crate::share::any_list::*;
use crate::runtime::inlines;
use std::os::raw::c_void;

// 实体属性信息
#[derive(Debug)]
pub(crate) struct IPropInfo {
    name_: String,
    hash_: u32,
    type_: i32,
    get_fn_: *const c_void,
    set_fn_: *const c_void,
}

impl IPropInfo {
    // 获取名字
    fn get_name(self) -> String {
        self.name_
    }
    // 获取类型
    fn get_type(&self) -> i32 {
        self.type_
    }
    // 获取get方法
    fn get_get_func(&self) -> *const c_void {
        self.get_fn_
    }
    // 获取set方法
    fn get_set_func(&self) -> *const c_void {
        self.set_fn_
    }
    // 获取hash
    fn get_hash(&self) -> u32 {
        self.hash_
    }
    // 设置名字
    fn set_name(&mut self, value: String) {
        self.name_ = value;
    }

    fn set_hash(&mut self, value: u32) {
        self.hash_ = value;
    }

    fn set_type(&mut self, value: i32) {
        self.type_ = value;
    }

    fn set_getfn(&mut self, value: *const c_void) {
        self.get_fn_ = value;
    }

    fn set_setfn(&mut self, value: *const c_void) {
        self.set_fn_ = value;
    }
}

// 实体方法信息
#[derive(Debug)]
pub struct IFuncInfo {
    name_: String,
    hash_: u32,
    mid_fn_: *const c_void,
    return_table_: bool,
}

impl IFuncInfo {
    // 获取名字
    fn new(name: String, hash: u32, mid_fn: *const c_void, return_table: bool) -> IFuncInfo {
        IFuncInfo {
            name_: name,
            hash_: hash,
            mid_fn_: mid_fn,
            return_table_: return_table,
        }
    }
    fn get_name(self) -> String {
        self.name_
    }
    // 获取修改方法
    fn get_mid_func(&self) -> *const c_void {
        self.mid_fn_
    }
    // 获取是否返回
    fn get_returnable(&self) -> bool {
        self.return_table_
    }

    fn get_hash(&self) -> u32 {
        self.hash_
    }

    fn set_name(&mut self, value: String) {
        self.name_ = value;
    }

    fn set_hash(&mut self, value: u32) {
        self.hash_ = value;
    }

    fn set_mid_fn(&mut self, value: *const c_void) {
        self.mid_fn_ = value;
    }

    fn set_return_table(&mut self, value: bool) {
        self.return_table_ = value;
    }
}

// 实体信息

#[derive(Debug)]
pub(crate) struct IEntityInfo {
    creator_: IEntityCreator,
    parent_name_: String,
    space_name_: String,
    entity_name_: String,
    parent_: Option<Box<IEntityInfo>>,
    prop_infos_: Vec<Box<IPropInfo>>,
    func_infos_: Vec<Box<IFuncInfo>>,
}

impl IEntityInfo {
    // 获取创建器
    fn get_creator(&self) -> &IEntityCreator {
        &self.creator_
    }

    // 获取父类名称
    fn get_parent_name(self) -> String {
        self.parent_name_
    }

    // 返回名字空间
    fn get_space_name(self) -> String {
        self.space_name_
    }

    // 返回类名
    fn get_entity_name(self) -> String {
        self.entity_name_
    }

    // 获取父类信息
    fn get_parent(self) -> Option<Box<IEntityInfo>> {
        self.parent_
    }

    // 是否属于某类或是自雷
    fn is_kind_of(&self, name: &str) -> bool {
        if name.eq(self.entity_name_.as_str()) {
            return true;
        }

        if name.eq(self.parent_name_.as_str()) {
            return true;
        }

        match self.parent_ {
            None => return false,
            IEntityInfo => return self.parent_.unwrap().is_kind_of(name),
        }
    }

    // 是否属于统一名字空间的某类或者是其子类
    fn is_kind_same_space(&self, entity: &IBaseEntity, name: &str) -> bool {
        if !entity
            .get_entity_info()
            .get_space_name()
            .eq(&self.space_name_)
        {
            return false;
        }

        self.is_kind_of(name)
    }

    // 实体属性
    fn get_property_count(&self) -> usize {
        self.prop_infos_.len()
    }

    // 获得属性名字列表
    pub fn get_property_list(&self, result: &mut IArrayList) {
        for prop in self.prop_infos_ {
            result.add_str(prop.get_name().as_str());
        }
    }

    // 在本类中获得属性信息
    pub fn get_property_info(&self, name: &str) -> Option<Box<IPropInfo>> {
        let index: usize = 0;
        if !self.find_property_index(name, index) {
            return None;
        }

        Some(self.prop_infos_[index as usize])
    }

    fn find_property_index(&self, name: &str, mut index: u32) -> bool {
        if name.is_empty() {
            assert!(false, "property name is empty!");
        }

        let hash = inlines::get_hash_value_case(name);

        let size = self.prop_infos_.len();
        for i in 0 .. size {
            if self.prop_infos_[i].get_hash() == hash {
                index = 1;
                return true;
            }
        }
        false
    }

    // 在本类和父类查找属性嘻嘻
    fn find_property_info(&self, name: &str) -> Option<Box<IPropInfo>> {
        let mut prop_info = self.get_property_info(name);

        if let Some(temp_prop) = prop_info {
            let mut entity_info = self.get_parent();

            while let Some(temp_entity) = entity_info {
                prop_info = temp_entity.get_property_info(name);

                match prop_info {
                    None => {entity_info = temp_entity.get_parent()},
                    IPropInfo => {
                        break;
                    },
                }
            }
        }

        prop_info
    }

    // 获得本类和父类的属性名字列表
    fn get_property_all(&self, result: &mut IArrayList) -> usize {
        result.clear();

        self.inner_get_property_list(result);

        let mut entity_info = self.get_parent();
        while let Some(temp_entity) = entity_info {
            temp_entity.inner_get_property_list(result);
            entity_info = temp_entity.get_parent();
        }

        result.get_count()
    }

    fn inner_get_property_list(&self, result: &mut IArrayList) {
        let size = self.prop_infos_.len();
        for i in 0 .. size {
            result.add_str(self.prop_infos_[i].get_name().as_str());
        }
    }
}
