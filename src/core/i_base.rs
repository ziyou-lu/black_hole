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
use ci;
use bi_creator

trait i_base {

    // 初始化
    pub init() -> bool;
    // 关闭
    pub shut() -> bool;

    //是否需要每帧运行
    pub need_exec_perframe() -> bool {
        return false;
    }

    // 每帧开始时调用
    pub exec_frame_begin();
    // 每帧结束时调用
    pub exec_frame_end();

    // 释放
    pub void release() {
        ci_
    }

    ci_ *i_core;
    creator_ *bi_creator; 
}
