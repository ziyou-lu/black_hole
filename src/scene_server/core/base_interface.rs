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
}

struct BaseInterface<A, B>
where
    A: ICore,
    B: IBaseInterfaceCreator<Self>,
{
    pub core_: A,
    pub creator_: B,
}

impl<A, B> IBaseInterface for BaseInterface<A, B>
where
    A: ICore,
    B: IBaseInterfaceCreator<Self>,
{
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
}
