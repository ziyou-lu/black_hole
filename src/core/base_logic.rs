/*************************************************
//  Copyright (C), 2020-2020, luwangda.
//  File name:       IBaseLogic
//  Author:        	 luwangda
//  Version:         1.0
//  Date:            2020/05/16
//  Description:     base logic interface define
//  Others:
//  History:
*************************************************/
use crate::share::any_list::IArrayList;
use crate::core::base_entity::IBaseEntity;
use crate::core::base_logic_info::IBaseLogicInfo;

pub(crate) trait IBaseLogic {
    // 初始化
    fn init(args: &dyn IArrayList) -> bool {
        true
    }
    // 关闭
    fn shut() -> bool {
        true
    }

    // 释放自身
    fn release(&self);

    // 获得板顶的对象
    fn get_entity(&self) -> * dyn IBaseEntity;

    // 获得逻辑类信息
    fn get_logic_info() -> * dyn IBaseLogicInfo;

    fn set_entity(&self, value: * dyn IBaseEntity);

    fn set_logic_info(&self, value: * dyn IBaseLogicInfo);
}

struct BaseLogic {
    entity_: * dyn IBaseEntity,
    logic_info_: * dyn IBaseLogicInfo,
}

impl IBaseLogic for BaseLogic {
    fn release(&self) {
        self.logic_info_.get_creator().destroy(self);
    }

    fn get_entity(&self) -> *const dyn IBaseEntity {
        unimplemented!()
    }

    fn get_logic_info() -> *const dyn IBaseLogicInfo {
        unimplemented!()
    }

    fn set_entity(&self, value: *const dyn IBaseEntity) {
        unimplemented!()
    }

    fn set_logic_info(&self, value: *const dyn IBaseLogicInfo) {
        unimplemented!()
    }
}
