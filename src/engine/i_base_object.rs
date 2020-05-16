/*************************************************
//  Copyright (C), 2017-2018, luwangda.
//  File name:       i_base_object.h
//  Author:        	 luwangda
//  Version:         1.0
//  Date:            2020/05/18
//  Description:     游戏对象操作接口
//  Others:
//  History:
*************************************************/
use obj_id;
trait i_base_object {

    //////////////////////////////////////////////////////////////////////////
    // 对象配置
    
    // 获取对象类型
    pub get_obj_type() -> i32;

    // 获取对象号
    pub get_object_id() -> obj_id;

    // 获取对象脚本名
    pub get_script() -> str;

    // 获取对象配置名
    pub get_config() -> i64;

    // 获取名字
    pub get_name() -> str;

    // 获取组号
    pub get_group_id() -> i32;

    //////////////////////////////////////////////////////////////////////////
    // 对象层次结构
    
    // 获取对象在容器中的位置
    pub get_index_in_container() i32;

    // 获取父对象
    pub get_parent_obj() -> *i_base_object;
}
