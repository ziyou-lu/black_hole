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
use crate::share::any_list::IArrayList;

pub(crate) trait IRecord {
    /// @brief 获取表名
	fn get_table_name(&self) -> str;
	/// @brief 获得表列数
	fn get_column_number(&self) -> i32;
	/// @brief 获得表行数
	fn get_row_number(&self) -> i32;
	/// @brief 获得表最大行数
	fn get_max_row_number(&self) -> i32;
	/// @brief 获得列数据类型
	fn get_column_type(&self, col: i32) -> i32;
	/// @brief 添加一行，返回插入的行号
	fn add_row(&self, row: i32) -> i32;
	/// @brief 添加一行并初始化，返回插入的行号
	fn add_row_with_value<T: IArrayList>(&self, row: i32, value: &T) -> i32;
	/// @brief 删除一行
	fn delete_row(&self, row: i32) -> bool;
	/// @brief 清除所有表数据
	fn clear_all_row(&self) -> bool;

	/// @brief 写一行的表数据
	fn set_row_with_value<T: IArrayList>(&self, row: i32, value: &T) -> bool;
	/// @brief 读一行表数据
	fn get_row_value<T: IArrayList>(&self, row: i32, value: &mut T) -> bool;

}

trait IRecordOp<T> {
	fn set_value<T>(&self, row: i32, col: i32, value: T) -> bool;
	/// @brief 读表数据
	fn get_value<T>(&self, row: i32, col: i32) -> T;

	/// @brief 查找指定列相符的某行，返回-1表示没有
	fn search_value<T>(&self, col: i32, value: T) -> i32;
}

impl<T> IRecordOp<T> for dyn IRecord {
	fn set_value<T>(&self, row: i32, col: i32, value: T) -> bool {
		unimplemented!()
	}

	fn get_value<T>(&self, row: i32, col: i32) -> T {
		unimplemented!()
	}

	fn search_value<T>(&self, col: i32, value: T) -> i32 {
		unimplemented!()
	}
}