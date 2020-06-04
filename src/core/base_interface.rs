/*************************************************
//  Copyright (C), 2020-2020 luwangda.
//  File name:       ci.h
//  Author:        	 luwangda
//  Version:         1.0
//  Date:            2020/05/16
//  Description:     base interface
//  Others:
//  History:
*************************************************/
use super::core::ICore;
use super::base_interface_creator::IBaseInterfaceCreator;

pub(crate) trait IBaseInterface {

    // 初始化
    fn init(&self) -> bool;
    // 关闭
    fn shut(&self) -> bool;

    //是否需要每帧运行
    fn need_exec_per_frame(&self) -> bool {
        return false;
    }

    // 每帧开始时调用
    fn exec_frame_begin(&self);
    // 每帧结束时调用
    fn exec_frame_end(&self);

    // 释放
    fn release(&self);

    // 获得内存占用
    fn get_memory_usage(&self) -> u32 {
        return 0;
    }

    // 获取核心接口
    fn get_core(&self) -> *dyn ICore;

    // 获取创建器
    fn get_interface_creator(&self) -> *I;

    fn set_core(&self, value: *dyn ICore);

    fn set_interface_creator(&self, value: *dyn IBaseInterfaceCreator);
}

struct IBase {
    core_: * dyn ICore,
    creator_: * dyn IBaseInterfaceCreator,
}

impl IBaseInterface for IBase {
    fn init(&self) -> bool {
        unimplemented!()
    }

    fn shut(&self) -> bool {
        unimplemented!()
    }

    fn exec_frame_begin(&self) {
        unimplemented!()
    }

    fn exec_frame_end(&self) {
        unimplemented!()
    }

    fn release(&self) {
        unimplemented!()
    }

    fn get_core(&self) -> *const dyn ICore {
        self.core_
    }

    fn get_interface_creator(&self) -> *const dyn IBaseInterfaceCreator {
        self.creator_
    }

    fn set_core(&mut self, value: *const dyn ICore) {
        self.core_ = value;
    }

    fn set_interface_creator(&mut self, value: *const dyn IBaseInterfaceCreator) {
        self.creator_ = value;
    }
}



