/*************************************************
//  Copyright (C), 2020-2020, luwangda.
//  File name:      IBaseInterfaceCreator
//  Author:        	 luwangda
//  Version:         1.0
//  Date:            2020/05/19
//  Description:     base interface creator define
//  Others:
//  History:
*************************************************/
use super::base_interface::IBaseInterface;
use std::mem::drop;

#[derive(Debug)]
pub(crate) struct IBaseInterfaceCreator {
    next_: Option<Box<Self>>,
    space_name_: String,
    name_: String,
    Ratio: f32,
}


impl IBaseInterfaceCreator {
    // 返回名字空间
    fn get_space(&self) -> String {
        self.space_name_
    }

    // 返回名称
    fn get_name(&self) -> String {
        self.name_
    }

    // 删除
    fn destroy(&self, interface: Box<impl IBaseInterface>) {
        drop(interface);
    }

    // 获得下一个
    fn get_next(&self) -> Option<Box<Self>> {
        self.next_
    }
}
