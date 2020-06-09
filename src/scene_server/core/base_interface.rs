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
use super::base_interface_creator::IBaseInterfaceCreator;
use super::core::ICore;

#[derive(Debug)]
pub(crate) struct IBaseInterface {
    core_: ICore,
    creator_: IBaseInterfaceCreator,
}

impl IBaseInterface {
    // 初始化
    fn init(&self) -> bool{
        true
    }
    // 关闭
    fn shut(&self) -> bool {
        true
    }

    //是否需要每帧运行
    fn need_exec_per_frame(&self) -> bool {
        false
    }

    // 每帧开始时调用
    fn exec_frame_begin(&self) {}
    // 每帧结束时调用
    fn exec_frame_end(&self) {}

    // 释放
    fn release(&self) {}

    // 获得内存占用
    fn get_memory_usage(&self) -> u32 {
        0
    }
}

