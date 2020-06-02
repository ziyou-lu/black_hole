/*************************************************
//  Copyright (C), 2020-2020, luwangda.
//  File name:       i_record.h
//  Author:        	 luwangda
//  Version:         1.0
//  Date:            2020/05/18
//  Description:     表格接口
//  Others:
//  History
*************************************************/
use share;

trait IRecord {
    /// @brief 获取表名
	fn get_table_name() -> str;
	/// @brief 获得表列数
	fn get_column_number() -> i32;
	/// @brief 获得表行数
	fn get_row_number() -> i32;
	/// @brief 获得表最大行数
	fn get_max_row_number() -> i32;
	/// @brief 获得列数据类型
	fn get_column_type(col: i32) -> i32;
	/// @brief 添加一行，返回插入的行号
	fn add_row(row: i32) -> i32;
	/// @brief 添加一行并初始化，返回插入的行号
	fn add_row_with_value(row: i32, value: &i_array_list) -> i32;
	/// @brief 删除一行
	fn delete_row(row: i32) -> bool;
	/// @brief 清除所有表数据
	fn clear_all_row() -> bool;

	/// @brief 写一行的表数据
	fn set_row_with_value(row: i32, value: &IArrayList) -> bool;
	fn set_value<T>(row: i32, col: i32, value: T) -> bool;
	/// @brief 写表数据
	/*fn set_i32(row: i32, col: i32, value: i32) -> bool;
	fn set_i64(row: i32, col: i32, value: i64) -> bool;
	fn set_f32(row: i32, col: i32, value: f32) -> bool;
	fn set_f64(row: i32, col: i32, value: f64) -> bool;
	fn set_str(row: i32, col: i32, value: &str) -> bool;
	fn set_obj(row: i32, col: i32, value: &obj_id) -> bool;*/

	/// @brief 读一行表数据
	fn get_row_value(row: i32, value: &IArrayList) -> bool;
	/// @brief 读表数据
	fn get_value<T>(row: i32, col: i32) -> T;
	/*fn get_i32(row: i32, col: i32) -> i32;
	fn get_i64(row: i32, col: i32) -> i64;
	fn get_f32(row: i32, col: i32) -> f32;
	fn get_f64(row: i32, col: i32) -> f64;
	fn get_str(row: i32, col: i32) -> str;
	fn get_obj(row: i32, col: i32) -> ObjId;*/

	/// @brief 查找指定列相符的某行，返回-1表示没有
	fn search_value(col: i32, value: T) -> i32;
	/*fn search_i32(col: i32, value: i32) -> i32;
	fn search_i64(col: i32, value: i64) -> i32;
	fn search_int64(col: i32, value: i64, start_row: i32) -> i32;
	fn search_float(col: i32, value: f32) -> i32;
	fn search_double(col: i32, value: f64) -> i32;
	fn search_string(col: i32, value: &str) -> i32;
	fn search_object(col: i32, value: &ObjId) -> i32;*/
	/// @brief 不区分大小写查找
	//fn search_stringci(col: i32, value: &str) -> i32;
}
