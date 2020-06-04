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
struct BaseLogicCallBack {
    name_: String,
    mid_func_: fn(),
    return_table_: bool,
    next_: *i_base_logic_call_back,
}

pub(crate) trait IBaseLogicCreator {
    // 返回空间名字
    fn get_space(&self) -> String;
    // 返回名称
    fn get_name(&self) -> String;
    // 创建
    fn create(&self) -> * dyn IBaseLogic;
    // 删除
    fn destroy(&self, p: * dyn IBaseLogic);
    // 获得下一个
    fn get_next(&self) -> * dyn IBaseLogicCreator;
    // 获取方法链表
    fn get_callback_link(&self) -> * BaseLogicCallBack;

    // 设置方法链表
    fn set_callback_link(&self, value: * BaseLogicCallBack);


}

struct BaseLogicCreator {
    next_: * dyn IBaseLogicCreator,
    call_back_: *BaseLogicCallBack,
}

impl IBaseLogicCreator for BaseLogicCreator {
    fn get_space(&self) -> String {
        unimplemented!()
    }

    fn get_name(&self) -> String {
        unimplemented!()
    }

    fn create(&self) -> *const dyn IBaseLogic {
        unimplemented!()
    }

    fn destroy(&mut self, value: *const dyn IBaseLogicCreator) {
        self.next_ = value;
    }

    fn get_next(&self) -> *const dyn IBaseLogicCreator {
        self.next_
    }

    fn get_callback_link(&self) -> *const BaseLogicCallBack {
        self.call_back_
    }

    fn set_callback_link(&mut self, value: *const BaseLogicCallBack) {
        self.call_back_ = value;
    }
}