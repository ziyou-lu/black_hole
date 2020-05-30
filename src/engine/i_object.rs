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
uee i_record;

trait i_object {

    //////////////////////////////////////////////////////////////////////////
    // 对象配置
    
    // 获取对象类型
    pub fn fn get_obj_type() -> i32;

    // 获取对象号
    pub fn get_object_id() -> obj_id;

    // 获取对象脚本名
    pub fn get_script() -> str;

    // 获取对象配置名
    pub fn get_config() -> i64;

    // 获取名字
    pub fn get_name() -> str;

    // 获取组号
    pub fn get_group_id() -> i32;

    //////////////////////////////////////////////////////////////////////////
    // 对象层次结构
    
    // 获取对象在容器中的位置
    pub fn get_index_in_container() i32;

    // 获取父对象
    pub fn get_parent_obj() -> *i_base_object;

    // 返回容器容量
    pub fn get_container_capacity() -> i32;

    // 获取第一个子对象
    pub fn get_first_child_obj(it: &u32) -> obj_id;

    // 获取下一个子对象
    pub fn get_next_child_obj(it: &u32) -> obj_id;

    // 返回子对象数量
    pub fn get_child_obj_number() -> i32;

    // 获得指定位置子对象
    pub fn get_child_obj_by_index(index: i32) -> *i_base_object;

    // 根据名字获得子对象
    pub fn get_child_obj_by_name(name: &str) -> *i_base_object;

    // 获得子对象列表
    pub fn get_child_obj_id_list(class_type: i32, result: &i_array_list);

    // 查找子对象
    pub fn search_child_obj_by_name(name: &str, class_type: i32) -> obj_id;

    // 查找多个子对象
    pub fn search_more_child_obj_by_name(name: &str, class_type: i32, result: &i_array_list);

    // 使用配置名查找子对象
    pub fn search_child_obj_by_config(config: i64, class_type: i32) -> obj_id;

    // 使用配置名查找多个子对象
    pub fn search_more_child_obj_by_config(config: i64, class_type: i32, result: &i_array_list);

    // 清空子对象
    pub fn clear_child_obj();

    // 添加进弱关联容器
    pub fn add_to_container(container: &obj_id, pos: i32) -> Result;

    // 从弱关联容器中移除对象
    pub fn remove_from_container(container: &obj_id) -> Result;

    // 清空关联容器引用对象
    pub fn clear_container_child() -> Result;

    // 获得被弱关联的次数
    pub fn get_container_refs() -> i32;

    // 获取容器名字列表
    pub fn get_container_list(result: &i_array_list);

    // 对象坐标X
    pub fn get_pos_x() -> f32;

    // 对象坐标Y
    pub fn get_pos_y() -> f32;

    // 对象坐标Z
    pub fn get_pos_z() -> f32;

    // 对象方向
    pub fn get_orient() -> f32;

    ////////////////////////////////////////////////////////////////////////////
    /// 属性相关操作
    /// @brief 对象是否需要保存到db
	pub fn set_obj_unsave(value: bool);
	/// @brief 对象是否需要保存到db
	pub fn get_obj_unsave() -> bool;
	/// @brief 测试是否有一个属性
	// \param index_ptr 属性index，不存在会被置-1
	pub fn is_attr_exist(name: &str, index_ptr: *i32 = None) -> bool;
	pub fn is_attr_exist(name: &str, index_ptr: *i32 = None) -> bool;
	/// @brief 属性是否可视
	pub fn is_attr_visible(name: &str) -> bool;
	/// @brief 属性是否公共可视
	pub fn is_attr_pub fnlicvisible(name: &str) -> bool;
	/// @brief 可视属性是否即时刷新
	pub fn is_attr_realtime(name: &str) -> bool;
	/// @brief 属性是否保存
	pub fn is_attr_saving(name: &str) -> bool;
	/// @brief 可视属性是否隐藏
	pub fn is_attr_hide(name: &str) -> bool;
	/// @brief 设置可视属性是否隐藏
	pub fn set_attribute_hide(name: &str, value: bool) -> bool;
	/// @brief 增加(减少)属性值
	pub fn inc_i32(name: &str, value: i32) -> bool;
	pub fn inc_i32(name: &str, value: i32) -> bool;
	pub fn inc_i32(index: i32, value: i32) -> bool;
	/// @brief 增加(减少)属性值
	pub fn inc_f32(name: &str, value: f32) -> bool;
	pub fn inc_f32(name: &str, value: f32) -> bool;
	pub fn inc_f32( index: i32, value: f32) -> bool;
	/// @brief 用属性实现位标志操作（目的是节约存储，属性类型必须为整数）
	pub fn set_flag(name: &str, pos: i32) -> bool;
	/// @brief 清除标志
	pub fn clear_flag(name: &str, pos: i32) -> bool;
	/// @brief 测试标志
	pub fn test_flag(name: &str, pos: i32) -> bool;
	/// @brief 获得属性类型
	// \param index_ptr 属性index，不存在会被置-1
	pub fn get_attr_type(name: &str, index_ptr: *i32) -> i32;
	pub fn get_attr_type(name: &str, index_ptr: *i32) -> i32;
	pub fn get_attr_type(index: i32) -> i32;
	/// @brief 获得属性数量
	pub fn get_attr_count() -> i32;
	/// @brief 获得属性名称列表
	pub fn get_attr_name_list(result: &i_array_list) -> i32;
	/// @brief 设置属性值
	/// \param name 属性名
	/// \param value 属性值
	pub fn set_i32(name: &str, value: i32) -> bool;
	pub fn set_i64(name: &str, value: i64) -> bool;
	pub fn set_f32(name: &str, value: f32) -> bool;
	pub fn set_f64(name: &str, value: f64) -> bool;
	pub fn set_str(name: &str, value: &str) -> bool;
	pub fn set_obj(name: &str, value: &obj_id) -> bool;
	/// @brief 查询属性
	/// \param name 属性名
	// \param index_ptr 属性index，不存在会被置-1
	pub fn get_i32(name: &str, index_ptr: *i32) -> i32;
	pub fn get_i64(name: &str, index_ptr: *i32) -> i64;
	pub fn get_f32(name: &str, index_ptr: *i32) -> f32;
	pub fn get_f64(name: &str, index_ptr: *i32) -> f64;
	pub fn get_str(name: &str, index_ptr: *i32) -> str;
	pub fn get_obj(name: &str, index_ptr: *i32) -> obj_id;

	/// @brief 获得属性的索引值（返回-1表示未找到）
	/// \param name 属性名
	pub fn int get_attr_index(name: &str) -> i32;
	/// @brief 通过索引值设置属性值
	/// \param index 属性索引
	/// \param value 属性值
	pub fn set_i32(index: i32, value: i32) -> bool;
	pub fn set_i64(index: i32, value: i64) -> bool;
	pub fn set_f32(index: i32, value: f32) -> bool;
	pub fn set_f64(index: i32, value: f64) -> bool;
	pub fn set_str(index: i32, value: &str) -> bool;
	pub fn set_obj(index: i32, value: &obj_id) -> bool;
	/// @brief 通过索引值查询属性
	/// \param index 属性索引
	pub fn get_i32(index: i32) -> i32;
	pub fn get_i64(index: i32) -> i64;
	pub fn get_f32(index: i32) -> f32;
	pub fn get_f64(index: i32) -> f64;
	pub fn get_str(index: i32) -> str;
	pub fn get_obj(index: i32) -> obj_id;

	//////////////////////////////////////////////////////////////////////////
	// 属性回调相关
	/// @brief 查找关键属性回调是否存在
	pub fn attr_has_cb(property: &str, func: &str) -> bool;
	/// @brief 添加关键属性回调
	pub fn add_attr_cb(property: &str, func: &str, switch_carry: bool) -> bool;
	/// @brief 删除关键属性回调
	pub fn remove_attr_cb(property: &str) -> bool;
	/// @brief 删除指定函数名的关键属性回调
	pub fn remove_attr_cb(property: &str, func: &str) -> bool;

	//////////////////////////////////////////////////////////////////////////
	// 表格相关操作
	/// @brief 获得表数量
	pub fn get_record_count() -> i32;
	/// @brief 获得对象表格访问接口
	pub fn get_record_by_index(index: i32) -> *i_record;
	/// @brief 获得指定索引的表格访问接口
	pub fn get_record(name: &str) -> *i_record;
	/// @brief 表是否存在
	pub fn is_record_exist(name: &str) -> bool;
	/// @brief 获得表格的索引值（返回-1表示未找到）
	pub fn get_record_index(name: &str) -> i32;
	/// @brief 获得表名称列表
	pub fn get_record_name_list(result: &i_array_list) -> i32;
	/// @brief  获得表是否可视
	pub fn is_record_visible(name: &str) -> bool;
	/// @brief 获得表是否公共可视
	pub fn is_record_pub fnlic(name: &str) -> bool;
	/// @brief 获得表是否保存
	pub fn is_record_saving(name: &str) -> bool;
	/// @brief 设置标志
	pub fn set_record_flag(name: &str, pos: i32) -> bool;
	/// @brief 清除标志
	pub fn clear_record_flag(name: &str, pos: i32) -> bool;
	/// @brief 测试标志
	pub fn test_record_flag( name: &str, pos: i32) -> bool;

	//////////////////////////////////////////////////////////////////////////
	// 表格操作回调
	/// @brief  查找表钩子回调是否存在
	pub fn record_has_cb(record: &str, func: &str) -> bool;
	/// @brief 添加表钩子回调
	pub fn add_record_cb(record: &str, func: &str) -> bool;
	pub fn add_record_cb(record: &str, func: &str, switch_carry: bool) -> bool;
	/// @brief 删除表钩子回调
	pub fn remove_record_cb(record: &str) -> bool;
	/// @brief 删除指定函数名的表钩子回调
	pub fn remove_record_cb(record: &str, func: &str) -> bool;

	//////////////////////////////////////////////////////////////////////////
	// 临时数据相关操作
	/// @brief 查找临时数据
	pub fn volatile_exist(name: &str) -> bool;
	/// @brief 获得临时数据数量
	pub fn get_volatile_count() -> i32;
	/// @brief 获得临时数据名称列表
	pub fn get_volatile_name_list(IArrayList& result) -> i32;

	/// @brief 增加临时数据
	pub fn add_volatile(name: &str, int type) -> bool;
	pub fn add_volatile(name: &str, int type, switch_carry: bool) = 0;
	pub fn add_volatile_i32(name: &str, value: i32) -> bool;
	pub fn add_volatile_i32(name: &str, value: i32, switch_carry: bool) -> bool;
	pub fn add_volatile_i64(name: &str, value: i64) -> bool;
	pub fn add_volatile_i64(name: &str, value: i64, switch_carry: bool) -> bool;
	pub fn add_volatile_f32(name: &str, value: f32) -> bool;
	pub fn add_volatile_f32(name: &str, value: f32, switch_carry: bool) -> bool;
	pub fn add_volatile_f64(name: &str, value: f64) -> bool;
	pub fn add_volatile_f64(name: &str, value: f64, switch_carry: bool) -> bool;
	pub fn add_volatile_str(name: &str, value: &str) -> bool;
	pub fn add_volatile_str(name: &str, value: &str, switch_carry: bool) -> bool;
	pub fn add_volatile_obj(name: &str, value: &obj_id) -> bool;
	pub fn add_volatile_obj(name: &str, value: &obj_id, switch_carry: bool) -> bool;
	/// @brief 删除临时数据
	pub fn remove_volatile(name: &str) -> bool;
	/// @brief 获得临时数据类型
	pub fn get_volatile_type(name: &str) -> i32;

	/// @brief 设置临时数据
	pub fn bool set_volatile_i32(name: &str, value: i32) -> bool;
	pub fn bool set_volatile_i64(name: &str, value: i64) -> bool;
	pub fn bool set_volatile_f32(name: &str, value: f32) -> bool;
	pub fn bool set_volatile_f64(name: &str, value: f64) -> bool;
	pub fn bool set_volatile_str(name: &str, value: &str) -> bool;
	pub fn bool set_volatile_obj(name: &str, value: &obj_id) -> bool;

	/// @brief 查询临时数据
	pub fn get_volatile_i32(name: &str) -> i32;
	pub fn get_volatile_i64(name: &str) -> i64;
	pub fn get_volatile_f32(name: &str) -> f32;
	pub fn get_volatile_f64(name: &str) -> f64;
	pub fn get_volatile_str(name: &str) -> str;
	pub fn get_volatile_obj(name: &str) -> obj_id;

	//////////////////////////////////////////////////////////////////////////
	// 容器视窗相关操作(必须是玩家)
	/// @brief 玩家的视窗是否存在
	pub fn search_monitor(id: i32) -> bool;

	//////////////////////////////////////////////////////////////////////////
	// 缓存数据

	/// @brief 角色缓存数据(data_type 参照 E_ROLE_CACHE_DATA_TYPES )
	pub fn set_cache_data(data_type: i32, cache_data: &str) -> bool;
	pub fn get_cache_data(data_type: i32) -> str;

	//////////////////////////////////////////////////////////////////////////
	// 主从对象相关

	/// @brief 添加删除获取从对象
	pub fn add_slave_obj(slave_obj: &obj_id) -> bool;
	pub fn remove_slave_obj(slave_obj: &obj_id) -> bool;
	pub fn get_salve_objs(slave_objs: &i_array_list) -> bool;

	/// @brief 设置主对象
	pub fn get_master_obj() -> obj_id;
	pub fn set_master_obj(master_obj: &obj_id);

	//////////////////////////////////////////////////////////////////////////
	// 运维操作
	/// @brief 改变主角名字
	pub fn modify_name(name: &str) -> bool;
}
