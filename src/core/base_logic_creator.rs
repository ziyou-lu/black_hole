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
pub struct BaseLogicCallBack {
    name_: String,
    mid_func_: fn(),
    return_table_: bool,
    pub next_: Some(BaseLogicCallBack),
}

pub(crate) trait IBaseLogicCreator {
    // 返回空间名字
    fn get_space(&self) -> String;
    // 返回名称
    fn get_name(&self) -> String;
    // 创建
    fn create<T: IBaseLogic>(&self) -> Option<T>;
    // 删除
    fn destroy<T: IBaseLogic>(&self, logic: &T);
    // 获得下一个
    fn get_next<T: IBaseLogicCreator>(&self) -> Option<T>;
    // 获取方法链表
    fn get_callback_link(&self) -> Option<BaseLogicCallBack>;

    // 设置方法链表
    fn set_callback_link(&mut self, value: BaseLogicCallBack);


}

struct BaseLogicCreator {
    next_: Some(dyn IBaseLogicCreator),
    call_back_: Some(BaseLogicCallBack),
}

impl IBaseLogicCreator for BaseLogicCreator {
    fn get_space(&self) -> String {
        unimplemented!()
    }

    fn get_name(&self) -> String {
        unimplemented!()
    }

    fn create<T: IBaseLogic>(&self) -> Option<T> {
        unimplemented!()
    }

    fn destroy<T: IBaseLogic>(&self, logic: &T) {
        unimplemented!()
    }

    fn get_next<T: IBaseLogicCreator>(&self) -> Option<T> {
        self.next_
    }

    fn get_callback_link(&self) -> Option<BaseLogicCallBack> {
        self.call_back_
    }

    fn set_callback_link(&mut self, value: BaseLogicCallBack) {
        self.call_back_ = value;
    }
}