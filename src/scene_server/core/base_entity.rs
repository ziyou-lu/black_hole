/*************************************************
//  Copyright (C), 2020-2020, luangada.
//  File name:       i_base_entity
//  Author:        	 luwangda
//  Version:         1.0
//  Date:            2020/05/16
//  Description:     base entity interface define
//  Others:
//  History:
*************************************************/
use super::core::ICore;
use super::entity_info::IEntityInfo;
use super::entity_script::IEntityScript;
use crate::share::obj_id::ObjId;

#[derive(Debug)]
pub(crate) struct IBaseEntity {
    core_: Box<ICore>,
    entity_info_: IEntityInfo,
    entity_script_: IEntityScript,
    deleted_: bool,
    can_del_by_script_: bool,
    entity_id_: ObjId,
}

impl IBaseEntity {
    pub fn init(&self) {
        unimplemented!()
    }

    pub fn shut(&self) {
        unimplemented!()
    }

    pub fn execute(&self, seconds: f32) {
        unimplemented!()
    }

    pub fn release(&self) {
        self.core_.delete_entity(&self.entity_id_);
    }

    pub fn get_memory_usage(&self) -> u32 {
        0
    }

    pub fn get_core(self) -> Box<ICore> {
        self.core_
    }

    pub fn get_entity_info(&self) -> &IEntityInfo {
        &self.entity_info_
    }

    pub fn get_entity_id(&self) -> ObjId {
        self.entity_id_
    }

    pub fn get_deleted(&self) -> bool {
        self.deleted_
    }

    pub fn get_can_del_by_script(&self) -> bool {
        self.can_del_by_script_
    }

    pub fn set_deleted(&mut self, value: bool) {
        self.deleted_ = value;
    }

    pub fn set_can_del_by_script(&mut self, value: bool) {
        self.can_del_by_script_ = value;
    }

    pub fn set_core(&mut self, value: Box<ICore>) {
        self.core_ = value;
    }

    pub fn set_entity_info(&mut self, value: IEntityInfo) {
        self.entity_info_ = value;
    }

    pub fn set_entity_script(&mut self, value: IEntityScript) {
        self.entity_script_ = value;
    }

    pub fn set_entity_id(&mut self, value: ObjId) {
        self.entity_id_ = value;
    }
}
