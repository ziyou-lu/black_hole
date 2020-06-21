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
use std::borrow::Borrow;

#[derive(Debug)]
pub(crate) struct IBaseLogic {
    entity_: Option<Box<IBaseEntity>>,
    logic_info_: Option<Box<IBaseLogicInfo>>,
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

    fn release(self) {
        match self.logic_info_ {
            None => {},
            Some(IBaseLogicInfo) => {
                self.logic_info_.unwrap().get_creator().unwrap().destroy(self.borrow());
            },
        }
        
    }

    fn get_entity(self) -> Option<Box<IBaseEntity>> {
        self.entity_
    }

    fn get_logic_info(self) -> Option<Box<IBaseLogicInfo>> {
        self.logic_info_
    }

    fn set_entity(&mut self, value: Option<Box<IBaseEntity>>) {
        self.entity_ = value;
    }

    fn set_logic_info(&mut self, value: Option<Box<IBaseLogicInfo>>) {
        self.logic_info_ = value;
    }
}
