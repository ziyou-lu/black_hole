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

pub(crate) trait IBaseInterfaceCreator<T>
where
    T: IBaseInterface,
{
    // 返回名字空间
    fn get_space(&self) -> String;

    // 返回名称
    fn get_name(&self) -> String;

    // 创建
    fn create(&self) -> T;

    // 删除
    fn destroy(&self, interface: T);

    // 获得下一个
    fn get_next(&self) -> &T;
}

struct BaseInterfaceCreator {
    next_: BaseInterfaceCreator,
}

impl<T> IBaseInterfaceCreator<T> for BaseInterfaceCreator
where
    T: IBaseInterface,
{
    fn get_space(&self) -> String {
        unimplemented!()
    }

    fn get_name(&self) -> String {
        unimplemented!()
    }

    fn create(&self) -> T {
        unimplemented!()
    }

    fn destroy(&self, interface: T) {
        unimplemented!()
    }

    fn get_next(&self) -> &BaseInterfaceCreator {
        &self.next_
    }
}
