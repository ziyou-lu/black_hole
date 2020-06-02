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
use core;
use base_logic_creator;

trait IBaseInterface {

    // 初始化
    fn init() -> bool;
    // 关闭
    fn shut() -> bool;

    //是否需要每帧运行
    fn need_exec_perframe() -> bool {
        return false;
    }

    // 每帧开始时调用
    fn exec_frame_begin();
    // 每帧结束时调用
    fn exec_frame_end();

    // 释放
    fn release();

    // 获得内存占用
    fn get_memory_usage() -> u32 {
        return 0;
    }

    // 获取核心接口
    fn get_core() -> *i_core {
        return core_;
    }

    // 获取创建器
    fn get_interface_creator() -> *i_base_interface_creator;

    fn set_core(value: *i_core) {
        core_ = value;
    }

    fn set_interface_creator(value: *i_base_interface_creator) {
        creator_ = value;
    }
}

struct IBase {
    core_: *i_core,
    creator_: *i_base_interface_creator,
}

impl IBaseInterface for IBase {
    fn init() -> bool {
        unimplemented!()
    }

    fn shut() -> bool {
        unimplemented!()
    }

    fn exec_frame_begin() {
        unimplemented!()
    }

    fn exec_frame_end() {
        unimplemented!()
    }

    fn release() {
        unimplemented!()
    }

    fn get_interface_creator() -> *const _ {
        unimplemented!()
    }
}



