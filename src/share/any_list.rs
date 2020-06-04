/*************************************************
//  Copyright (C), 2020-2020, luwangda.
//  File name:       i_any_list.h
//  Author:        	 luwangda
//  Version:         1.0
//  Date:            2020/05/16
//  Description:     array list
//  Others:          可变类型列表
//  History:
*************************************************/
use std::ops::Shl;

pub(crate) trait IArrayList {
    // 合并列表
    fn combine(&self, src: &dyn IArrayList) -> bool;

    // 附加列表
    fn append(&self, src: &dyn IArrayList, start: u32, count: u32);

    // 清空
    fn clear(&self);

    // 是否为空
    fn is_empty(&self) -> bool;

    // 数据长度
    fn get_count(&self) -> u32;

    // 数据类型
    fn get_type(&self, index: &u32) -> i32;

    // 获取内存用量
    fn get_memory_usage(&self) -> u32;


}

trait IArrayListOp<T> {
    fn add_value<T>(&self, value: T) ->bool;

    fn set_value<T>(&self, value: T) -> bool;

    fn get_value<T>(&self) -> Option<T>;

    fn shl<T>(&self, value: T) {
        self.add_value(value);
    }
}

impl IArrayListOp<T> for dyn IArrayList {
    fn add_value<T>(&self, value: T) -> bool {
        unimplemented!()
    }

    fn set_value<T>(&self, value: T) -> bool {
        unimplemented!()
    }

    fn get_value<T>(&self) -> Option<T> {
        unimplemented!()
    }

    fn shl<T>(&self, value: T) {
        unimplemented!()
    }
}



