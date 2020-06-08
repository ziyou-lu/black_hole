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



use crate::share::obj_id::ObjId;
use crate::share::any_list::IArrayList;
use crate::core::record::IRecord;

pub(crate) trait IObject {

    //////////////////////////////////////////////////////////////////////////
    // 对象配置
    
    // 获取对象类型
    fn get_obj_type(&self) -> i32;

    // 获取对象号
    fn get_object_id(&self) -> Option<ObjId>;

    // 获取对象脚本名
    fn get_script(&self) -> String;

    // 获取对象配置名
    fn get_config(&self) -> i64;

    // 获取名字
    fn get_name(&self) -> String;

    // 获取组号
    fn get_group_id(&self) -> i32;

    //////////////////////////////////////////////////////////////////////////
    // 对象层次结构
    
    // 获取对象在容器中的位置
    fn get_index_in_container(&self) -> i32;

    // 获取父对象
    fn get_parent_obj<T: IObject>(&self) -> Option<T>;

    // 返回容器容量
    fn get_container_capacity(&self) -> i32;

    // 获取第一个子对象
    fn get_first_child_obj(&self, it: &u32) -> Option<ObjId>;

    // 获取下一个子对象
    fn get_next_child_obj(&self, it: &u32) -> Option<ObjId>;

    // 返回子对象数量
    fn get_child_obj_number(&self) -> i32;

    // 获得指定位置子对象
    fn get_child_obj_by_index<T: IObject>(&self, index: i32) -> Option<T>;

    // 根据名字获得子对象
    fn get_child_obj_by_name<T: IObject>(&self, name: &str) -> Option<T>;

    // 获得子对象列表
    fn get_child_obj_id_list<T: IArrayList>(&self, class_type: i32, result: &mut T);

    // 查找子对象
    fn search_child_obj_by_name(&self, name: &str, class_type: i32) -> Option<ObjId>;

    // 查找多个子对象
    fn search_more_child_obj_by_name<T: IArrayList>(&self, name: &str, class_type: i32, result: &mut T);

    // 使用配置名查找子对象
    fn search_child_obj_by_config(&self, config: i64, class_type: i32) -> Option<ObjId>;

    // 使用配置名查找多个子对象
    fn search_more_child_obj_by_config<T: IArrayList>(&self, config: i64, class_type: i32, result: &mut T);

    // 清空子对象
    fn clear_child_obj(&self);

    // 添加进弱关联容器
    fn add_to_container(&self, container: &ObjId, pos: i32) -> bool;

    // 从弱关联容器中移除对象
    fn remove_from_container(&self, container: &ObjId) -> bool;

    // 清空关联容器引用对象
    fn clear_container_child(&self) -> bool;

    // 获得被弱关联的次数
    fn get_container_refs(&self) -> i32;

    // 获取容器名字列表
    fn get_container_list<T: IArrayList>(&self, result: &mut T);

    // 对象坐标X
    fn get_pos_x(&self) -> f32;

    // 对象坐标Y
    fn get_pos_y(&self) -> f32;

    // 对象坐标Z
    fn get_pos_z(&self) -> f32;

    // 对象方向
    fn get_orient(&self) -> f32;




	//////////////////////////////////////////////////////////////////////////
	// 表格相关操作
	/// @brief 获得表数量
	fn get_record_count(&self) -> i32;
	/// @brief 获得对象表格访问接口
	fn get_record_by_index<T: IRecord>(&self, index: i32) -> Option<T>;
	/// @brief 获得指定索引的表格访问接口
	fn get_record<T: IRecord>(&self, name: &str) -> Option<T>;
	/// @brief 表是否存在
	fn is_record_exist(&self, name: &str) -> bool;
	/// @brief 获得表格的索引值（返回-1表示未找到）
	fn get_record_index(&self, name: &str) -> i32;
	/// @brief 获得表名称列表
	fn get_record_name_list<T: IArrayList>(&self, result: &mut T) -> i32;
	/// @brief  获得表是否可视
	fn is_record_visible(&self, name: &str) -> bool;
	/// @brief 获得表是否公共可视
	fn is_record_public(&self, name: &str) -> bool;
	/// @brief 获得表是否保存
	fn is_record_saving(&self, name: &str) -> bool;
	/// @brief 设置标志
	fn set_record_flag(&self, name: &str, pos: i32) -> bool;
	/// @brief 清除标志
	fn clear_record_flag(&self, name: &str, pos: i32) -> bool;
	/// @brief 测试标志
	fn test_record_flag(&self, name: &str, pos: i32) -> bool;

	//////////////////////////////////////////////////////////////////////////
	// 表格操作回调
	/// @brief  查找表钩子回调是否存在
	fn record_has_cb(&self, record: &str, func: &str) -> bool;
	/// @brief 添加表钩子回调
	fn add_record_cb(&self, record: &str, func: &str, switch_carry: bool) -> bool;
	/// @brief 删除表钩子回调
	fn remove_record_cb(&self, record: &str) -> bool;
	/// @brief 删除指定函数名的表钩子回调
	fn remove_record_cb_func(&self, record: &str, func: &str) -> bool;

	//////////////////////////////////////////////////////////////////////////
	// 容器视窗相关操作(必须是玩家)
	/// @brief 玩家的视窗是否存在
	fn search_monitor(&self, id: i32) -> bool;

	//////////////////////////////////////////////////////////////////////////
	// 缓存数据

	/// @brief 角色缓存数据(data_type 参照 E_ROLE_CACHE_DATA_TYPES )
	fn set_cache_data(&self, data_type: i32, cache_data: &str) -> bool;
	fn get_cache_data(&self, data_type: i32) -> String;

	//////////////////////////////////////////////////////////////////////////
	// 主从对象相关

	/// @brief 添加删除获取从对象
	fn add_slave_obj(&self, slave_obj: &ObjId) -> bool;
	fn remove_slave_obj(&self, slave_obj: &ObjId) -> bool;
	fn get_salve_objs<T: IArrayList>(&self, slave_objs: &mut T) -> bool;

	/// @brief 设置主对象
	fn get_master_obj(&self) -> Option<ObjId>;
	fn set_master_obj(&self, master_obj: &ObjId);

	//////////////////////////////////////////////////////////////////////////
	// 运维操作
	/// @brief 改变主角名字
	fn modify_name(&self, name: &str) -> bool;

	////////////////////////////////////////////////////////////////////////////
	/// 属性相关操作
    /// @brief 对象是否需要保存到db
	fn set_obj_un_save(&self, value: bool);
	/// @brief 对象是否需要保存到db
	fn get_obj_un_save(&self) -> bool;
	/// @brief 测试是否有一个属性
	// \param index_ptr 属性index，不存在会被置-1
	fn is_attr_exist(&self, name: &str, index_ptr: *i32) -> bool;
	/// @brief 属性是否可视
	fn is_attr_visible(&self, name: &str) -> bool;
	/// @brief 属性是否公共可视
	fn is_attr_pub_visible(&self, name: &str) -> bool;
	/// @brief 可视属性是否即时刷新
	fn is_attr_realtime(&self, name: &str) -> bool;
	/// @brief 属性是否保存
	fn is_attr_saving(&self, name: &str) -> bool;
	/// @brief 可视属性是否隐藏
	fn is_attr_hide(&self, name: &str) -> bool;
	/// @brief 设置可视属性是否隐藏
	fn set_attribute_hide(&self, name: &str, value: bool) -> bool;
	/// @brief 用属性实现位标志操作（目的是节约存储，属性类型必须为整数）
	fn set_flag(&self, name: &str, pos: i32) -> bool;
	/// @brief 清除标志
	fn clear_flag(&self, name: &str, pos: i32) -> bool;
	/// @brief 测试标志
	fn test_flag(&self, name: &str, pos: i32) -> bool;
	/// @brief 获得属性类型
	// \param index_ptr 属性index，不存在会被置-1
	fn get_attr_type(&self, name: &str, index_ptr: *i32) -> i32;
	fn get_attr_type_index(&self, index: i32) -> i32;
	/// @brief 获得属性数量
	fn get_attr_count(&self) -> i32;
	/// @brief 获得属性名称列表
	fn get_attr_name_list<T: IArrayList>(&self, result: &mut T) -> i32;
	//////////////////////////////////////////////////////////////////////////
	// 属性回调相关
	/// @brief 查找关键属性回调是否存在
	fn attr_has_cb(&self, attr: &str, func: &str) -> bool;
	/// @brief 添加关键属性回调
	fn add_attr_cb(&self, attr: &str, func: &str, switch_carry: bool) -> bool;
	/// @brief 删除关键属性回调
	fn remove_attr_cb(&self, attr: &str) -> bool;
	/// @brief 删除指定函数名的关键属性回调
	fn remove_attr_cb_func(&self, attr: &str, func: &str) -> bool;
	/// @brief 获得属性的索引值（返回-1表示未找到）
	/// \param name 属性名
	fn get_attr_index(&self, name: &str) -> i32;

	//////////////////////////////////////////////////////////////////////////
	// 临时数据相关操作
	/// @brief 查找临时数据
	fn volatile_exist(&self, name: &str) -> bool;
	/// @brief 获得临时数据数量
	fn get_volatile_count(&self) -> i32;
	/// @brief 获得临时数据名称列表
	fn get_volatile_name_list<T: IArrayList>(&self, result: &mut T) -> i32;

	/// @brief 删除临时数据
	fn remove_volatile(&self, name: &str) -> bool;
	/// @brief 获得临时数据类型
	fn get_volatile_type(&self, name: &str) -> i32;

}

trait IObjectAttr {
	/// @brief 增加(减少)属性值
	fn inc_attr<T>(&self, name: &str, value: T) -> bool;
	fn inc_attr_index<T>(&self, index: i32, value: T) -> bool;
	/// @brief 设置属性值
	/// \param name 属性名
	/// \param value 属性值
	fn set_attr<T>(&self, name: &str, value: T) -> bool;
	/// @brief 查询属性
	/// \param name 属性名
	// \param index_ptr 属性index，不存在会被置-1
	fn get_attr<T>(&self, name: &str) -> Option<T>;
	/// @brief 通过索引值设置属性值
	/// \param index 属性索引
	/// \param value 属性值
	fn set_attr_idx<T>(&self, index: i32, value: T) -> bool;

	/// @brief 通过索引值查询属性
	/// \param index 属性索引
	fn get_attr_idx<T>(&self, index: i32) -> Option<T>;
}

trait IObjectVolatile<T> {

	/// @brief 增加临时数据
	fn add_volatile<T>(&self, name: &str, value: T, switch_carry: bool) -> bool;

	/// @brief 设置临时数据
	fn set_volatile<T>(&self, name: &str, value: T) -> bool;

	/// @brief 查询临时数据
	fn get_volatile<T>(&self, name: &str) -> Option<T>;
}

struct Object {

}

impl IObject for Object {
	fn get_obj_type(&self) -> i32 {
		unimplemented!()
	}

	fn get_object_id(&self) -> Option<ObjId> {
		unimplemented!()
	}

	fn get_script(&self) -> String {
		unimplemented!()
	}

	fn get_config(&self) -> i64 {
		unimplemented!()
	}

	fn get_name(&self) -> String {
		unimplemented!()
	}

	fn get_group_id(&self) -> i32 {
		unimplemented!()
	}

	fn get_index_in_container(&self) -> i32 {
		unimplemented!()
	}

	fn get_parent_obj<T: IObject>(&self) -> Option<T> {
		unimplemented!()
	}

	fn get_container_capacity(&self) -> i32 {
		unimplemented!()
	}

	fn get_first_child_obj(&self, it: &u32) -> Option<ObjId> {
		unimplemented!()
	}

	fn get_next_child_obj(&self, it: &u32) -> Option<ObjId> {
		unimplemented!()
	}

	fn get_child_obj_number(&self) -> i32 {
		unimplemented!()
	}

	fn get_child_obj_by_index<T: IObject>(&self, index: i32) -> Option<T> {
		unimplemented!()
	}

	fn get_child_obj_by_name<T: IObject>(&self, name: &str) -> Option<T> {
		unimplemented!()
	}

	fn get_child_obj_id_list<T: IArrayList>(&self, class_type: i32, result: &mut T) {
		unimplemented!()
	}

	fn search_child_obj_by_name(&self, name: &str, class_type: i32) -> Option<ObjId> {
		unimplemented!()
	}

	fn search_more_child_obj_by_name<T: IArrayList>(&self, name: &str, class_type: i32, result: &mut T) {
		unimplemented!()
	}

	fn search_child_obj_by_config(&self, config: i64, class_type: i32) -> Option<ObjId> {
		unimplemented!()
	}

	fn search_more_child_obj_by_config<T: IArrayList>(&self, config: i64, class_type: i32, result: &mut T) {
		unimplemented!()
	}

	fn clear_child_obj(&self) {
		unimplemented!()
	}

	fn add_to_container(&self, container: &ObjId, pos: i32) -> bool {
		unimplemented!()
	}

	fn remove_from_container(&self, container: &ObjId) -> bool {
		unimplemented!()
	}

	fn clear_container_child(&self) -> bool {
		unimplemented!()
	}

	fn get_container_refs(&self) -> i32 {
		unimplemented!()
	}

	fn get_container_list<T: IArrayList>(&self, result: &mut T) {
		unimplemented!()
	}

	fn get_pos_x(&self) -> f32 {
		unimplemented!()
	}

	fn get_pos_y(&self) -> f32 {
		unimplemented!()
	}

	fn get_pos_z(&self) -> f32 {
		unimplemented!()
	}

	fn get_orient(&self) -> f32 {
		unimplemented!()
	}

	fn get_record_count(&self) -> i32 {
		unimplemented!()
	}

	fn get_record_by_index<T: IRecord>(&self, index: i32) -> Option<T> {
		unimplemented!()
	}

	fn get_record<T: IRecord>(&self, name: &str) -> Option<T> {
		unimplemented!()
	}

	fn is_record_exist(&self, name: &str) -> bool {
		unimplemented!()
	}

	fn get_record_index(&self, name: &str) -> i32 {
		unimplemented!()
	}

	fn get_record_name_list<T: IArrayList>(&self, result: &mut T) -> i32 {
		unimplemented!()
	}

	fn is_record_visible(&self, name: &str) -> bool {
		unimplemented!()
	}

	fn is_record_public(&self, name: &str) -> bool {
		unimplemented!()
	}

	fn is_record_saving(&self, name: &str) -> bool {
		unimplemented!()
	}

	fn set_record_flag(&self, name: &str, pos: i32) -> bool {
		unimplemented!()
	}

	fn clear_record_flag(&self, name: &str, pos: i32) -> bool {
		unimplemented!()
	}

	fn test_record_flag(&self, name: &str, pos: i32) -> bool {
		unimplemented!()
	}

	fn record_has_cb(&self, record: &str, func: &str) -> bool {
		unimplemented!()
	}

	fn add_record_cb(&self, record: &str, func: &str, switch_carry: bool) -> bool {
		unimplemented!()
	}

	fn remove_record_cb(&self, record: &str) -> bool {
		unimplemented!()
	}

	fn remove_record_cb_func(&self, record: &str, func: &str) -> bool {
		unimplemented!()
	}

	fn search_monitor(&self, id: i32) -> bool {
		unimplemented!()
	}

	fn set_cache_data(&self, data_type: i32, cache_data: &str) -> bool {
		unimplemented!()
	}

	fn get_cache_data(&self, data_type: i32) -> String {
		unimplemented!()
	}

	fn add_slave_obj(&self, slave_obj: &ObjId) -> bool {
		unimplemented!()
	}

	fn remove_slave_obj(&self, slave_obj: &ObjId) -> bool {
		unimplemented!()
	}

	fn get_salve_objs<T: IArrayList>(&self, slave_objs: &mut T) -> bool {
		unimplemented!()
	}

	fn get_master_obj(&self) -> Option<ObjId> {
		unimplemented!()
	}

	fn set_master_obj(&self, master_obj: &ObjId) {
		unimplemented!()
	}

	fn modify_name(&self, name: &str) -> bool {
		unimplemented!()
	}

	fn set_obj_un_save(&self, value: bool) {
		unimplemented!()
	}

	fn get_obj_un_save(&self) -> bool {
		unimplemented!()
	}

	fn is_attr_exist(&self, name: &str, index_ptr: *const i32) -> bool {
		unimplemented!()
	}

	fn is_attr_visible(&self, name: &str) -> bool {
		unimplemented!()
	}

	fn is_attr_pub_visible(&self, name: &str) -> bool {
		unimplemented!()
	}

	fn is_attr_realtime(&self, name: &str) -> bool {
		unimplemented!()
	}

	fn is_attr_saving(&self, name: &str) -> bool {
		unimplemented!()
	}

	fn is_attr_hide(&self, name: &str) -> bool {
		unimplemented!()
	}

	fn set_attribute_hide(&self, name: &str, value: bool) -> bool {
		unimplemented!()
	}

	fn set_flag(&self, name: &str, pos: i32) -> bool {
		unimplemented!()
	}

	fn clear_flag(&self, name: &str, pos: i32) -> bool {
		unimplemented!()
	}

	fn test_flag(&self, name: &str, pos: i32) -> bool {
		unimplemented!()
	}

	fn get_attr_type(&self, name: &str, index_ptr: *const i32) -> i32 {
		unimplemented!()
	}

	fn get_attr_type_index(&self, index: i32) -> i32 {
		unimplemented!()
	}

	fn get_attr_count(&self) -> i32 {
		unimplemented!()
	}

	fn get_attr_name_list<T: IArrayList>(&self, result: &mut T) -> i32 {
		unimplemented!()
	}

	fn attr_has_cb(&self, attr: &str, func: &str) -> bool {
		unimplemented!()
	}

	fn add_attr_cb(&self, attr: &str, func: &str, switch_carry: bool) -> bool {
		unimplemented!()
	}

	fn remove_attr_cb(&self, attr: &str) -> bool {
		unimplemented!()
	}

	fn remove_attr_cb_func(&self, attr: &str, func: &str) -> bool {
		unimplemented!()
	}

	fn get_attr_index(&self, name: &str) -> i32 {
		unimplemented!()
	}

	fn volatile_exist(&self, name: &str) -> bool {
		unimplemented!()
	}

	fn get_volatile_count(&self) -> i32 {
		unimplemented!()
	}

	fn get_volatile_name_list<T: IArrayList>(&self, result: &mut T) -> i32 {
		unimplemented!()
	}

	fn remove_volatile(&self, name: &str) -> bool {
		unimplemented!()
	}

	fn get_volatile_type(&self, name: &str) -> i32 {
		unimplemented!()
	}
}

impl<T> IObjectAttr<T> for dyn IObject {
	fn inc_attr<T>(&self, name: &str, value: T) -> bool {
		unimplemented!()
	}

	fn inc_attr_index<T>(&self, index: i32, value: T) -> bool {
		unimplemented!()
	}

	fn set_attr<T>(&self, name: &str, value: T) -> bool {
		unimplemented!()
	}

	fn get_attr<T>(&self, name: &str) -> Option<T> {
		unimplemented!()
	}

	fn set_attr_idx<T>(&self, index: i32, value: T) -> bool {
		unimplemented!()
	}

	fn get_attr_idx<T>(&self, index: i32) -> Option<T> {
		unimplemented!()
	}
}

impl<T> IObjectVolatile<T> for dyn IObject {
	fn add_volatile<T>(&self, name: &str, value: T, switch_carry: bool) -> bool {
		unimplemented!()
	}

	fn set_volatile<T>(&self, name: &str, value: T) -> bool {
		unimplemented!()
	}

	fn get_volatile<T>(&self, name: &str) -> Option<T> {
		unimplemented!()
	}
}
