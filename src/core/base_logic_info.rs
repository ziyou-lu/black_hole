/*************************************************
//  Copyright (C), 2020-2020, luwangda.
//  File name:       IBaseLogicInfo.h
//  Author:        	 luwangda
//  Version:         1.0
//  Date:            2020/05/16
//  Description:     base logic info interface define
//  Others:
//  History:
*************************************************/
use super::base_logic_creator::IBaseLogicCreator;
use crate::share::any_list::IArrayList;

// 逻辑方法信息
#[derive(Debug)]
trait ICallBackInfo {
    fn get_name() -> str;
    fn get_nid_func() -> fn();
    fn get_return_table() -> bool;
}

pub(crate) trait IBaseLogicInfo {
    // 获得创建起
    fn get_creator(&self) -> * dyn IBaseLogicCreator;
    
    // 返回名字空间
    fn get_space_name() -> String;

    // 返回类名
    fn get_logic_name() -> String;

    // 方法数量
    fn get_callback_count() -> u32;

    // 获取方法名字列表
    fn get_callback_list(result: &dyn IArrayList) -> u32;

    // 在本类中获取方法信息
    fn get_callback_info(name: &str) -> * dyn ICallBackInfo;
}
