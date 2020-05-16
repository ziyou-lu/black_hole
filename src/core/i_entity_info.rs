/*************************************************
//  Copyright (C), 2020-2020, luwangda.
//  File name:       i_entity_info.rs
//  Author:        	 luwangda       
//  Version:         1.0     
//  Date:            2020/05/16
//  Description:     entity info interface define
//  Others:
//  History:    
*************************************************/

// 实体属性信息
trait i_prop_info {
    // 获取名字
    pub get_name() -> &str;
    // 获取类型
    pub get_type() -> i32;
    // 获取get方法
    pub get_getfunc() -> fn();
    // 获取set方法
    pub get_setfunc() -> fn();
}

// 实体方法信息
trait i_func_info {
    // 获取名字
    pub get_name() -> &str;
    // 获取修改方法
    pub get_midfunc() -> fn();
    // 获取是否返回
    pub get_returnable() -> bool;
}

// 实体信息
trait i_entity_info {

}