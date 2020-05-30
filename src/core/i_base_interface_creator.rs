/*************************************************
//  Copyright (C), 2020-2020, luwangda.
//  File name:      i_base_interface_creator.rs
//  Author:        	 luwangda
//  Version:         1.0
//  Date:            2020/05/19
//  Description:     base interface creator define
//  Others:
//  History:
*************************************************/
use i_base_interface;

trait i_base_interface_creator {

    // 返回名字空间
    pub fn get_space() -> str;

    // 返回名称
    pub fn get_name() -> str;

    // 创建
    pub fn create() -> *i_base_interface;

    // 删除
    pub fn destroy(p: *i_base_interface);

    // 获得下一个
    pub fn get_next() -> *i_base_interface;

    next_: *i_base_interface_creator;
}
