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

#[derive(Debug)]
pub(crate) struct IBaseInterfaceCreator {
    next_: Box<IBaseInterfaceCreator>,
}

impl IBaseInterfaceCreator {
    // 返回名字空间
    fn get_space(&self) -> String {
        String::from("")
    }

    // 返回名称
    fn get_name(&self) -> String {
        String::from("")
    }

    // 创建
    fn create(&self) -> Box<IBaseInterface> {}

    // 删除
    fn destroy(&self, interface: IBaseInterface) {}

    // 获得下一个
    fn get_next(&self) -> Box<IBaseInterfaceCreator> {
        self.next_
    }
}
