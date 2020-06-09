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
    pub next_: *IBaseLogicCallBack,
}

#[derive(Debug)]
pub(crate) struct IBaseLogicCreator {
    next_: *IBaseLogicCreator,
    call_back_: *IBaseLogicCallBack,
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

    fn get_next<T: IBaseLogicCreator>(&self) -> const* IBaseLogicCallBack {
        self.next_
    }

    fn get_callback_link(&self) -> Option<BaseLogicCallBack> {
        self.call_back_
    }

    fn set_callback_link(&mut self, value: BaseLogicCallBack) {
        self.call_back_ = value;
    }

}
