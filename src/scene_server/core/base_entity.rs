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
use crate::core::core::ICore;
use crate::core::entity_info::IEntityInfo;
use crate::core::entity_script::IEntityScript;
use crate::share::obj_id::ObjId;

#[derive(Debug)]
pub(crate) struct IBaseEntity {
    core_: A,
    entity_info_: B,
    entity_script_: C,
    deleted_: bool,
    can_del_by_script_: bool,
    entity_id_: ObjId,
}

impl BaseEntity {
    fn init(&self) {
        unimplemented!()
    }

    fn shut(&self) {
        unimplemented!()
    }

    fn execute(&self, seconds: f32) {}

    fn release(&self) {
        self.core_.delete_entity(&self.entity_id_);
    }

    fn get_memory_usage(&self) -> u32 {
        0
    }

    fn get_deleted(&self) -> bool {
        self.deleted_
    }

    fn get_can_del_by_script(&self) -> bool {
        self.can_del_by_script_
    }

    fn set_deleted(&mut self, value: bool) {
        self.deleted_ = value;
    }

    fn set_can_del_by_script(&mut self, value: bool) {
        self.can_del_by_script_ = value;
    }
}
