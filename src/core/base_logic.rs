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
    fn init(&self, args: &dyn IArrayList) -> bool {
        true
    }
    // 关闭
    fn shut(&self) -> bool {
        true
    }

    // 释放自身
    fn release(&self);

    // 获得板顶的对象
    fn get_entity(&self) -> Box<dyn IBaseEntity>;

    // 获得逻辑类信息
    fn get_logic_info(&self) -> Box<dyn IBaseLogicInfo>;

    fn set_entity(&self, value: Box<dyn IBaseEntity>);

    fn set_logic_info(&self, value: Box<dyn IBaseLogicInfo>);
}

struct BaseLogic {
    entity_: impl IBaseEntity,
    logic_info_: Box<dyn IBaseLogicInfo>,
}

impl IBaseLogic for BaseLogic {
    fn release(&self) {
        self.logic_info_.get_creator().destroy(self);
    }

    fn get_entity(&self) -> Box<dyn IBaseEntity> {
        *self.entity_
    }

    fn get_logic_info(&self) -> Box<dyn IBaseLogicInfo> {
        &self.logic_info_
    }

    fn set_entity(&mut self, value: Box<dyn IBaseEntity>) {
        self.entity_ = value;
    }

    fn set_logic_info(&mut self, value: Box<dyn IBaseLogicInfo>) {
        self.logic_info_ = value;
    }
}
