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

pub(crate) trait IBaseEntity {
    // 初始化
    fn init(&self);

    // 关闭
    fn shut(&self);

    // 每帧执行逻辑
    fn execute(&self, seconds: f32);

    // 释放自身
    fn release(&self);

    // 获取内存占用
    fn get_memory_usage(&self) -> u32;

    //  获得核心接口
    fn get_core(&self) -> * dyn ICore;

    // 获得实体类信息
    fn get_entity_info(&self) -> * dyn IEntityInfo;

    // 获得绑定的脚本
    fn get_entity_script(&self) -> * dyn IEntityScript;

    // 获得对象ID
    fn get_entity_id(&self) -> ObjId;

    // 是否已删除
    fn get_deleted(&self) -> bool;

    // 是否可以被脚本删除
    fn get_can_del_by_script(&self) -> bool;

    fn set_core(&mut self, value: * dyn ICore);

    fn set_entity_info(&mut self, value: * dyn IEntityInfo);

    fn set_entity_script(&mut self, value: * dyn IEntityScript);

    fn set_entity_id(&mut self, value: ObjId);

    fn set_deleted(&mut self, value: bool);

    fn set_can_del_by_script(&mut self, value: bool);
}

struct BaseEntity {
    core_: * dyn ICore,
    entity_info_: * dyn IEntityInfo,
    entity_script_: *dyn IEntityScript,
    deleted_: bool,
    can_del_by_script_: bool,
    entity_id_: ObjId
}

impl IBaseEntity for BaseEntity {
    fn init(&self) {
        unimplemented!()
    }

    fn shut(&self) {
        unimplemented!()
    }

    fn execute(&self, seconds: f32) {}

    fn release(&self) {
        self.core_.delete_entity(self.get_entity_id());
    }

    fn get_memory_usage(&self) -> u32 {
        0
    }

    fn get_core(&self) -> *const dyn ICore {
        self.core_
    }

    fn get_entity_info(&self) -> *const dyn IEntityInfo {
        self.entity_info_
    }

    fn get_entity_script(&self) -> *const dyn IEntityScript {
        self.entity_script_
    }

    fn get_entity_id(&self) -> &ObjId {
        &self.entity_id_
    }

    fn get_deleted(&self) -> bool {
        self.deleted_
    }

    fn get_can_del_by_script(&self) -> bool {
        self.can_del_by_script_
    }

    fn set_core(&mut self, value: *const dyn ICore) {
        self.core_ = value;
    }

    fn set_entity_info(&mut self, value: *const dyn IEntityInfo) {
        self.entity_info_ = value;
    }

    fn set_entity_script(&mut self, value: *const dyn IEntityScript) {
        self.entity_script_ = value;
    }

    fn set_entity_id(&mut self, value: ObjId) {
        self.entity_id_ = value;
    }

    fn set_deleted(&mut self, value: bool) {
        self.deleted_ = value;
    }

    fn set_can_del_by_script(&mut self, value: bool) {
        self.can_del_by_script_ = value;
    }
}
