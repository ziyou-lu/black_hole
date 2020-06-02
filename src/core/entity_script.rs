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
use crate::share::any_list::IArrayList;
use super::base_logic::IBaseLogic;

trait IEntityScript {
    // 获得脚本名称
    fn get_name() -> str;

    // 获得逻辑类
    fn get_logic() -> * dyn IBaseLogic;
}
