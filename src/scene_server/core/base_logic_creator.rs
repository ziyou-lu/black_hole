/*************************************************
//  Copyright (C), 2020-2020, luwangda.
//  File name:       i_base_creator.rs
//  Author:        	 luwangda
//  Version:         1.0
//  Date:            2020/05/16
//  Description:     base interface creator define
//  Others:
//  History:
*************************************************/
use super::base_logic::IBaseLogic;

#[derive(Debug)]
pub struct IBaseLogicCallBack {
    pub name_: String,
    pub mid_func_: fn(),
    pub return_table_: bool,
    pub next_: Option<Box<IBaseLogicCallBack>>,
}

#[derive(Debug)]
pub(crate) struct IBaseLogicCreator {
    next_: Option<Box<IBaseLogicCreator>>,
    call_back_: Option<Box<IBaseLogicCallBack>>,
}

impl IBaseLogicCreator {
    fn get_space(&self) -> String {
        unimplemented!()
    }

    fn get_name(&self) -> String {
        unimplemented!()
    }

    fn create(&self) -> Box<IBaseLogic> {
        unimplemented!()
    }

    fn destroy(&self, logic: &IBaseLogic) {
        unimplemented!()
    }

    fn get_next(&self) -> Option<Box<IBaseLogicCreator>> {
        self.next_
    }

    fn get_callback_link(&self) -> Option<Box<IBaseLogicCallBack>> {
        self.call_back_
    }

    fn set_callback_link(&mut self, value: Option<Box<IBaseLogicCallBack>>) {
        self.call_back_ = value;
    }
}
