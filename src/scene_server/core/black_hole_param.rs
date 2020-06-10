/*************************************************
//  Copyright (C), 2020-2020, luwangda.
//  File name:       black_hole_param.h
//  Author:        	 luwangda
//  Version:         1.0
//  Date:            2020/05/18
//  Description:     黑洞参数（一些引擎内定义和常量）
//  Others:
//  History
*************************************************/
use super::gravity::IGravity;
use crate::share::any_list::IArrayList;
use crate::share::obj_id::ObjId;

/***
 ************** 函数定义
 */
pub type LogicClassFunc = fn(Box<IGravity>, i32, &str) -> i32;
pub type LogicEventFunc = fn(Box<IGravity>, &ObjId, &ObjId, &IArrayList);
