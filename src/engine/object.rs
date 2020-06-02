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
use share::*;
use record;


trait IObject {

    //////////////////////////////////////////////////////////////////////////
    // 对象配置
    
    // 获取对象类型
    fn get_obj_type() -> i32;

    // 获取对象号
    fn get_object_id() -> ObjId;

    // 获取对象脚本名
    fn get_script() -> str;

    // 获取对象配置名
    fn get_config() -> i64;

    // 获取名字
    fn get_name() -> str;

    // 获取组号
    fn get_group_id() -> i32;

    //////////////////////////////////////////////////////////////////////////
    // 对象层次结构
    
    // 获取对象在容器中的位置
    fn get_index_in_container() -> i32;

    // 获取父对象
    fn get_parent_obj() -> *IObject;

    // 返回容器容量
    fn get_container_capacity() -> i32;

    // 获取第一个子对象
    fn get_first_child_obj(it: &u32) -> ObjId;

    // 获取下一个子对象
    fn get_next_child_obj(it: &u32) -> ObjId;

    // 返回子对象数量
    fn get_child_obj_number() -> i32;

    // 获得指定位置子对象
    fn get_child_obj_by_index(index: i32) -> *IObject;

    // 根据名字获得子对象
    fn get_child_obj_by_name(name: &str) -> *IObject;

    // 获得子对象列表
    fn get_child_obj_id_list(class_type: i32, result: &IArrayList);

    // 查找子对象
    fn search_child_obj_by_name(name: &str, class_type: i32) -> obj_id;

    // 查找多个子对象
    fn search_more_child_obj_by_name(name: &str, class_type: i32, result: &i_array_list);

    // 使用配置名查找子对象
    fn search_child_obj_by_config(config: i64, class_type: i32) -> obj_id;

    // 使用配置名查找多个子对象
    fn search_more_child_obj_by_config(config: i64, class_type: i32, result: &i_array_list);

    // 清空子对象
    fn clear_child_obj();

    // 添加进弱关联容器
    fn add_to_container(container: &obj_id, pos: i32) -> Result;

    // 从弱关联容器中移除对象
    fn remove_from_container(container: &obj_id) -> Result;

    // 清空关联容器引用对象
    fn clear_container_child() -> Result;

    // 获得被弱关联的次数
    fn get_container_refs() -> i32;

    // 获取容器名字列表
    fn get_container_list(result: &i_array_list);

    // 对象坐标X
    fn get_pos_x() -> f32;

    // 对象坐标Y
    fn get_pos_y() -> f32;

    // 对象坐标Z
    fn get_pos_z() -> f32;

    // 对象方向
    fn get_orient() -> f32;

    ////////////////////////////////////////////////////////////////////////////
    /// 属性相关操作
    /// @brief 对象是否需要保存到db
	fn set_obj_un_save(value: bool);
	/// @brief 对象是否需要保存到db
	fn get_obj_un_save() -> bool;
	/// @brief 测试是否有一个属性
	// \param index_ptr 属性index，不存在会被置-1
	fn is_attr_exist(name: &str, index_ptr: *i32) -> bool;
	/// @brief 属性是否可视
	fn is_attr_visible(name: &str) -> bool;
	/// @brief 属性是否公共可视
	fn is_attr_pub_visible(name: &str) -> bool;
	/// @brief 可视属性是否即时刷新
	fn is_attr_realtime(name: &str) -> bool;
	/// @brief 属性是否保存
	fn is_attr_saving(name: &str) -> bool;
	/// @brief 可视属性是否隐藏
	fn is_attr_hide(name: &str) -> bool;
	/// @brief 设置可视属性是否隐藏
	fn set_attribute_hide(name: &str, value: bool) -> bool;
	/// @brief 增加(减少)属性值
	fn inc_i32(name: &str, value: i32) -> bool;
	fn inc_i32_index(index: i32, value: i32) -> bool;
	/// @brief 增加(减少)属性值
	fn inc_f32(name: &str, value: f32) -> bool;
	fn inc_f32_index( index: i32, value: f32) -> bool;
	/// @brief 用属性实现位标志操作（目的是节约存储，属性类型必须为整数）
	fn set_flag(name: &str, pos: i32) -> bool;
	/// @brief 清除标志
	fn clear_flag(name: &str, pos: i32) -> bool;
	/// @brief 测试标志
	fn test_flag(name: &str, pos: i32) -> bool;
	/// @brief 获得属性类型
	// \param index_ptr 属性index，不存在会被置-1
	fn get_attr_type(name: &str, index_ptr: *i32) -> i32;
	fn get_attr_type_index(index: i32) -> i32;
	/// @brief 获得属性数量
	fn get_attr_count() -> i32;
	/// @brief 获得属性名称列表
	fn get_attr_name_list(result: &i_array_list) -> i32;
	/// @brief 设置属性值
	/// \param name 属性名
	/// \param value 属性值
	fn set_i32(name: &str, value: i32) -> bool;
	fn set_i64(name: &str, value: i64) -> bool;
	fn set_f32(name: &str, value: f32) -> bool;
	fn set_f64(name: &str, value: f64) -> bool;
	fn set_str(name: &str, value: &str) -> bool;
	fn set_obj(name: &str, value: &obj_id) -> bool;
	/// @brief 查询属性
	/// \param name 属性名
	// \param index_ptr 属性index，不存在会被置-1
	fn get_i32(name: &str, index_ptr: *i32) -> i32;
	fn get_i64(name: &str, index_ptr: *i32) -> i64;
	fn get_f32(name: &str, index_ptr: *i32) -> f32;
	fn get_f64(name: &str, index_ptr: *i32) -> f64;
	fn get_str(name: &str, index_ptr: *i32) -> str;
	fn get_obj(name: &str, index_ptr: *i32) -> obj_id;

	/// @brief 获得属性的索引值（返回-1表示未找到）
	/// \param name 属性名
	fn get_attr_index(name: &str) -> i32;
	/// @brief 通过索引值设置属性值
	/// \param index 属性索引
	/// \param value 属性值
	fn set_i32_idx(index: i32, value: i32) -> bool;
	fn set_i64_idx(index: i32, value: i64) -> bool;
	fn set_f32_idx(index: i32, value: f32) -> bool;
	fn set_f64_idx(index: i32, value: f64) -> bool;
	fn set_str_idx(index: i32, value: &str) -> bool;
	fn set_obj_idx(index: i32, value: &ObjId) -> bool;
	/// @brief 通过索引值查询属性
	/// \param index 属性索引
	fn get_i32_idx(index: i32) -> i32;
	fn get_i64_idx(index: i32) -> i64;
	fn get_f32_idx(index: i32) -> f32;
	fn get_f64_idx(index: i32) -> f64;
	fn get_str_idx(index: i32) -> str;
	fn get_obj_idx(index: i32) -> obj_id;

	//////////////////////////////////////////////////////////////////////////
	// 属性回调相关
	/// @brief 查找关键属性回调是否存在
	fn attr_has_cb(property: &str, func: &str) -> bool;
	/// @brief 添加关键属性回调
	fn add_attr_cb(property: &str, func: &str, switch_carry: bool) -> bool;
	/// @brief 删除关键属性回调
	fn remove_attr_cb(property: &str) -> bool;
	/// @brief 删除指定函数名的关键属性回调
	fn remove_attr_cb_func(property: &str, func: &str) -> bool;

	//////////////////////////////////////////////////////////////////////////
	// 表格相关操作
	/// @brief 获得表数量
	fn get_record_count() -> i32;
	/// @brief 获得对象表格访问接口
	fn get_record_by_index(index: i32) -> *i_record;
	/// @brief 获得指定索引的表格访问接口
	fn get_record(name: &str) -> *i_record;
	/// @brief 表是否存在
	fn is_record_exist(name: &str) -> bool;
	/// @brief 获得表格的索引值（返回-1表示未找到）
	fn get_record_index(name: &str) -> i32;
	/// @brief 获得表名称列表
	fn get_record_name_list(result: &i_array_list) -> i32;
	/// @brief  获得表是否可视
	fn is_record_visible(name: &str) -> bool;
	/// @brief 获得表是否公共可视
	fn is_record_public(name: &str) -> bool;
	/// @brief 获得表是否保存
	fn is_record_saving(name: &str) -> bool;
	/// @brief 设置标志
	fn set_record_flag(name: &str, pos: i32) -> bool;
	/// @brief 清除标志
	fn clear_record_flag(name: &str, pos: i32) -> bool;
	/// @brief 测试标志
	fn test_record_flag( name: &str, pos: i32) -> bool;

	//////////////////////////////////////////////////////////////////////////
	// 表格操作回调
	/// @brief  查找表钩子回调是否存在
	fn record_has_cb(record: &str, func: &str) -> bool;
	/// @brief 添加表钩子回调
	fn add_record_cb(record: &str, func: &str, switch_carry: bool) -> bool;
	/// @brief 删除表钩子回调
	fn remove_record_cb(record: &str) -> bool;
	/// @brief 删除指定函数名的表钩子回调
	fn remove_record_cb_func(record: &str, func: &str) -> bool;

	//////////////////////////////////////////////////////////////////////////
	// 临时数据相关操作
	/// @brief 查找临时数据
	fn volatile_exist(name: &str) -> bool;
	/// @brief 获得临时数据数量
	fn get_volatile_count() -> i32;
	/// @brief 获得临时数据名称列表
	fn get_volatile_name_list(result: &IArrayList) -> i32;

	/// @brief 增加临时数据
	fn add_volatile(name: &str, datatype: i32, switch_carry: bool) -> bool;
	fn add_volatile_i32(name: &str, value: i32, switch_carry: bool) -> bool;
	fn add_volatile_i64(name: &str, value: i64, switch_carry: bool) -> bool;
	fn add_volatile_f32(name: &str, value: f32, switch_carry: bool) -> bool;
	fn add_volatile_f64(name: &str, value: f64, switch_carry: bool) -> bool;
	fn add_volatile_str(name: &str, value: &str, switch_carry: bool) -> bool;
	fn add_volatile_obj(name: &str, value: &ObjId, switch_carry: bool) -> bool;
	/// @brief 删除临时数据
	fn remove_volatile(name: &str) -> bool;
	/// @brief 获得临时数据类型
	fn get_volatile_type(name: &str) -> i32;

	/// @brief 设置临时数据
	fn set_volatile_i32(name: &str, value: i32) -> bool;
	fn set_volatile_i64(name: &str, value: i64) -> bool;
	fn set_volatile_f32(name: &str, value: f32) -> bool;
	fn set_volatile_f64(name: &str, value: f64) -> bool;
	fn set_volatile_str(name: &str, value: &str) -> bool;
	fn set_volatile_obj(name: &str, value: &ObjId) -> bool;

	/// @brief 查询临时数据
	fn get_volatile_i32(name: &str) -> i32;
	fn get_volatile_i64(name: &str) -> i64;
	fn get_volatile_f32(name: &str) -> f32;
	fn get_volatile_f64(name: &str) -> f64;
	fn get_volatile_str(name: &str) -> str;
	fn get_volatile_obj(name: &str) -> ObjId;

	//////////////////////////////////////////////////////////////////////////
	// 容器视窗相关操作(必须是玩家)
	/// @brief 玩家的视窗是否存在
	fn search_monitor(id: i32) -> bool;

	//////////////////////////////////////////////////////////////////////////
	// 缓存数据

	/// @brief 角色缓存数据(data_type 参照 E_ROLE_CACHE_DATA_TYPES )
	fn set_cache_data(data_type: i32, cache_data: &str) -> bool;
	fn get_cache_data(data_type: i32) -> str;

	//////////////////////////////////////////////////////////////////////////
	// 主从对象相关

	/// @brief 添加删除获取从对象
	fn add_slave_obj(slave_obj: &obj_id) -> bool;
	fn remove_slave_obj(slave_obj: &obj_id) -> bool;
	fn get_salve_objs(slave_objs: &i_array_list) -> bool;

	/// @brief 设置主对象
	fn get_master_obj() -> obj_id;
	fn set_master_obj(master_obj: &obj_id);

	//////////////////////////////////////////////////////////////////////////
	// 运维操作
	/// @brief 改变主角名字
	fn modify_name(name: &str) -> bool;
}
