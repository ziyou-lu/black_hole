/*************************************************
//  Copyright (C), 2020-2020, luwangda.
//  File name:       bi_creator.rs
//  Author:        	 luwangda
//  Version:         1.0
//  Date:            2020/05/16
//  Description:     base interface creator define
//  Others:
//  History:
*************************************************/
use i_base;

trait i_base_creator {
    // 返回空间名字
    pub get_space() -> &str;
    // 返回名称
    pub get_name() -> &str;
    // 创建
    pub create() -> *bi;
    // 删除
    pub destroy(p: *bi);
    // 获得下一个
    pub get_next() -> *i_base_creator {
        return next_;
    }

    next_ *i_base_creator;
}
