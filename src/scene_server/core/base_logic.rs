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
use super::base_entity::IBaseEntity;
use super::base_logic_info::IBaseLogicInfo;
use crate::share::any_list::IArrayList;

#[derive(Debug)]
pub(crate) struct IBaseLogic {
    entity_: IBaseEntity,
    logic_info_: IBaseLogicInfo,
}

impl IBaseLogic {
    // 初始化
    fn init(&self, args: IArrayList) -> bool {
        true
    }
    // 关闭
    fn shut(&self) -> bool {
        true
    }

    fn release(&self) {
        self.logic_info_.get_creator().destroy(self);
    }

    fn get_entity(&self) -> Box<IBaseEntity> {
        Box::new(self.entity_)
    }

    fn get_logic_info(&self) -> Box<IBaseLogicInfo> {
        Box::new(self.logic_info_)
    }

    fn set_entity(&mut self, value: Box<IBaseEntity>) {
        self.entity_ = value;
    }

    fn set_logic_info(&mut self, value: Box<IBaseLogicInfo>) {
        self.logic_info_ = value;
    }
}
