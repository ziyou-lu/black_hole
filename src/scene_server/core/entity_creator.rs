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
use super::base_entity::IBaseEntity;

#[derive(Debug)]
struct EntityProp {
    name_: String,
    type_: i32,
    get_func_: fn(),
    set_func_: fn(),
    next_: Option<Box<EntityProp>>,
}

#[derive(Debug)]
struct EntityFunc {
    name_: String,
    mid_func_: fn(),
    return_table_: bool,
    next_: Option<Box<EntityFunc>>,
}

#[derive(Debug)]
pub(crate) struct IEntityCreator {
    next_: Box<IEntityCreator>,
    property_: Box<EntityProp>,
    method_: Option<Box<EntityFunc>>,
}

impl IEntityCreator {
    // 是否为纯虚类（只能用来继承）
    fn is_abstract(&self) -> bool {
        false
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

    fn destroy(&self, p: &IBaseEntity) {
        unimplemented!()
    }

    fn get_next(self) -> Box<IEntityCreator> {
        self.next_
    }

    fn get_property_link(&self) -> &EntityProp {
        &self.property_
    }

    fn set_property_link(&mut self, value: Box<EntityProp>) {
        self.property_ = value;
    }

    fn get_method_link(self) -> Option<Box<EntityFunc>> {
        self.method_
    }

    fn set_method_link(&mut self, value: Option<Box<EntityFunc>>) {
        self.method_ = value;
    }
}
