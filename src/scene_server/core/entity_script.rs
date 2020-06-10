/*************************************************
//  Copyright (C), 2020-2020, luwangda.
//  File name:       IEntityScript
//  Author:        	 luwangda
//  Version:         1.0
//  Date:            2020/05/16
//  Description:     entity script interface define
//  Others:
//  History:
*************************************************/
use super::base_logic::IBaseLogic;

#[derive(Debug)]
pub(crate) struct IEntityScript {}

impl IEntityScript {
    // 获得脚本名称
    fn get_name(&self) -> String {}

    // 获得逻辑类
    fn get_logic(&self) -> Box<IBaseLogic> {}

    // 添加回调
    fn add_callback(&self, event: &str, func: &str) -> bool {}

    // 删除回调
    fn remove_callback(&self, event: &str) -> bool {}

    // 清空回调
    fn clear_callback(&self) {}

    // 获取回调数量
    fn get_callback_count(&self) -> u32 {}

    // 获取回调对应的脚本函数
    fn get_callback_func(&self, event: &str) -> String {}
}
