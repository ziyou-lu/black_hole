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
struct IBaseLogicCallBack {
    name_: str,
    mid_func_: fn(),
    return_table_: bool,
    next_: *i_base_logic_call_back,
}

pub(crate) trait IBaseLogicCreator {
    // 返回空间名字
    fn get_space() -> &str;
    // 返回名称
    fn get_name() -> &str;
    // 创建
    fn create() -> * dyn IBaseLogic;
    // 删除
    fn destroy(&self, p: * dyn IBaseLogic);
    // 获得下一个
    fn get_next() -> * dyn IBaseLogicCreator {
        return next_;
    }
    // 获取方法链表
    fn get_callback_link() -> * IBaseLogicCallBack {
        return call_back_;
    }

    // 设置方法链表
    fn set_callback_link(value: * IBaseLogicCallBack) {
        call_back_ = value;
    }


}

struct BaseLogicCreator {
    next_: * dyn IBaseLogicCreator,
    call_back_: *IBaseLogicCallBack,
}

impl IBaseLogicCreator for BaseLogicCreator {
    fn get_space() -> &str {
        unimplemented!()
    }

    fn get_name() -> &str {
        unimplemented!()
    }

    fn create() -> *const dyn IBaseLogic {
        unimplemented!()
    }

    fn destroy(&self, p: *const dyn IBaseLogic) {
        unimplemented!()
    }
}