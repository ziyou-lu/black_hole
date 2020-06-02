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
use base_interface;

trait IBaseInterfaceCreator {

    // 返回名字空间
    fn get_space() -> String;

    // 返回名称
    fn get_name() -> String;

    // 创建
    fn create() -> *IBaseInterface;

    // 删除
    fn destroy(p: *IBaseInterface);

    // 获得下一个
    fn get_next() -> *IBaseInterface;


}

struct BaseInterfaceCreator {
    next_: *BaseInterfaceCreator,
}

impl IBaseInterfaceCreator for BaseInterfaceCreator {
    fn get_space() -> String {
        unimplemented!()
    }

    fn get_name() -> String {
        unimplemented!()
    }

    fn create() -> *IBaseInterface {
        unimplemented!()
    }

    fn destroy(p: *IBaseInterface) {
        unimplemented!()
    }

    fn get_next() -> *IBaseInterface {
        unimplemented!()
    }
}