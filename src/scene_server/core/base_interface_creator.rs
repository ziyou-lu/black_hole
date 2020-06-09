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
use crate::core::base_interface::IBaseInterface;

#[derive(Debug)]
pub(crate) struct IBaseInterfaceCreator
{
    next_: *IBaseInterfaceCreator,
}

impl BaseInterfaceCreator {

    // 返回名字空间
    fn get_space(&self) -> String {
        ""
    }

    // 返回名称
    fn get_name(&self) -> String;

    // 创建
    fn create(&self) -> Box<IBaseInterface>;

    // 删除
    fn destroy(&self, interface: T);

    // 获得下一个
    fn get_next(&self) -> *IBaseInterfaceCreator;

}

